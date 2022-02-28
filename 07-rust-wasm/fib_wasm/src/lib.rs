#[no_mangle]
fn add(x: i32, y:i32) -> i32 {
    x + y
}

#[no_mangle]
fn fibonacci(num: i32) -> i32 {
    match num {
        0 => 0,
        1 => 1,
        _ => fibonacci(num-1) + fibonacci(num-2),
    }
}

