use crate::prelude::*;

pub fn player_input(state: &mut State) {
    // let &mut ecs = state.ecs;
    let key = state.key;
    // let &mut commands = state.command_buffer;
    let turn_state = state.turnstate;
    // let mut players = <(Entity, &Point)>::query().filter(component::<Player>()); //hasn't been changed from legion yet
    let mut players = state.ecs.query::<(&Entity, &Point)>();
    if let Some(key) = key {
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            _ => Point::new(0, 0),
        };

        players.iter(ecs).for_each(|(entity, pos)| {
            let destination = *pos + delta;
            state.command_buffer.spawn((
                (),
                WantsToMove {
                    entity: *entity,
                    destination,
                },
            ));
        });
        state.turnstate = TurnState::PlayerTurn;
    }
}
