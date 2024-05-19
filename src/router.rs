
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::pages::home::Home;
use crate::components::pages::hello::Hello;


#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/hello")]
    Hello,
}


pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {<Home /> },
        Route::Hello => html! { <Hello /> },        
    }
}