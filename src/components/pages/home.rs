use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::Route;


#[function_component(Home)]
pub fn home() -> Html {
    html!{
        <div>
        <h1>{"Home"}</h1>
        <div> 
        <Link<Route> to={Route::Hello}>{ "To Hello" }</Link<Route>>
        </div>
        </div>
    }
}