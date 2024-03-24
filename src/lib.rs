use gloo::console::log;
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use stylist::Style;
use stylist::yew::styled_component;

mod components;
use components::automs::main_title::{MainTitle, Color};

#[derive(Serialize, Deserialize)]
struct MyObject {
    username: String,
    favorite_language: String
}

const STYLE_FILE: &str = include_str!("main.css");

//#[function_component]
#[styled_component]
pub fn App() -> Html {
    let stylesheet = Style::new(STYLE_FILE).unwrap();
    let name = "Selva";
    let my_object = MyObject{
        username: name.to_owned(),
        favorite_language: "Rust".to_owned(),
    };

    log!("My name is", name);
    log!(serde_json::to_string_pretty(&my_object).unwrap());
    let class = "my_title";
    let message:Option<&str> = None;

    let tasks = vec!["record video", "grocery shopping", "pet Xilbe"];

    html!{
        <>
        <div class={stylesheet}>
        <MainTitle title="Hi there" color={Color::Ok}/>
        if class == "my_titles" {
            <p>{"Hi there"}</p>
        } else {
            <p> {"I'm not a title"}</p>
        }

        if let Some(msg) = message {
            <p>{msg}</p>
        } else {
            <p>{"No message to see today"}</p>
        }
        <ul>
            {list_to_html(tasks)}
        </ul>
        </div>
        </>
    }
}

fn list_to_html(list: Vec<&str>) -> Vec<Html> 

{
    list.iter().map(|&item| html!{<li>{item}</li>}).collect()
}