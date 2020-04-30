use wasm_bindgen::prelude::*;
use jsmx::{JSMX_EXCHANGE};
use rustwebact::rwa_time::{set_interval_forget};
use std::sync::Mutex;
use std::sync::Arc;
use serde_json::{Value, Number};

fn parse_time(t: u64) -> (u64,u64,u64) {
    (t/3600, (t/60)%60, t%60)
}

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    let start_time = 9*3600 + 10*60 + 11;

    let fast_seconds: Arc<Mutex<u64>> = Arc::new(Mutex::new(start_time));
    let timer = fast_seconds.clone();
    JSMX_EXCHANGE.listen("fast_timer", "tick", move |_msg| {
       let mut fast_seconds = timer.lock().unwrap();
       *fast_seconds += 1;
       let (h,m,s) = parse_time(*fast_seconds);
       let window = web_sys::window().expect("should have a window in this context");
       let document = window.document().expect("window should have a document");
       document
           .get_element_by_id("fast_timer")
           .expect("should have #fast_timer on the page")
           .set_inner_html(&format!("{:02}:{:02}:{:02}", h, m, s));
    });
    let timer = fast_seconds.clone();
    JSMX_EXCHANGE.listen("ntp", "set", move |msg| {
       if let Value::Number(n) = msg {
          let n = n.as_u64().unwrap();
          let mut fast_seconds = timer.lock().unwrap();
          *fast_seconds = n;
          let (h,m,s) = parse_time(n);
          let window = web_sys::window().expect("should have a window in this context");
          let document = window.document().expect("window should have a document");
          document
             .get_element_by_id("fast_timer")
             .expect("should have #fast_timer on the page")
             .set_inner_html(&format!("{:02}:{:02}:{:02}", h, m, s));
       }
    });

    let mid_seconds: Arc<Mutex<u64>> = Arc::new(Mutex::new(start_time));
    let timer = mid_seconds.clone();
    JSMX_EXCHANGE.listen("mid_timer", "tick", move |_msg| {
       let mut mid_seconds = timer.lock().unwrap();
       *mid_seconds += 1;
       let (h,m,s) = parse_time(*mid_seconds);
       let window = web_sys::window().expect("should have a window in this context");
       let document = window.document().expect("window should have a document");
       document
           .get_element_by_id("mid_timer")
           .expect("should have #mid_timer on the page")
           .set_inner_html(&format!("{:02}:{:02}:{:02}", h, m, s));
    });
    let timer = mid_seconds.clone();
    JSMX_EXCHANGE.listen("ntp", "set", move |msg| {
       if let Value::Number(n) = msg {
          let n = n.as_u64().unwrap();
          let mut mid_seconds = timer.lock().unwrap();
          *mid_seconds = n;
          let (h,m,s) = parse_time(n);
          let window = web_sys::window().expect("should have a window in this context");
          let document = window.document().expect("window should have a document");
          document
             .get_element_by_id("mid_timer")
             .expect("should have #mid_timer on the page")
             .set_inner_html(&format!("{:02}:{:02}:{:02}", h, m, s));
       }
    });

    let slow_seconds: Arc<Mutex<u64>> = Arc::new(Mutex::new(start_time));
    let timer = slow_seconds.clone();
    JSMX_EXCHANGE.listen("slow_timer", "tick", move |_msg| {
       let mut slow_seconds = timer.lock().unwrap();
       *slow_seconds += 1;
       let (h,m,s) = parse_time(*slow_seconds);
       let window = web_sys::window().expect("should have a window in this context");
       let document = window.document().expect("window should have a document");
       document
           .get_element_by_id("slow_timer")
           .expect("should have #slow_timer on the page")
           .set_inner_html(&format!("{:02}:{:02}:{:02}", h, m, s));
    });
    let timer = slow_seconds.clone();
    JSMX_EXCHANGE.listen("ntp", "set", move |msg| {
       if let Value::Number(n) = msg {
          let n = n.as_u64().unwrap();
          let mut slow_seconds = timer.lock().unwrap();
          *slow_seconds = n;
          let (h,m,s) = parse_time(n);
          let window = web_sys::window().expect("should have a window in this context");
          let document = window.document().expect("window should have a document");
          document
             .get_element_by_id("slow_timer")
             .expect("should have #slow_timer on the page")
             .set_inner_html(&format!("{:02}:{:02}:{:02}", h, m, s));
       }
    });

    /*
    HtmlActor::new("#fast_timer", start_time, |&mut time| {
          let (h,m,s) = parse_time(time);
          format!("{:02}:{:02}:{:02}", h, m, s)
       }, vec![
       ("fast_timer", "tick", move |&mut time, msg| {
          let mut timer = fast_timer.lock().unwrap();
          *timer += 1;
          true
       }),
       ("ntp", "set", move |&mut time, msg| {
          if let Value::Number(time) = msg {
             let mut timer = fast_timer.lock().unwrap();
             *timer = time.as_u64().expect("ntp expected u64 time value");
          };
          true
       })],
    );
    */

    set_interval_forget(|| { JSMX_EXCHANGE.push("fast_timer","tick",&Value::Null) }, 900);
    set_interval_forget(|| { JSMX_EXCHANGE.push("mid_timer","tick",&Value::Null) }, 1000);
    set_interval_forget(|| { JSMX_EXCHANGE.push("slow_timer","tick",&Value::Null) }, 1100);

    let mut ntp_timer = start_time;
    set_interval_forget(move || {
       ntp_timer += 60;
       JSMX_EXCHANGE.push("ntp","set",&Value::Number(Number::from(ntp_timer)));
    }, 60000);

    Ok(())
}
