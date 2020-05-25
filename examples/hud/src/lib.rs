#![feature(proc_macro_hygiene)]
use std::cmp::{max,min};
use wasm_bindgen::prelude::*;
use web_sys::console;
use jsmx::{JSMX_EXCHANGE};
use serde_json::{Value,Number,json};
use rustwebact::rwa_time::{set_interval_forget};
use rustwebact::rwa_html::{HtmlActor,progress_bar};
use rdxl::rdxl;
pub mod hud_html; use hud_html::*;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {

    struct CharacterProfile {
       health: (u64,u64),
       energy: (u64,u64),
       mana: (u64,u64),
    }
    HtmlActor::new("topleft", CharacterProfile { health:(777,1000), energy:(82,100), mana:(123,300) }, |stats| {
          rdxl!(<div style="position:absolute; top:0; left:0; width:400px; height:100px; background-color:#666666;">
             {{ progress_bar(&json!({ "progress":[stats.health.0, stats.health.1], "progress_style":{"background-color":"#FF0000"} })) }}
             {{ progress_bar(&json!({ "progress":[stats.energy.0, stats.energy.1], "progress_style":{"background-color":"#FFFF00"} })) }}
             {{ progress_bar(&json!({ "progress":[stats.mana.0, stats.mana.1], "progress_style":{"background-color":"#0000FF"} })) }}
          </div>)
       }, vec![
          ("character","regen_health", Box::new(|cp, v| {
             cp.health.0 = min(cp.health.1, cp.health.0 + (v.as_i64().unwrap() as u64));
             true
          })),
          ("character","regen_energy", Box::new(|cp, v| {
             cp.energy.0 = min(cp.energy.1, cp.energy.0 + (v.as_i64().unwrap() as u64));
             true
          })),
          ("character","regen_mana", Box::new(|cp, v| {
             cp.mana.0 = min(cp.mana.1, cp.mana.0 + (v.as_i64().unwrap() as u64));
             true
          })),
          ("document", "ready", Box::new(|_, _| { true })
       )],
    );

    HtmlActor::new("topright", (), |minimap| {
          "<div style='position: absolute; top: 0; right: 0; width: 200px; height: 200px; background-color: #00FF00;'>
Map Overlay</div>".to_string()
       }, vec![
          ("document", "ready", Box::new(|time, msg| { true })
       )],
    );

    HtmlActor::new("bottomleft", json!({ "input":"", "show":"", "channels":{"local":[], "character":[]} }), |log| {
          rdxl!(<div>
          </div>)
          /*
          format!("<div style='position: absolute; bottom: 40px; left: 0; width: 600px; height: 250px; background-color: #111111; border: 1px solid limegreen;'>{}{}{}</div>",
          chatlog_channels(log),
          chatlog_log(log),
          chatlog_input(log))
          */
       }, vec![
          /*
          ("log", "local", Box::new(|cl, msg| {
             let msg = if let Value::String(c) = msg { c.clone() } else { "".to_string() };
             for (n,log) in cl.channels.iter_mut() {
                if n=="local" {
                  log.push(msg.clone());
                }
             }
             true
          })),
          ("log", "character", Box::new(|cl, msg| {
             let msg = if let Value::String(c) = msg { c.clone() } else { "".to_string() };
             for (n,log) in cl.channels.iter_mut() {
                if n=="character" {
                  log.push(msg.clone());
                }
             }
             true
          })),
          ("log", "set_show", Box::new(|cl, msg| {
             let msg = if let Value::String(c) = msg { c.clone() } else { "".to_string() };
             if msg!=cl.show {
                cl.show = msg;
                true
             } else { false }
          })),
          ("document", "keydown", Box::new(|cl, msg| {
             let keyCode = msg["keyCode"].as_str().unwrap();
             cl.input += keyCode;
             true
          })),
          */
          ("document", "ready", Box::new(|_, _| { true })
       )],
    );

    HtmlActor::new("bottomright", (), |help| {
          "<div style='position: absolute; bottom: 40px; right: 0; width: 300px; height: 400px; background-color: #FFFF00;'>
Help Text</div>".to_string()
       }, vec![
          ("document", "ready", Box::new(|time, msg| { true })
       )],
    );

    HtmlActor::new("bottombottom", (), |notifications| {
          "<div style='position: absolute; bottom: 0; left: 0; width: 100%; height: 40px; background-color: #444444;'>
Action Bar</div>".to_string()
       }, vec![
          ("document", "ready", Box::new(|time, msg| { true })
       )],
    );

    set_interval_forget(|| { JSMX_EXCHANGE.push("character","regen_health",&json!(10)) }, 5000);
    set_interval_forget(|| { JSMX_EXCHANGE.push("character","regen_energy",&json!(5)) }, 1000);
    set_interval_forget(|| { JSMX_EXCHANGE.push("character","regen_mana",&json!(5)) }, 5000);

    let mut haltn = 0;
    set_interval_forget(move || {
       haltn += 1;
       let s = format!("Halt, who goes there! #{}", haltn);
       JSMX_EXCHANGE.push("log","local",&json!(s))
    }, 5000);

    JSMX_EXCHANGE.push("document","ready",&Value::Null);
    Ok(())
}

#[wasm_bindgen]
pub fn jsmx_push(r1: &str, r2: &str, msg: &JsValue) {
   let msg: Value = msg.into_serde().unwrap();
   JSMX_EXCHANGE.push(r1,r2,&msg);
}
