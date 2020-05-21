use serde_json::{Value,json};
use std::sync::{Arc,Mutex};
use std::borrow::{Borrow,BorrowMut};
use jsmx::{JSMX_EXCHANGE};
use rdxl::rdxl;

pub fn json_merge(_l: &Value, _r: &Value) -> Value {
   Value::Null
}

pub fn progress_bar(js: &Value) -> String {
   //{ style={...}, progress:Number }
   let _style = json_merge(&json!({}), js);
   rdxl!(<div>
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
