pub mod collisions;
pub mod end_turn;
pub mod entity_render;
pub mod map_render;
pub mod movement;
pub mod player_input;
pub mod random_move;

pub fn run_systems(state: &mut State) -> State {
    let current_state = state.turnstate;
    match current_state {
        TurnState::AwaitingInput => state = input_systems(&mut state),
        TurnState::PlayerTurn => state = player_systems(&mut state),
        TurnState::MonsterTurn => state = monster_systems(&mut state),
    }
    state
}

fn input_systems(state: &mut State) -> State {
    state = player_input::player_input(&mut state); //read and write system
    map_render::map_render(state); //read only system
    entity_render::entity_render(state); //read only system
    state
}

fn player_systems(state: &mut State) -> State {
    //filler
    state
}

fn monster_systems(state: &mut State) -> State {
    //filler
    state
}
