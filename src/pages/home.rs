use log::*;
use yew::prelude::*;

pub struct Home {
    link: ComponentLink<Self>,
    value: i64,
}

pub enum Msg {
    AddOne,
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, value: 0 }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => self.value += 1,
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        info!("home rendered!");
        html! {
            <div class="border ml-5">
                    <button onclick=self.link.callback(|_| Msg::AddOne)>{ "+1" }</button>
                    <p>{ self.value }</p>
            </div>
        }
    }
}
