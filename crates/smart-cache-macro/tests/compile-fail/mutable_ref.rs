use smart_cache_macro::cached;

#[cached]
fn impure_function(x: &mut i32) -> i32 {
    *x += 1;
    *x
}

fn main() {
    let mut x = 42;
    impure_function(&mut x);
}
