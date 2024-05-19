use stylist::Style;
use stylist::style;
use yew::prelude::*;


#[derive(Properties, PartialEq)]
pub struct Props {
    pub message: String
}

pub struct StructHello {   
    pub stylesheet: Style,
}

impl StructHello {
    fn style() -> Style {
        style!( 
            r#"
             color: green;
            "#
        ).unwrap()
    }
}


impl Component for StructHello {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {          
            stylesheet: Self::style(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html!{
           <h1 class={self.stylesheet.clone()}>{&ctx.props().message}</h1>
        }
    }
    
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }
    
    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        true
    }
    
    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {}
    
    fn prepare_state(&self) -> Option<String> {
        None
    }
    
    fn destroy(&mut self, ctx: &Context<Self>) {}
    
}