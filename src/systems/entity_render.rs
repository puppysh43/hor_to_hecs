use crate::prelude::*;

pub fn entity_render(state: &mut State) {
    //need ECS and camera
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1);
    let offset = Point::new(state.camera.left_x, state.camera.top_y);

    for (_, (pos, render)) in state.ecs.query_mut::<(&Point, &Render)>() {
        draw_batch.set(*pos - offset, render.color, render.glyph);
    }

    draw_batch.submit(5000).expect("Batch error");
}
