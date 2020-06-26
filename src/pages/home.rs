use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct Home {}

pub enum Msg {}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="min-h-screen">
                <h1>{"Home Page"}</h1>
                <p>{"Home Page Content"}</p>
            </div>
        }
    }
}
