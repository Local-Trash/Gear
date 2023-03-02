use fish::*;
// You do need to inport this in you .toml file
use pollster::*;

fn main() {
    let ctx = Context::new()
        .withTitle("Window");

    let mut engine = block_on(Engine::new(ctx));
    engine.insertUpdate(|_,_| {
        println!("WIndow is running");
    });

    engine.run();
}