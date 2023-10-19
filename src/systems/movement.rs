use crate::prelude::*;

pub fn movement(state: &mut State) {
    let commands = &mut CommandBuffer::new();
    for (entity, want_move) in state.ecs.query::<&WantsToMove>().iter() {
        if state.map.can_enter_tile(want_move.destination) {
            commands.insert_one(want_move.entity, want_move.destination);
            for (player_id, _) in state.ecs.query::<&Player>().iter() {
                if player_id == want_move.entity {
                    state.camera.on_player_move(want_move.destination);
                }
            }
        }
        commands.despawn(entity); // (7)
    }
    commands.run_on(&mut state.ecs);
}
