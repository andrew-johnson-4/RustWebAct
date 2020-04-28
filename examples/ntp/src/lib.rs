use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn setInterval(closure: &Closure<FnMut()>, time: u32) -> i32;
    fn clearInterval(id: i32);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct IntervalHandle {
    interval_id: i32,
    _closure: Closure<FnMut()>,
}
impl Drop for IntervalHandle {
    fn drop(&mut self) {
        clearInterval(self.interval_id);
    }
}

fn parse_time(t: u64) -> (u64,u64,u64) {
    (t/3600, (t/60)%60, t%60)
}

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {

    let cb = Closure::wrap(Box::new(|| {
        log("interval elapsed!");
    }) as Box<FnMut()>);
    let interval_id = setInterval(&cb, 1_000);

    let ih = IntervalHandle {
        interval_id,
        _closure: cb,
    };

    /*
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let mut i = 99999;
    Interval::new(1000, || {
       let (fh,fm,fs) = parse_time(i);
       let (mh,mm,ms) = parse_time(i/2);
       let (sh,sm,ss) = parse_time(i/4);
       body.set_inner_html(
          &format!("<p>{:02}:{:02}:{:02}</p><p>{:02}:{:02}:{:02}</p><p>{:02}:{:02}:{:02}</p>", fh,fm,fs, mh,mm,ms, sh,sm,ss)
       );
       i += 1;
    })
    */

    Ok(())
}
