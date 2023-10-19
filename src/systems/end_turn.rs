use crate::prelude::*;

pub fn end_turn(state: &mut State) {
    let new_state = match state.turnstate {
        TurnState::AwaitingInput => return,                 // (1)
        TurnState::PlayerTurn => TurnState::MonsterTurn,    // (2)
        TurnState::MonsterTurn => TurnState::AwaitingInput, // (3)
    };

    state.turnstate = new_state; // (4)
}
