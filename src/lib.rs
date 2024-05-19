use std::ops::Deref;

use gloo::console::log;
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use stylist::Style;
use stylist::yew::styled_component;
use yew::ContextProvider;
use yew_router::prelude::*;

mod components;
use components::atoms::main_title::{MainTitle, Color};
use components::molecules::custom_form::CustomForm;
use components::molecules::custom_form::Data;
// use components::atoms::struct_hello::StructHello;
// use components::molecules::struct_counter::StructCounter;
mod router;
use router::{Route, switch};

mod stores;

#[derive(Serialize, Deserialize)]
struct MyObject {
    username: String,
    favorite_language: String
}

#[derive(Clone, PartialEq, Default)]
pub struct User {
    pub username: String,
    pub favorite_language: String
}


const STYLE_FILE: &str = include_str!("main.css");

//#[function_component]
#[styled_component]
pub fn App() -> Html {
    let stylesheet = Style::new(STYLE_FILE).unwrap();    
    let user_state = use_state(|| User::default());
    let main_title_load = Callback::from(|msg:String| log!(msg));
    let first_load = use_state(|| true);

    use_effect( move || {

        if *first_load {
            first_load.set(false);
        }
        || {}
    });

    let custom_form_submit = {
        let user_state = user_state.clone();
        Callback::from(move |data:Data| {
            let mut user = user_state.deref().clone();
            user.username = data.username;
            user.favorite_language =  data.favorite_language;
            user_state.set(user);
        })
    };

    // html!{
    //     <div>
    //     <StructHello message={"Hello from lib.rs".to_string()}/>
    //     <StructCounter />
    //     </div>
    // }

    html!{
        <ContextProvider<User> context={user_state.deref().clone()}>
        <div class={stylesheet}>
        <MainTitle title="Hi there" color={Color::Ok} on_load={main_title_load}/>
        <CustomForm onsubmit={custom_form_submit} />   
        <BrowserRouter>
        <Switch<Route> render={switch} />
        </BrowserRouter>
        </div>
        </ContextProvider<User>>
    }
}

