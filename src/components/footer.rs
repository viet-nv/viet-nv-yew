use log::*;
use yew::prelude::*;

pub struct Footer {}

pub enum Msg {}

impl Component for Footer {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {}
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        info!("footer rendered!");
        html! {
            <div class="">
                {"footer"}
            </div>
        }
    }
}
