fn main() {
    let haystack = [1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862];

    for item in haystack.iter() {
        println!("{} -> {}", item, 
            match item {
                42 | 132 => "hit!",
                _ => "miss"
        });
    }
  
}
