use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Point {
    x: i32,
    y: i32,
}

#[wasm_bindgen]
impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y}
    }

    pub fn get_x(&self) -> i32 {
        self.x
    }

    pub fn get_y(&self) -> i32 {
        self.y
    }

    pub fn set_x(&mut self, x: i32) {
        self.x = x;
    }

    pub fn set_y(&mut self, y:i32) {
        self.y = y;
    }

    pub fn add(&mut self, p: Point) {
        self.x = self.x + p.x;
        self.y = self.y + p.y;
     }
}