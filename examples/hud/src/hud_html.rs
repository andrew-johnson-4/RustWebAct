
pub fn resource_bar(resource: (u64, u64), bbox: (u64, u64, u64, u64), color: &str) -> String {
    let (left, top, width, height) = bbox;
    format!("<div style='position: absolute; left:{}px; top:{}px; width:{}px; height:{}px; background-color:#444444; z-index:2; overflow:hidden;'>
<div style='position: absolute; left:1px; top:1px; width:{}px; height:{}px; background-color:{}; z-index:3'></div>
</div>",
       left, top, width, height,
       width-1, height-1, color)
}
