#[derive(Debug)]
enum Cereal {
    Barley, Millet, Rice,
    Rye, Spelt, Wheat,
}

fn main() {
    let mut grains: Vec<Cereal> = vec![];
    grains.push(Cereal::Rye);
    drop(grains);
    // We saw in _1-3-2-first-rust-program that println! borrows the value
    // In this case, the value is owned by drop and its no longer in the scope of the code.
    // When we try to run println! afterwards, we get code that fails to compile.
    // 
    // particularly:
    // 
    //     error[E0382]: borrow of moved value: `grains`
    //   --> src/main.rs:12:22
    //    |
    // 8  |     let mut grains: Vec<Cereal> = vec![];
    //    |         ---------- move occurs because `grains` has type `Vec<Cereal>`, which does not implement the `Copy` trait
    // 9  |     grains.push(Cereal::Rye);
    // 10 |     drop(grains);
    //    |          ------ value moved here
    // 11 |
    // 12 |     println!("{:?}", grains);
    //    |                      ^^^^^^ value borrowed here after move
    println!("{:?}", grains);
}
