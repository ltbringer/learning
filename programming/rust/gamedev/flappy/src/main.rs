use bracket_lib::prelude::*;


fn main() -> BError {
    let ctx = BTermBuilder::simple80x50()
        .with_title("Hello Rust World!")
        .build()?;
    main_loop(ctx, State::new())
}
