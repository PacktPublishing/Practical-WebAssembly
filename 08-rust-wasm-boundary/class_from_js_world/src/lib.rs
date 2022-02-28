use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "./point")] . // 1
extern "C" {
     pub type Point; // 2

    #[wasm_bindgen(constructor)] //3
    fn new(x: i32, y: i32) -> Point;

    #[wasm_bindgen(method, getter)] //4
    fn get_x(this: &Point) -> i32;

    #[wasm_bindgen(method, getter)]
    fn get_y(this: &Point) -> i32;

    #[wasm_bindgen(method, setter)] //5
    fn set_x(this: &Point, x:i32) -> i32;

    #[wasm_bindgen(method, setter)]
    fn set_y(this: &Point, y:i32) -> i32;

    #[wasm_bindgen(method)] // 6
    fn add(this: &Point, p: Point);
}

#[wasm_bindgen]
fn get_precious_point() -> Point { //7
    let p = Point::new(10, 10);
    let p1 = Point::new(3, 3);
    p.add(p1); // 8
    p
}