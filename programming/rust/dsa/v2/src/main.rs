fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err("Division by zero".to_string());
    }
    Ok(a / b)
}

fn main() {
    let a = 12;
    let b = 0;
    let c = 4;
    match divide(a, b) {
        Ok(x) => println!("{}", x),
        _ => println!("Looks like we can't perform {}/{}", a, b),
    }
    println!("{}/{} = {:?}", a, b, divide(12, 0));
    println!("{}/{} = {:?}", a, c, divide(12, 4));
}
