#![cfg(all(target_arch = "wasm32", target_os = "emscripten"))]

extern "C" {
    fn open(_: *const (), _: i32, _: i32) -> i32;
}

#[no_mangle]
pub extern "C" fn open64(a: *const (), b: i32, c: i32) -> i32 {
    unsafe { open(a, b, c) }
}
