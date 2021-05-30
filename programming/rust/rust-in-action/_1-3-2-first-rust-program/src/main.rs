fn greet() {
    let default_ = "Hello World!";
    let southern_germany = "Grüß Gott!";
    let japan = "ハロー・ワールド"; // HARO WARUDO 😅

    let regions = [default_, southern_germany, japan];

    for region in regions.iter() {
        println!("{}", region);
        // Book had 👆 this line with `&region` but trying without it worked fine.
        // the reasoning should be `println` cannot borrow region.
        // -----------------------------------------------------------------------------------------
        // This stackoverflow thread:
        //  https://stackoverflow.com/questions/30450399/does-println-borrow-or-own-the-variable
        // explains that some macros do this for the sake of being syntactical sugar. 
        // -----------------------------------------------------------------------------------------
        // Ran into [cargo-expand](https://github.com/dtolnay/cargo-expand) that supports a nightly
        // compiler.
        // -----------------------------------------------------------------------------------------
        // Running `cargo expand` gives us the expansion of the macros used.
        // -----------------------------------------------------------------------------------------
        // #![feature(prelude_import)]
        // #[prelude_import]
        // use std::prelude::rust_2018::*;
        // #[macro_use]
        // extern crate std;
        // fn greet() {
        //     let default_ = "Hello World!";
        //     let southern_germany = "Grüß Gott!";
        //     let japan = "ハロー・ワールド";
        //     let regions = [default_, southern_germany, japan];
        //     for region in regions.iter() {
        //         {
        //             ::std::io::_print(::core::fmt::Arguments::new_v1(
        //                 &["", "\n"],
        //                 &match (&region,) { // 👈 Notice the `&region` here.
        //                     (arg0,) => [::core::fmt::ArgumentV1::new(
        //                         arg0,
        //                         ::core::fmt::Display::fmt,
        //                     )],
        //                 },
        //             ));
        //         };
        //     }
        // }
        // fn main() {
        //     greet();
        // }
        
    }
}

fn main() {
    greet();
}
