
pub fn resource_bar(resource: (u64, u64), bbox: (u64, u64, u64, u64), color: &str) -> String {
    let (left, top, width, height) = bbox;
    let (resource_current, resource_cap) = resource;
    let inner_width = (width * resource_current) / resource_cap;
    format!("<div style='position: absolute; left:{}px; top:{}px; width:{}px; height:{}px; background-color:#444444; z-index:2; overflow:hidden;'>
<div style='position: absolute; left:4px; top:4px; width:{}px; height:{}px; background-color:#FFFFFF; color:#000000; text-align:center; z-index:3'></div>
<div style='position: absolute; left:4px; top:4px; width:{}px; height:{}px; background-color:{}; z-index:4'></div>
<div style='position: absolute; left:0px; top:6px; width: 100%; height: 100%; color:#000000; font-size:16px; font-family: sans-serif; text-align:center; z-index:5;'>{}/{}</div>
</div>",
       left, top, width, height,
       width-8, height-8,
       inner_width-8, height-8, color,
       resource_current, resource_cap)
}
