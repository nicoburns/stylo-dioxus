// background: rgb(2,0,36);
// background: linear-gradient(90deg, rgba(2,0,36,1) 0%, rgba(9,9,121,1) 35%, rgba(0,212,255,1) 100%);

use blitz::Config;
use dioxus::prelude::*;

fn main() {
    let cfg = Config {
        stylesheets: vec![CSS.to_string()],
    };
    blitz::launch_cfg(app, cfg);
}

fn app(cx: Scope) -> Element {
    render! {
        div { id: "a", " Dioxus                                     \n\n\n\n\n\n\n\n\n" }
    }
}

const CSS: &str = r#"
#a {
    height:300px;
    background-color: gray;
    border: 1px solid black;
    // border-radius: 50px 20px;
    border-top-color: red;
    padding:20px;
    margin:20px;
    // border-radius: 10% 30% 50% 70%;
    border-left: 4px solid #000;
    border-top: 10px solid #ff0;
    border-right:  3px solid #F01;
    border-bottom:  9px solid #0f0;
    box-shadow: 10px 10px gray;
    // background: linear-gradient(#e66465, #9198e5);
    // background: linear-gradient(#3f87a6, #ebf8e1, #f69d3c);
    background: linear-gradient(0.25turn, #3f87a6, #ebf8e1, #f69d3c);
}
"#;
