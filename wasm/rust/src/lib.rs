use wasm_bindgen::prelude::*;

// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn fibonacci(n: i32) -> i32 {
    if n < 2 {
        n
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
    // let mut a = 0;
	// let mut b = 1;
    
	// for _ in 0..n {
	// 	let new = a + b;
	// 	a = b;
	// 	b = new;
	// }
	
	// return a
}
