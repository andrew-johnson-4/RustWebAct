use wasm_bindgen::prelude::*;
use std::sync::Mutex;
use serde_json::{Value};

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    let start_time = 9*3600 + 10*60 + 11;

    let mut fast_timer: Mutex<u64> = Mutex::new(start_time);
    HtmlActor::new("#fast_timer", "{:02}:{:02}:{:02}", vec![
       ("fast_timer", "tick", move |msg| {
          let mut timer = fast_timer.lock().unwrap();
          *timer += 1;
       }),
       ("ntp", "set", move |msg| {
          if let Value::Number(time) = msg {
             let mut timer = fast_timer.lock().unwrap();
             *timer = time.as_u64().expect("ntp expected u64 time value");
          }
       }),
    ]);

    setIntervalForget(|| { jsmx_exchange.push("fast_timer","tick",Value::Null) }, 900);
    setIntervalForget(|| { jsmx_exchange.push("mid_timer","tick",Value::Null) }, 1000);
    setIntervalForget(|| { jsmx_exchange.push("slow_timer","tick",Value::Null) }, 1100);

    let mut ntp_timer = start_time;
    setIntervalForget(move || {
       ntp_timer += 60;
       jsmx_exchange.push("ntp","set",Value::Number(ntp_timer));
    }, 60000);

    Ok(())
}
