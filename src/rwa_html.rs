use serde_json::{Value,Map,json};
use std::sync::{Arc,Mutex};
use std::borrow::{Borrow,BorrowMut};
use jsmx::{JSMX_EXCHANGE};
use rdxl::rdxl;

pub fn json_merge(l: &Value, r: &Value) -> Value {
   let mut m = Map::new();
   for (k,v) in l.as_object().expect("json_merge expected l as Object").iter() {
      m.insert(k.to_string(), v.clone());
   }
   for (k,v) in r.as_object().expect("json_merge expected r as Object").iter() {
      m.insert(k.to_string(), v.clone());
   }
   Value::Object(m)
}

pub trait ToStyle {
   fn to_style(&self) -> String;
}
impl ToStyle for Value {
   fn to_style(&self) -> String {
      let mut s = String::new();
      for (k,v) in self.as_object().expect("ToStyle expected self as Object").iter() {
         let k = k.as_str();
         let v = v.as_str().expect("ToStyle expected val as String");
         s.push_str(&format!("{}:{};", k, v));
      }
      s
   }
}

pub fn progress_bar(js: &Value) -> String {
   let (per,cent) = {
      if js.is_object() {
         let ar = js["progress"].as_array().expect("progress_bar expected array");
         (ar[0].as_f64().unwrap_or(0.0), ar[1].as_f64().unwrap_or(1.0))
      } else {
         let ar = js.as_array().expect("progress_bar expected array");
         (ar[0].as_f64().unwrap_or(0.0), ar[1].as_f64().unwrap_or(1.0))
      }
   };
   let style = json_merge(&json!({ "box-sizing":"border-box", "width":"100%", "padding":"3px", "background-color":"#555555" }), js.get("style").unwrap_or(&json!({})));
   let style_bar = json_merge(&json!({ "box-sizing":"border-box", "position":"relative", "width":"100%", "padding":"3px", "text-align":"center", "font-size":"14px",
      "background-color":"#CCCCCC" }), js.get("bar_style").unwrap_or(&json!({})));
   let style_progress = json_merge(&json!({ "position":"absolute", "top":"0", "left":"0", "height":"100%",
      "width":format!("{}%", 100.0*per/cent), "background-color":"#FFFFFF" }), js.get("progress_style").unwrap_or(&json!({})));
   let style_text = json_merge(&json!({ "position":"absolute", "top":"0", "left":"0", "padding":"3px", "width":"100%",
      }), js.get("text_style").unwrap_or(&json!({})));
   rdxl!(<div style=[[style]]>
     <div style=[[style_bar]]>
       <div style=[[style_progress]]></div>
       <div style=[[style_text]]>{{per}} / {{cent}}</div>
       "&nbsp;"
     </div>
   </div>)
}

pub struct HtmlActor<T> {
   st: Arc<Mutex<T>>,
}
impl<T> HtmlActor<T>
   where T: 'static + Send + Sync {
   pub fn new<R>(nom: &'static str, st: T,
              render: R,
              mail: Vec<(&'static str,&'static str,Box<dyn Send + Sync + 'static + FnMut(&mut T, &Value) -> bool>)>
   ) -> HtmlActor<T>
     where R: 'static + Send + Sync + Fn(&T) -> String
   {
      let st = Arc::new(Mutex::new(st));
      let ha = HtmlActor { st: st.clone() };
      let render = Arc::new(render);

      for (r1,r2,mut cb) in mail.into_iter() {
         let st = st.clone();
         let render = render.clone();
         JSMX_EXCHANGE.listen(r1, r2, move |msg| {
            let mut st = st.lock().unwrap();
            let need_update = cb(st.borrow_mut(), msg);
            if need_update {
               let dhtml = render(st.borrow());
               let window = web_sys::window().expect("should have a window in this context");
               let document = window.document().expect("window should have a document");
               document
                  .get_element_by_id(nom)
                  .expect(&format!("should have {} on the page", nom))
                  .set_inner_html(&dhtml);
            }
         });
      }

      ha
   }
}
