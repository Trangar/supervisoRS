use crate::{Node, utils::Rectangle};

pub fn get_node_socket_position(node: &Node, socket: usize, is_input: bool) -> Rectangle {
    let (offset, step) = if is_input {
        node.direction.input_offset()
    } else {
        node.direction.output_offset()
    };
    let count = if is_input {
        node.inputs.len()
    } else {
        node.outputs.len()
    };
    let half_count = if count == 0 {
        0.0
    } else {
        (count - 1) as f32 / 2.0
    };

    let offset = node.position + offset - (step * half_count);

    let pos = offset + step * socket as f32;

    Rectangle::centered_square(pos, 20.)
}

// pub fn draw_bezier(
//     canvas: &mut Canvas,
//     paint: &Paint,
//     from: Point2,
//     from_direction: Vec2,
//     to: Point2,
//     to_direction: Option<Vec2>,
// ) {
//     let mut path = Path::new();
//     path.move_to(from.x, from.y);
//     let end_direction = match to_direction {
//         Some(dir) => -dir,
//         None => to.relative_to(from) * (from.distance(to) * BEZIER_CURVE_FACTOR),
//     };
//     path.bezier_to(
//         from.x + from_direction.x,
//         from.y + from_direction.y,
//         to.x - end_direction.x,
//         to.y - end_direction.y,
//         to.x,
//         to.y,
//     );

//     canvas.stroke_path(&path, paint);
// }
