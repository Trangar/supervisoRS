use femtovg::Paint;
use rustc_hash::FxHashMap;

use crate::{
    utils::{Point2, Vec2},
    Node, NodeId, SocketPos,
};

use super::{
    utils::{draw_bezier, get_node_socket_position},
    Canvas, EventCtx,
};

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Drag {
    state: DragState,
    start_drag: Option<Point2>,
    overcame_min_distance: bool,
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub enum DragState {
    #[default]
    None,
    Background,
    Node {
        node_id: NodeId,
    },
    LineFromNodeSocket {
        pos: SocketPos,
        initial_direction: Vec2,
    },
}

impl Drag {
    pub fn state(&self) -> &DragState {
        &self.state
    }

    pub fn draw_line(
        &self,
        nodes: &FxHashMap<NodeId, Node>,
        canvas: &mut Canvas,
        paint: &Paint,
        to: Point2,
    ) {
        if let DragState::LineFromNodeSocket {
            pos,
            initial_direction,
            ..
        } = &self.state
        {
            draw_bezier(
                canvas,
                paint,
                get_node_socket_position(
                    nodes.get(&pos.node_id).unwrap(),
                    pos.socket_index,
                    pos.input,
                )
                .center(),
                *initial_direction,
                to,
                None,
            );
        }
    }

    pub fn should_highlight_node(&self, node: &Node) -> bool {
        match &self.state {
            DragState::Node { node_id, .. } => node_id == &node.id,
            DragState::LineFromNodeSocket { pos, .. } => pos.node_id == node.id,
            _ => false,
        }
    }

    pub fn get_highlight_socket(&self, node: &Node) -> Option<(usize, bool)> {
        match &self.state {
            DragState::LineFromNodeSocket { pos, .. } if pos.node_id == node.id => {
                Some((pos.socket_index, pos.input))
            }
            _ => None,
        }
    }

    pub(crate) fn start_drag_node(&mut self, mouse: Point2, node_id: NodeId) {
        self.state = DragState::Node { node_id };
        self.start_drag = Some(mouse);
        self.overcame_min_distance = false;
    }

    pub fn start_drag_line(&mut self, start_drag: Point2, pos: SocketPos, initial_direction: Vec2) {
        self.state = DragState::LineFromNodeSocket {
            pos,
            initial_direction,
        };
        self.start_drag = Some(start_drag);
        self.overcame_min_distance = false;
    }

    pub fn start_drag_background(&mut self, start_drag: Point2) {
        self.state = DragState::Background;
        self.start_drag = Some(start_drag);
        self.overcame_min_distance = false;
    }

    pub(crate) fn clear(&mut self) {
        self.state = DragState::None;
        self.start_drag = None;
        self.overcame_min_distance = false;
    }

    pub(crate) fn mouse_move(
        &mut self,
        delta: Vec2,
        ctx: &mut EventCtx,
        nodes: &mut FxHashMap<NodeId, Node>,
    ) {
        if let Some(start_point) = self.start_drag {
            if (ctx.mouse - start_point).length() > 10. {
                self.overcame_min_distance = true;
            }
        }
        match &self.state {
            DragState::Background { .. } => {
                ctx.translate_by(delta);
                ctx.redraw();
                return;
            }
            DragState::Node { node_id, .. } => {
                let node = nodes.get_mut(&node_id).unwrap();
                node.position += delta;
                ctx.redraw();
                return;
            }
            DragState::LineFromNodeSocket { .. } => ctx.redraw(),
            _ => {}
        }
    }

    pub fn mouse_up_was_click(&self) -> bool {
        matches!(
            self.state,
            DragState::Background { .. }
                | DragState::LineFromNodeSocket { .. }
                | DragState::Node { .. }
        ) && !self.overcame_min_distance
    }
}
