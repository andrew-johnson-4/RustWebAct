use wasm_bindgen::prelude::*;

fn parse_time(t: u64) -> (u64,u64,u64) {
    (t/3600, (t/60)%60, t%60)
}

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    //wasm has no threads
    let i = 99999;
    let (fh,fm,fs) = parse_time(i);
    let (mh,mm,ms) = parse_time(i/2);
    let (sh,sm,ss) = parse_time(i/4);
    body.set_inner_html(
       &format!("<p>{:02}:{:02}:{:02}</p><p>{:02}:{:02}:{:02}</p><p>{:02}:{:02}:{:02}</p>", fh,fm,fs, mh,mm,ms, sh,sm,ss)
    );

    Ok(())
}
