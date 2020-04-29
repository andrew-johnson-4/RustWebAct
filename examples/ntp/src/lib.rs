use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn setInterval(closure: &Closure<dyn FnMut()>, time: u32) -> i32;
    fn clearInterval(id: i32);
}

fn parse_time(t: u64) -> (u64,u64,u64) {
    (t/3600, (t/60)%60, t%60)
}

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {

    let mut fast_seconds = 9*3600 + 10*60 + 11;
    let cb = Closure::wrap(Box::new(move || {
       let (h,m,s) = parse_time(fast_seconds);
       fast_seconds += 1;
       let window = web_sys::window().expect("should have a window in this context");
       let document = window.document().expect("window should have a document");
       document
           .get_element_by_id("fast_timer")
           .expect("should have #fast_timer on the page")
           .set_inner_html(&format!("{:02}:{:02}:{:02}", h, m, s));
    }) as Box<dyn FnMut()>);
    setInterval(&cb, 900);
    cb.forget();

    let mut mid_seconds = 9*3600 + 10*60 + 11;
    let cb = Closure::wrap(Box::new(move || {
       let (h,m,s) = parse_time(mid_seconds);
       mid_seconds += 1;
       let window = web_sys::window().expect("should have a window in this context");
       let document = window.document().expect("window should have a document");
       document
           .get_element_by_id("mid_timer")
           .expect("should have #mid_timer on the page")
           .set_inner_html(&format!("{:02}:{:02}:{:02}", h, m, s));
    }) as Box<dyn FnMut()>);
    setInterval(&cb, 1000);
    cb.forget();

    let mut slow_seconds = 9*3600 + 10*60 + 11;
    let cb = Closure::wrap(Box::new(move || {
       let (h,m,s) = parse_time(slow_seconds);
       slow_seconds += 1;
       let window = web_sys::window().expect("should have a window in this context");
       let document = window.document().expect("window should have a document");
       document
           .get_element_by_id("slow_timer")
           .expect("should have #slow_timer on the page")
           .set_inner_html(&format!("{:02}:{:02}:{:02}", h, m, s));
    }) as Box<dyn FnMut()>);
    setInterval(&cb, 1100);
    cb.forget();

    Ok(())
}
