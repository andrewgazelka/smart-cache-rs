use smart_cache_macro::cached;

#[cached]
fn expensive_computation(x: &str, y: i32) -> String {
    use std::{thread, time::Duration};

    thread::sleep(Duration::from_secs(3));

    format!("example computation {x}_{y}")
}

#[test]
fn test_cached() {
    let x = expensive_computation("hello", 2);
    println!("{x}");
}
