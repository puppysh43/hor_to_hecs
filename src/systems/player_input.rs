use crate::prelude::*;

pub fn player_input(state: &mut State) -> State {
    let &mut ecs = state.ecs;
    let key = state.key;
    let &mut commands = state.command_buffer;
    let turn_state = state.turn_state;
    let mut players = <(Entity, &Point)>::query().filter(component::<Player>()); //hasn't been changed from legion yet

    if let Some(key) = *key {
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            _ => Point::new(0, 0),
        };

        players.iter(ecs).for_each(|(entity, pos)| {
            let destination = *pos + delta;
            commands.spawn((
                (),
                WantsToMove {
                    entity: *entity,
                    destination,
                },
            ));
        });
        *turn_state = TurnState::PlayerTurn;
    }
    state.ecs = ecs;
    state.turn_state = turn_state;
    state.command_buffer = commands;
    state
}
