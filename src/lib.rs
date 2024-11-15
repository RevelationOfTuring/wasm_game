// wasm_bindgen用于js与rust交互
use wasm_bindgen::prelude::*;

// 从外部将js的alert方法import进来，这样wasm就可以在内部使用alert方法了
#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen] // 被wasm_bindgen修饰的方法才会被编译器认作是用于wasm的
pub fn hello(name: &str) {
    // alert为js的方法
    alert(name);
}
