//use gloo::console::log;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Prop {
    pub name: String,
    pub handle_onchange: Callback<String>,
}


#[function_component(TextInput)]
pub fn text_input(props: &Prop) -> Html {
    let handle_onchange = props.handle_onchange.clone();
    let onchange = Callback::from( move |event:Event| {
        let value = event
        .target()
        .unwrap()
        .unchecked_into::<HtmlInputElement>()
        .value();
        handle_onchange.emit(value);
    });
    html!{
        <input type="text" name={props.name.clone()} onchange={onchange} placeholder={props.name.clone()}/>
    }
}