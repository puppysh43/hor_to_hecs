use crate::prelude::*;

pub fn collisions(state: &mut State) {
    let commands = &mut CommandBuffer::new();
    let mut player_pos = Point::zero(); //will hold player position for comparison reasons later

    for (_, pos) in state.ecs.query_mut::<With<&Point, &Player>>() {
        player_pos = *pos; //gets player's position for later
    }

    for (enemy_entity, enemy_pos) in state.ecs.query_mut::<With<&Point, &Enemy>>() {
        if player_pos == *enemy_pos {
            //if the position of the enemy is the same as the position of the player
            commands.despawn(enemy_entity); //then remove the enemy!
        }
    }
    commands.run_on(&mut state.ecs);
}
