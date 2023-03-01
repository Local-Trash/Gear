use fish::*;
// You do need to inport this in you .toml file
use pollster::*;

fn main() {
    let ctx = Context::new()
        .withTitle("Window");

    let engine: Engine = block_on(Engine::new(ctx));

    engine.run();
}