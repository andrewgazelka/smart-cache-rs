use smart_cache_macro::cached;

#[cached]
fn takes_ref(s: String) -> String {
    s
}

fn main() {
    takes_ref("hello".to_string());
}
