#[derive(Debug)]
enum Cereal {
    Barley, Millet, Rice,
    Rye, Spelt, Wheat,
}

fn main() {
    let mut grains: Cereal = Cereal::Rye;
    println!("{:p}", &grains);
    drop(grains);
    println!("{:?}", grains);
}
