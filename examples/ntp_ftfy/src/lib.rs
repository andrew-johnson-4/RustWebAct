use wasm_bindgen::prelude::*;
use jsmx::{JSMX_EXCHANGE};
use rustwebact::rwa_time::{set_interval_forget};
use rustwebact::rwa_html::{HtmlActor};
use serde_json::{Value, Number};

fn parse_time(t: u64) -> (u64,u64,u64) {
    (t/3600, (t/60)%60, t%60)
}

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    let start_time = 9*3600 + 10*60 + 11;

    HtmlActor::new("fast_timer", start_time, |time| {
          let (h,m,s) = parse_time(*time);
          format!("{:02}:{:02}:{:02}", h, m, s)
       }, vec![
       ("fast_timer", "tick", Box::new(|time, _msg| {
          *time += 1;
          true
       })),
       ("ntp", "set", Box::new(|time, msg| {
          if let Value::Number(n) = msg {
             *time = n.as_u64().unwrap();
          };
          true
       }))],
    );

    HtmlActor::new("mid_timer", start_time, |time| {
          let (h,m,s) = parse_time(*time);
          format!("{:02}:{:02}:{:02}", h, m, s)
       }, vec![
       ("mid_timer", "tick", Box::new(|time, _msg| {
          *time += 1;
          true
       })),
       ("ntp", "set", Box::new(|time, msg| {
          if let Value::Number(n) = msg {
             *time = n.as_u64().unwrap();
          };
          true
       }))],
    );

    HtmlActor::new("slow_timer", start_time, |time| {
          let (h,m,s) = parse_time(*time);
          format!("{:02}:{:02}:{:02}", h, m, s)
       }, vec![
       ("slow_timer", "tick", Box::new(|time, _msg| {
          *time += 1;
          true
       })),
       ("ntp", "set", Box::new(|time, msg| {
          if let Value::Number(n) = msg {
             *time = n.as_u64().unwrap();
          };
          true
       }))],
    );

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
