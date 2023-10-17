use crate::prelude::*;
/*
pub fn end_turn(#[resource] turn_state: &mut TurnState) */
pub fn end_turn(state: &mut State) {
    let new_state = match turn_state {
        TurnState::AwaitingInput => return,                 // (1)
        TurnState::PlayerTurn => TurnState::MonsterTurn,    // (2)
        TurnState::MonsterTurn => TurnState::AwaitingInput, // (3)
    };

    *turn_state = new_state; // (4)
}
