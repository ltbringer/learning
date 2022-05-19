fn string_find_f(s: &str) -> &str {
     for (n, x) in s.char_indices() {
         if x == 'f' {
             return &s[n..];
         }
     }
     s
}

fn choose_str(n: u8) -> &'static str {
    match n {
        0 => "hello",
        1 => "world",
        _ => "other",
    }
}

fn main() {
    let mut s = "   hello    ".to_string();
    let p = s.trim();
    let p = p.to_string();
    s.push_str(" world");
    println!("p={}\ns={}", p, s);

    let fstr = "help me find home";
    let ffstr = string_find_f(fstr);
    println!("fstr={}\nffstr={}", fstr, ffstr);
    println!("choose_str(4)={}", choose_str(4));
}
