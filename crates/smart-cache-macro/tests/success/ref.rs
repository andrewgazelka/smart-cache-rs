use smart_cache_macro::cached;

#[cached]
fn pure_function(x: &i32) -> i32 {
    *x + 1
}

fn main() {
    let x = 42;
    assert_eq!(pure_function(&x), 43);
    assert_eq!(pure_function(&x), 43); // Should hit cache
}
