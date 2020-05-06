use crate::hud_state::*;

pub fn resource_bar(resource: (u64, u64), bbox: (u64, u64, u64, u64), color: &str) -> String {
    let (left, top, width, height) = bbox;
    let (resource_current, resource_cap) = resource;
    let inner_width = (width * resource_current) / resource_cap;
    format!("<div style='position:absolute; left:{}px; top:{}px; width:{}px; height:{}px; background-color:#444444; z-index:2; overflow:hidden;'>
<div style='position:absolute; left:4px; top:4px; width:{}px; height:{}px; background-color:#FFFFFF; color:#000000; text-align:center; z-index:3'></div>
<div style='position:absolute; left:4px; top:4px; width:{}px; height:{}px; background-color:{}; z-index:4'></div>
<div style='position:absolute; left:0px; top:6px; width:100%; height:100%; color:#000000; font-size:16px; font-family:sans-serif; text-align:center; z-index:5;'>{}/{}</div>
</div>",
       left, top, width, height,
       width-8, height-8,
       inner_width-8, height-8, color,
       resource_current, resource_cap)
}

pub fn character_portrait() -> String {
   format!("<div style='position:absolute; left:10px; top:10px; width:80px; height:80px; background-color:#000000; z-index:2;'></div>")
}

pub fn chatlog_head(name: &str, show: &str) -> String {
   format!("<div style='float: left; height: 100%; padding: 0 10px; color:#FFFFFF; border-right: 1px solid limegreen; font-size:12px; line-height:24px; font-family:sans-serif; {}'>{}</div>",
      if name==show {"font-weight:bold;"} else {""},
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
pub fn chatlog_log() -> String {
   "".to_string()
}
pub fn chatlog_input() -> String {
   format!("<div style='position:absolute; left:0; bottom:0; width: 100%; height:24px; border-top:1px solid limegreen;'>{}</div>",
   "")
}
