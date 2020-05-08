use serde_json::{Value};
use std::sync::{Arc,Mutex};
use std::borrow::{Borrow,BorrowMut};
use jsmx::{JSMX_EXCHANGE};
use std::collections::HashMap;

pub struct HtmlDom {
   tag: String,
   style: HashMap<String,String>
}
impl HtmlDom {
   pub fn render(&self) -> String {
      format!("<{}></{}>", self.tag, self.tag)
   }
   pub fn z<'a>(&'a mut self) -> &'a mut HtmlDom {
      self.style.insert("z-index".to_string(),"auto".to_string());
      self
   }
   pub fn z1<'a>(&'a mut self) -> &'a mut HtmlDom {
      self.style.insert("z-index".to_string(),"1".to_string());
      self
   }
   pub fn z2<'a>(&'a mut self) -> &'a mut HtmlDom {
      self.style.insert("z-index".to_string(),"2".to_string());
      self
   }
   pub fn z3<'a>(&'a mut self) -> &'a mut HtmlDom {
      self.style.insert("z-index".to_string(),"3".to_string());
      self
   }
   pub fn z4<'a>(&'a mut self) -> &'a mut HtmlDom {
      self.style.insert("z-index".to_string(),"4".to_string());
      self
   }
   pub fn z5<'a>(&'a mut self) -> &'a mut HtmlDom {
      self.style.insert("z-index".to_string(),"5".to_string());
      self
   }
   pub fn z6<'a>(&'a mut self) -> &'a mut HtmlDom {
      self.style.insert("z-index".to_string(),"6".to_string());
      self
   }
   pub fn z7<'a>(&'a mut self) -> &'a mut HtmlDom {
      self.style.insert("z-index".to_string(),"7".to_string());
      self
   }
   pub fn z8<'a>(&'a mut self) -> &'a mut HtmlDom {
      self.style.insert("z-index".to_string(),"8".to_string());
      self
   }
   pub fn z9<'a>(&'a mut self) -> &'a mut HtmlDom {
      self.style.insert("z-index".to_string(),"9".to_string());
      self
   }
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
