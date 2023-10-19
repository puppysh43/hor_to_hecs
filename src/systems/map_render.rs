use crate::prelude::*;

pub fn map_render(state: &mut State) {
    let mut draw_batch = DrawBatch::new();
    let camera = &state.camera;
    draw_batch.target(0);
    for y in camera.top_y..=camera.bottom_y {
        for x in camera.left_x..camera.right_x {
            let pt = Point::new(x, y);
            let offset = Point::new(camera.left_x, camera.top_y);
            if state.map.in_bounds(pt) {
                let idx = map_idx(x, y);
                let glyph = match state.map.tiles[idx] {
                    TileType::Floor => to_cp437('.'),
                    TileType::Wall => to_cp437('#'),
                };
                draw_batch.set(
                    // (1)
                    pt - offset,
                    ColorPair::new(WHITE, BLACK),
                    glyph,
                );
            }
        }
    }
    draw_batch.submit(0).expect("Batch error"); // (2)
}
