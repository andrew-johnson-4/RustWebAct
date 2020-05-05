use wasm_bindgen::prelude::*;
use jsmx::{JSMX_EXCHANGE};
use serde_json::{Value};
use rustwebact::rwa_time::{set_interval_forget};
use rustwebact::rwa_html::{HtmlActor};

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {

    HtmlActor::new("topleft", (), |character_profile| {
          "<div style='position: absolute; top: 0; left: 0; width: 300px; height: 100px; background-color: #FF0000;'>
Character Profile</div>".to_string()
       }, vec![
          ("document", "ready", Box::new(|time, msg| { true })
       )],
    );

    HtmlActor::new("topright", (), |minimap| {
          "<div style='position: absolute; top: 0; right: 0; width: 200px; height: 200px; background-color: #00FF00;'>
Map Overlay</div>".to_string()
       }, vec![
          ("document", "ready", Box::new(|time, msg| { true })
       )],
    );

    HtmlActor::new("bottomleft", (), |notifications| {
          "<div style='position: absolute; bottom: 40px; left: 0; width: 600px; height: 250px; background-color: #0000FF;'>
Notifications</div>".to_string()
       }, vec![
          ("document", "ready", Box::new(|time, msg| { true })
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

    set_interval_forget(|| { JSMX_EXCHANGE.push("notifications","local",&Value::String("Halt, who goes there!".to_string())) }, 5000);
    JSMX_EXCHANGE.push("document","ready",&Value::Null);
    Ok(())
}
