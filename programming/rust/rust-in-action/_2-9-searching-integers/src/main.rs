fn main() {
    let needle = 0o52;
    let haystack = [1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862];

    // Alternatively `&haystack` can be used instead of `haystack.iter()`
    // but that usually is not available to all types. It is better to expect 
    // or implement the trait... At this point I don't fully understand what traits are.
    for item in haystack.iter() {
        // If the access to value via `*item` bothers 
        // then `haystack.into_iter()` produces an 
        // iterable over values directly instead of references.
        if *item == needle {
            println!("{}", item);
        }
    }
}
