use crate::prelude::*;
pub mod collisions;
pub mod end_turn;
pub mod entity_render;
pub mod map_render;
pub mod movement;
pub mod player_input;
pub mod random_move;

pub fn run_systems(state: &mut State) {
    let current_turn = state.turnstate;
    match current_turn {
        TurnState::AwaitingInput => input_systems(state),
        TurnState::PlayerTurn => player_systems(state),
        TurnState::MonsterTurn => monster_systems(state),
    }
}
//ALL SYSTEMS NEED TO BE CONVERTED.
fn input_systems(state: &mut State) {
    player_input::player_input(state); //read and write system
    map_render::map_render(state); //read only system
    entity_render::entity_render(state); //read only system
}

fn player_systems(state: &mut State) {
    movement::movement(state);
    collisions::collisions(state);
    map_render::map_render(state);
    entity_render::entity_render(state);
    end_turn::end_turn(state);
}

fn monster_systems(state: &mut State) {
    random_move::random_move(state);
    movement::movement(state);
    map_render::map_render(state);
    entity_render::entity_render(state);
    end_turn::end_turn(state);
}
