use crate::prelude::*;

pub fn entity_render(state: &mut State) {
    //need ECS and camera
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1);
    let offset = Point::new(camera.left_x, camera.top_y);

    <(&Point, &Render)>::query()
        .iter(ecs)
        .for_each(|(pos, render)| {
            draw_batch.set(*pos - offset, render.color, render.glyph);
        });
    draw_batch.submit(5000).expect("Batch error");
}
