use bracket_lib::prelude::*;

struct State {}

impl GameState for State {
    fn tick(&mut self, context: &mut BTerm) {
        context.cls();
        context.print(1, 1, "Ah sh*t, here we go again...");
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Rusty Dungeon")
        .build()?;
    main_loop(context, State {})
}
