use femtovg::{Paint, Path};
use rustc_hash::FxHashMap;

use crate::{
    state::{FluidId, ItemId},
    ui::Canvas,
    Cardinal, InOutput, ItemOrFluidId, Node, NodeId,
};

pub fn demo_nodes() -> FxHashMap<NodeId, Node> {
    let mut nodes = FxHashMap::default();
    nodes.insert(
        NodeId(0),
        Node {
            id: NodeId(0),
            position: Point2::new(100.0, 100.0),
            inputs: vec![
                InOutput {
                    item_or_fluid: ItemOrFluidId::Item(ItemId(0)),
                    rate: 1.0,
                },
                InOutput {
                    item_or_fluid: ItemOrFluidId::Fluid(FluidId(0)),
                    rate: 1.0,
                },
            ],
            outputs: vec![
                InOutput {
                    item_or_fluid: ItemOrFluidId::Item(ItemId(1)),
                    rate: 1.0,
                },
                InOutput {
                    item_or_fluid: ItemOrFluidId::Fluid(FluidId(1)),
                    rate: 1.0,
                },
            ],

            direction: Cardinal::East,
        },
    );
    nodes.insert(
        NodeId(1),
        Node {
            id: NodeId(1),
            position: Point2::new(200.0, 200.0),

            inputs: vec![
                InOutput {
                    item_or_fluid: ItemOrFluidId::Item(ItemId(1)),
                    rate: 1.0,
                },
                InOutput {
                    item_or_fluid: ItemOrFluidId::Fluid(FluidId(1)),
                    rate: 1.0,
                },
            ],
            outputs: vec![
                InOutput {
                    item_or_fluid: ItemOrFluidId::Item(ItemId(2)),
                    rate: 1.0,
                },
                InOutput {
                    item_or_fluid: ItemOrFluidId::Fluid(FluidId(2)),
                    rate: 1.0,
                },
            ],

            direction: Cardinal::South,
        },
    );
    nodes.insert(
        NodeId(2),
        Node {
            id: NodeId(2),
            position: Point2::new(300.0, 300.0),

            inputs: vec![
                InOutput {
                    item_or_fluid: ItemOrFluidId::Item(ItemId(2)),
                    rate: 1.0,
                },
                InOutput {
                    item_or_fluid: ItemOrFluidId::Fluid(FluidId(2)),
                    rate: 1.0,
                },
            ],
            outputs: vec![
                InOutput {
                    item_or_fluid: ItemOrFluidId::Item(ItemId(3)),
                    rate: 1.0,
                },
                InOutput {
                    item_or_fluid: ItemOrFluidId::Fluid(FluidId(3)),
                    rate: 1.0,
                },
            ],

            direction: Cardinal::West,
        },
    );
    nodes.insert(
        NodeId(3),
        Node {
            id: NodeId(3),
            position: Point2::new(400.0, 400.0),

            inputs: vec![
                InOutput {
                    item_or_fluid: ItemOrFluidId::Item(ItemId(3)),
                    rate: 1.0,
                },
                InOutput {
                    item_or_fluid: ItemOrFluidId::Fluid(FluidId(3)),
                    rate: 1.0,
                },
            ],
            outputs: vec![
                InOutput {
                    item_or_fluid: ItemOrFluidId::Item(ItemId(0)),
                    rate: 1.0,
                },
                InOutput {
                    item_or_fluid: ItemOrFluidId::Fluid(FluidId(0)),
                    rate: 1.0,
                },
            ],

            direction: Cardinal::North,
        },
    );
    nodes
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const ZERO: Vec2 = Vec2 { x: 0.0, y: 0.0 };

    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn dot(&self, other: Vec2) -> f32 {
        self.x * other.x + self.y * other.y
    }

    pub fn length(&self) -> f32 {
        self.dot(*self).sqrt()
    }

    pub fn with_size(self, size: Vec2) -> Rectangle {
        Rectangle {
            x: self.x,
            y: self.y,
            width: size.x,
            height: size.y,
        }
    }
}

impl From<(f32, f32)> for Vec2 {
    fn from((x, y): (f32, f32)) -> Self {
        Vec2::new(x, y)
    }
}

impl std::ops::Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, other: Vec2) -> Vec2 {
        Vec2::new(self.x + other.x, self.y + other.y)
    }
}

impl std::ops::Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, scalar: f32) -> Vec2 {
        Vec2::new(self.x * scalar, self.y * scalar)
    }
}

impl std::ops::Neg for Vec2 {
    type Output = Vec2;

    fn neg(self) -> Vec2 {
        Vec2::new(-self.x, -self.y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Point2 {
    pub x: f32,
    pub y: f32,
}

impl Point2 {
    pub const ZERO: Point2 = Point2 { x: 0.0, y: 0.0 };

    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    // pub fn add(&self, other: Vec2) -> Point2 {
    //     Point2::new(self.x + other.x, self.y + other.y)
    // }

    // pub fn sub(&self, other: Vec2) -> Point2 {
    //     Point2::new(self.x - other.x, self.y - other.y)
    // }

    // pub fn to_vec2(&self) -> Vec2 {
    //     Vec2::new(self.x, self.y)
    // }

    pub fn relative_to(&self, other: Point2) -> Vec2 {
        let xdiff = self.x - other.x;
        let ydiff = self.y - other.y;

        if xdiff.abs() > ydiff.abs() {
            Vec2::new(xdiff.signum(), 0.)
        } else {
            Vec2::new(0., ydiff.signum())
        }
    }

    pub(crate) fn distance(&self, to: Point2) -> f32 {
        let dx = self.x - to.x;
        let dy = self.y - to.y;
        (dx * dx + dy * dy).sqrt()
    }

    pub(crate) fn with_size(&self, size: Vec2) -> Rectangle {
        Rectangle {
            x: self.x,
            y: self.y,
            width: size.x,
            height: size.y,
        }
    }
}

impl From<(f32, f32)> for Point2 {
    fn from((x, y): (f32, f32)) -> Self {
        Point2::new(x, y)
    }
}

impl std::ops::Add<Point2> for Point2 {
    type Output = Point2;

    fn add(self, other: Point2) -> Point2 {
        Point2::new(self.x + other.x, self.y + other.y)
    }
}

impl std::ops::Add<Vec2> for Point2 {
    type Output = Point2;

    fn add(self, other: Vec2) -> Point2 {
        Point2::new(self.x + other.x, self.y + other.y)
    }
}

impl std::ops::Add<f32> for Point2 {
    type Output = Point2;

    fn add(self, scalar: f32) -> Point2 {
        Point2::new(self.x + scalar, self.y + scalar)
    }
}

impl std::ops::AddAssign<Vec2> for Point2 {
    fn add_assign(&mut self, other: Vec2) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl std::ops::Sub<Vec2> for Point2 {
    type Output = Point2;

    fn sub(self, other: Vec2) -> Point2 {
        Point2::new(self.x - other.x, self.y - other.y)
    }
}

impl std::ops::Sub<Point2> for Point2 {
    type Output = Vec2;

    fn sub(self, other: Point2) -> Vec2 {
        Vec2::new(self.x - other.x, self.y - other.y)
    }
}

impl From<Vec2> for Point2 {
    fn from(vec: Vec2) -> Self {
        Point2::new(vec.x, vec.y)
    }
}

impl From<Point2> for Vec2 {
    fn from(point: Point2) -> Self {
        Vec2::new(point.x, point.y)
    }
}

pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rectangle {
    pub fn centered_square(center: Point2, size: f32) -> Self {
        Self {
            x: center.x - size / 2.0,
            y: center.y - size / 2.0,
            width: size,
            height: size,
        }
    }

    pub fn contains(&self, point: impl Into<Point2>) -> bool {
        let Point2 { x, y } = point.into();
        x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height
    }

    pub fn draw_rounded(
        &self,
        canvas: &mut Canvas,
        bg_paint: &Paint,
        border_paint: &Paint,
        radius: f32,
    ) {
        let mut path = Path::new();
        path.rounded_rect(self.x, self.y, self.width, self.height, radius);
        canvas.fill_path(&path, bg_paint);
        canvas.stroke_path(&path, border_paint);
    }

    pub fn center(&self) -> Point2 {
        Point2::new(self.x + self.width / 2.0, self.y + self.height / 2.0)
    }

    pub(crate) fn draw_fill(&self, canvas: &mut Canvas, paint: &Paint) {
        let mut path = Path::new();
        path.move_to(self.x, self.y);
        path.line_to(self.x + self.width, self.y);
        path.line_to(self.x + self.width, self.y + self.height);
        path.line_to(self.x, self.y + self.height);
        canvas.fill_path(&path, paint);
    }
}
