mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, dfly!");
}

#[wasm_bindgen]
pub fn add(x: i32, y: i32) -> i32 {
    x + y
}

#[wasm_bindgen]
pub fn sum(data: Vec<i32>) -> i32 {
    data.iter().sum()
}

#[wasm_bindgen]
pub struct Arr1D {
    data: Vec<i32>,
}

#[wasm_bindgen]
impl Arr1D {
    pub fn new(data: Vec<i32>) -> Arr1D {
        Arr1D { data }
    }

    pub fn copy(&mut self) -> Arr1D {
        Arr1D {
            data: self.data.clone(),
        }
    }
    
    pub fn len(&mut self) -> usize {
        self.data.len()
    }

    pub fn sum(&mut self) -> i32 {
        self.data.iter().sum()
    }

    pub fn mean(&mut self) -> f64 {
        self.data.iter().sum::<i32>() as f64 / self.data.len() as f64
    }
}

