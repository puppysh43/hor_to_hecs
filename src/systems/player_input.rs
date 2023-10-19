use crate::prelude::*;

pub fn player_input(state: &mut State) {
    let key = state.key;
    let commands = &mut CommandBuffer::new();
    // let mut players = <(Entity, &Point)>::query().filter(component::<Player>()); //hasn't been changed from legion yet
    let players = state.ecs.query_mut::<With<&Point, &Player>>();
    if let Some(key) = key {
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            _ => Point::new(0, 0),
        };
        for (entity, pos) in players {
            let destination = *pos + delta;
            commands.spawn((
                (),
                WantsToMove {
                    entity,
                    destination,
                },
            ));
        }

        state.turnstate = TurnState::PlayerTurn;
    }
    commands.run_on(&mut state.ecs);
}
