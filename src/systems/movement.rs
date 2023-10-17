use crate::prelude::*;
/*
pub fn movement(
    entity: &Entity,
    want_move: &WantsToMove,
    #[resource] map: &Map,
    #[resource] camera: &mut Camera,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
)
this used to be a for_each system meaning I need to add in a for_each loop to go through all this crap for every movement intent message
*/
pub fn movement(state: &mut State) {
    if map.can_enter_tile(want_move.destination) {
        commands.add_component(want_move.entity, want_move.destination); // (2)

        if ecs
            .entry_ref(want_move.entity) // (3)
            .unwrap() // (4)
            .get_component::<Player>()
            .is_ok()
        // (5)
        {
            camera.on_player_move(want_move.destination); // (6)
        }
    }
    commands.remove(*entity); // (7)
}
