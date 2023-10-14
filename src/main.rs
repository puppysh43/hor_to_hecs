#![warn(clippy::pedantic)]

mod camera;
mod components;
mod map;
mod map_builder;
mod spawner;
mod systems;
mod turn_state;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use hecs::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
    pub use crate::turn_state::*;
}

use prelude::*;

struct State {
    ecs: World, //will need to switch the ecs variable from a legion world to a hecs world
    key: Option<VirtualKeyCode>,
    turnstate: TurnState,
    camera: Camera,
    map: Map,
    command_buffer: CommandBuffer,
}

impl State {
    fn new() -> Self {
        let mut ecs = World::new();
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);
        spawn_player(&mut ecs, map_builder.player_start);
        map_builder
            .rooms
            .iter()
            .skip(1)
            .map(|r| r.center())
            .for_each(|pos| spawn_monster(&mut ecs, &mut rng, pos));
        Self {
            ecs: ecs,
            key: None,
            turnstate: TurnState::AwaitingInput,
            camera: Camera::new(map_builder.player_start),
            map: map_builder.map,
            command_buffer: CommandBuffer::new(),
        }
    }

    //TICK SYSTEMS BELOW
    /*
    fn input_systems(&mut self, ctx: &mut BTerm) {
        use systems::*;
        player_input::player_input(
            &mut self.ecs,
            &mut self.command_buffer,
            &self.key,
            &mut self.turnstate,
        ); //TODO make compatible with HECS
        map_render::map_render(&self.map, &self.camera);
        //TODO make compatible with HECS
        entity_render::entity_render(&self.ecs, &self.camera);
        //TODO make compatible with HECS
    }

    fn player_systems(&mut self, ctx: &mut BTerm) {
        use systems::*;
        movement::movement(); //need to tweak this to not use for_each macro
                              //TODO make compatible with HECS
        collisions::collisions();
        //TODO make compatible with HECS
        map_render::map_render(&self.map, &self.camera);
        //TODO make compatible with HECS
        entity_render::entity_render(&self.ecs, &self.camera);
        //TODO make compatible with HECS
        end_turn::end_turn();
        //TODO make compatible with HECS
    }

    fn monster_systems(&mut self, ctx: &mut BTerm) {
        use systems::*;
        random_move::random_move();
        movement::movement();
        map_render::map_render(&self.map, &self.camera);
        entity_render::entity_render(&self.ecs, &self.camera);
        end_turn::end_turn();
    }
    */
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        self.key = ctx.key;
        state = systems::run_systems(self);
        render_draw_buffer(ctx).expect("Render error");
    }
}

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_title("Dungeon Crawler")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_resource_path("resources/")
        .with_font("dungeonfont.png", 32, 32)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .build()?;

    main_loop(context, State::new())
}
