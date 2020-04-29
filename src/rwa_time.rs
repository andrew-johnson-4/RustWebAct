use wasm_bindgen::prelude::*;

/*
   !! hide unstable interfaces behind stable interfaces

   here within lies the javascript setInterval and related interfaces
   the related wasm_bindgen interface is roughly hewn so we will provide a shim
*/

#[wasm_bindgen]
extern "C" {
    fn setInterval(closure: &Closure<dyn FnMut()>, time: u32) -> i32;
    fn clearInterval(id: i32);
}

pub fn set_interval_forget<F>(f: F, time: u32)
   where F: 'static + FnMut() {
   let cb = Closure::wrap(Box::new(f) as Box<dyn FnMut()>);
   setInterval(&cb, time);
   cb.forget();
}
