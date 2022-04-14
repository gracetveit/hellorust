use rltk::{GameState, Rltk};

struct State {} // Defines the struct STtate

impl GameState for State {
    // Implements the *trait* GameState onto the State struct
    fn tick(&mut self, ctx: &mut Rltk) {
        // tick is a special function that is called every frame the GameState
        // is running
        ctx.cls();
        // Generally speaking, you want to clear the screen at the beginning of
        // a frame, unless you specifically don't want to
        ctx.print(1, 1, "Hello Rust World");
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;
    let gs = State {};
    rltk::main_loop(context, gs)
}
