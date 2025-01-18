use smart_cache_macro::cached;

#[cached]
fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn main() {
    assert_eq!(fibonacci(5), 5);
    assert_eq!(fibonacci(5), 5); // Should hit cache
    assert_eq!(fibonacci(6), 8);
}
