//use gloo::console::log;
use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::Route;



#[function_component(Hello)]
pub fn hello() -> Html {
    let navigator = use_navigator().unwrap();
    //let link_onclick = props.onclick.clone();
    let onclick = Callback::from( move |_| 
        //log!("clicked");
        navigator.push(&Route::Home)
   );
    html!{
        <div>
        <h1>{"Hello"}</h1>
        <button {onclick}>{"Go Home"}</button>
        </div>
    }
}