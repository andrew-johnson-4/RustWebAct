use wasm_bindgen::prelude::*;

fn parse_time(t: u64) -> (u64,u64,u64) {
    (t/3600, (t/60)%60, t%60)
}

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {

let window = web_sys::window().expect("should have a window in this context");
    let document = window.document().expect("window should have a document");

    document
        .get_element_by_id("fast_timer")
        .expect("should have #fast_timer on the page")
        .set_inner_html(&format!("{:02}:{:02}:{:02}", 9, 14, 11));
    document
        .get_element_by_id("mid_timer")
        .expect("should have #mid_timer on the page")
        .set_inner_html(&format!("{:02}:{:02}:{:02}", 8, 14, 11));
    document
        .get_element_by_id("slow_timer")
        .expect("should have #slow_timer on the page")
        .set_inner_html(&format!("{:02}:{:02}:{:02}", 7, 14, 11));

    Ok(())
}
