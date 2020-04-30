use wasm_bindgen::prelude::*;
use jsmx::{JSMX_EXCHANGE};
use rustwebact::rwa_time::{set_interval_forget};
use std::sync::Mutex;
use serde_json::{Value, Number};

fn parse_time(t: u64) -> (u64,u64,u64) {
    (t/3600, (t/60)%60, t%60)
}

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    let start_time = 9*3600 + 10*60 + 11;

    let mut fast_timer: Mutex<u64> = Mutex::new(start_time);
    /*
    HtmlActor::new("#fast_timer", || {
          let mut timer = fast_timer.lock().unwrap();
          let (h,m,s) = parse_time(*timer);
          format!("{:02}:{:02}:{:02}", h, m, s)
       }, vec![
       ("fast_timer", "tick", move |msg| {
          let mut timer = fast_timer.lock().unwrap();
          *timer += 1;
          true
       }),
       ("ntp", "set", move |msg| {
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
