import("../pkg/index.js").then((mod) => {
   window.rwa = mod;
}).catch(console.error);

document.onkeydown = function(evt) {
    evt = evt || window.event;
    rwa.jsmx_push("document","keydown",{
       keyCode: String.fromCharCode(evt.keyCode).toLowerCase()
    });
};
