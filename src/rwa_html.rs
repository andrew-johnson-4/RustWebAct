use serde_json::{Value};

pub struct HtmlActor<T> {
   st: T
}
impl<T> HtmlActor<T> {
   pub fn new<R>(_nom: &str, st: T,
              _render: R,
              _mail: Vec<(&str,&str,Box<dyn FnMut(&mut T, Value) -> bool>)>
   ) -> HtmlActor<T>
     where R: FnMut(&mut T) -> String
   {
      HtmlActor { st: st }
   }
}
