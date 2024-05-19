use std::rc::Rc;

use yew::prelude::*;
use yewdux::prelude::*;



use crate::stores::counter_store::CounterStore;


pub enum Msg {
    Store(Rc<CounterStore>)
}

pub struct DisplayCount {
    dispatch: Dispatch<CounterStore>
}

impl Component for DisplayCount {
    type Message = Msg;
    type Properties = ();
    
    fn create(ctx: &Context<Self>) -> Self {
        let on_change = ctx.link().callback( Msg::Store);  
        let dispatch = Dispatch::<CounterStore>::global().subscribe(on_change);
       
        Self{dispatch}
    }
    
    fn view(&self, _ctx: &Context<Self>) -> Html {
        let count = self.dispatch.get().count;
        html!{
            <div>
                <h2>{"Count"}</h2>
                <div>{count}</div>
            </div>
        }
    }
    
}