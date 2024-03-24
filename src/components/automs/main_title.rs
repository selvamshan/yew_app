use stylist::{yew::styled_component, Style};
use yew::prelude::*;

const STYLE_FILE: &str = include_str!("main_title.css");

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: String,
    pub color: Color,
}

#[derive(PartialEq)]
pub enum Color {
    Normal,
    Ok,
    Error,
}

impl Color {
    pub fn to_string(&self) -> String {
        match self {
            Color::Normal => "normal".to_owned(),
            Color::Ok => "ok".to_owned(),
            Color::Error => "error".to_owned()
        }
    }
}

#[styled_component(MainTitle)]
pub fn main_title(props: &Props) -> Html{
    let stylesheet = Style::new(STYLE_FILE).unwrap();
    html!(
        <div class={stylesheet}>
        <h1 class={props.color.to_string()}>{"Hello World! "}{&props.title}</h1>
        </div>
    )
}