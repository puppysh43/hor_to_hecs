use crate::prelude::*;

pub fn random_move(state: &mut State) {
    let commands = &mut CommandBuffer::new();

    for (entity, pos) in state.ecs.query_mut::<With<&Point, &MovingRandomly>>() {
        let mut rng = RandomNumberGenerator::new();
        let destination = match rng.range(0, 4) {
            0 => Point::new(-1, 0),
            1 => Point::new(1, 0),
            2 => Point::new(0, -1),
            _ => Point::new(0, 1),
        } + *pos;
        commands.spawn((
            (),
            WantsToMove {
                entity,
                destination,
            },
        ));
    }

    commands.run_on(&mut state.ecs);
}
