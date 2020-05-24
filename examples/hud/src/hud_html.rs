pub fn character_portrait() -> String {
   format!("<div style='position:absolute; left:10px; top:10px; width:80px; height:80px; background-color:#000000; z-index:2;'></div>")
}

/*
pub fn chatlog_head(name: &str, show: &str) -> String {
   format!("<div style='float: left; height: 100%; padding: 0 10px; color:#FFFFFF; border-right: 1px solid limegreen; cursor:pointer;
font-size:12px; line-height:24px; font-family:sans-serif; {}' onclick='rwa.jsmx_push(\"log\",\"set_show\",\"{}\")'>{}</div>",
      if name==show {"font-weight:bold;"} else {""},
      name,
      name)
}
pub fn chatlog_channels(log: &ChatLog) -> String {
   let mut head = String::new();
   for (n,ch) in log.channels.iter() {
      head += &chatlog_head(&n, &log.show);
   }
   format!("<div style='position:absolute; left:0; top:0; width: 100%; height:24px; border-bottom:1px solid limegreen;'>{}</div>",
      head)
}
pub fn chatlog_line(msg: &str) -> String {
   format!("<div style='color:#FFFFFF; margin: 0 10px 4px 10px; font-size:13px; font-family:sans-serif;'>{}</div>", msg)
}
pub fn chatlog_log(log: &ChatLog) -> String {
   let mut msgs = String::new();
   for (n,ch) in log.channels.iter() {
      if n==&log.show {
         for m in ch.iter() {
            msgs += &chatlog_line(m); 
         }
      }
   }
   format!("<div style='position:absolute; left:0; bottom:24px; width: 100%; height:198px; overflow: hidden;'>
<div style='position:absolute; bottom:0;'>{}</div></div>", msgs)
}
pub fn chatlog_input(log: &ChatLog) -> String {
   format!("<div style='position:absolute; left:0; bottom:0; width: 100%; height:24px; border-top:1px solid limegreen; cursor:text;
font-size:13px; font-family:sans-serif; line-height:24px; color:#FFFFFF; padding: 0 10px;'>{}</div>",
      encode_minimal(&log.input))
}
*/
