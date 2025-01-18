use smart_cache_macro::cached;

#[cached]
fn unit(s: String) -> String {
    s
}

fn main() {
    let result = unit("hello".to_string());
    assert_eq!(result, "hello");
}
