use yew::prelude::*;


pub enum  StructCounterMessage {
    ButtonClicked,
}

pub struct StructCounter {
    pub count: u32,
}

impl Component for StructCounter {
    type Message = StructCounterMessage;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            count:0
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclink = ctx.link().callback(|_| 
            StructCounterMessage::ButtonClicked);
        html!{
            <div>
                <button onclick={onclink}>{"Click Me"}</button>
                <p>{"I have been clicked "}{self.count}{" times"}</p>
            </div>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            StructCounterMessage::ButtonClicked => {
                self.count += 1;
                true
            }
        }
    }
}