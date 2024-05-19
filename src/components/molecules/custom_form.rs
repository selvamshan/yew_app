use std::ops::Deref;

//use gloo::console::log;
use yew::prelude::*;
//use yew_router::prelude::*;
use crate::components::atoms::text_input::TextInput;
use crate::components::atoms::custom_button::CustomButton;
use crate::User;

#[derive(Default, Clone)]
pub struct Data {
    pub username: String,
    pub favorite_language: String,
}


#[derive(Properties, PartialEq)]
pub struct Props {
    pub onsubmit: Callback<Data>
}

#[function_component(CustomForm)]
pub fn custom_form(props: &Props) -> Html {
    let state = use_state(|| Data::default());  
    let user_context = use_context::<User>();
    let cloned_state = state.clone();
    let username_changed = Callback::from(move |username:String|{
       let mut data = cloned_state.deref().clone();
       data.username = username;
       cloned_state.set(data);
    });
    let cloned_state = state.clone();
    let language_changed = Callback::from(move |favorite_language:String|{
        let mut data = cloned_state.deref().clone();
        data.favorite_language = favorite_language;
        cloned_state.set(data);
    });

    let form_submit = props.onsubmit.clone();
    let cloned_state = state.clone();
    let onsubmit = Callback::from( move|event: SubmitEvent| {
        event.prevent_default();
        let  data = cloned_state.deref().clone();
        form_submit.emit(data);
    });

    html!{
        <form onsubmit={onsubmit}>
            <TextInput name="username" handle_onchange={username_changed}/>
            <TextInput name="favorite_language" handle_onchange={language_changed}/>
            <CustomButton label="Submit" /> 
            <p>{"Username: "}{user_context.clone().unwrap_or_default().username}</p>
            <p>{"Favorite Languate: "}{user_context.unwrap_or_default().favorite_language}</p>            
        </form>
    }
}