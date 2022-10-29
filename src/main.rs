#![warn(clippy::all, clippy::pedantic)]
mod map;
mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub use crate::map::*;
}
use prelude::*;

struct State {
    map: Map,
}

impl GameState for State {
    fn tick(&mut self, context: &mut BTerm) {
        context.cls();
        self.map.render(context);
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Rusty Dungeon")
        .with_fps_cap(30.0)
        .build()?;
    main_loop(context, State { map: Map::new() })
}
