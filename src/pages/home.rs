use crate::components::{Profile, SectionTitle};
use log::*;
use yew::format::Json;
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
        let data: String =
            r#"{posts: [{ title: "Hey! What the fuck is going on", "description": "desc" }]}"#
                .to_string();
        let dump = Json(&data);
        info!("{:?}", dump);
        html! {
            <div class="min-h-screen">
                <Profile />
                <div class="mb-6">
                    <SectionTitle title="Thư giãn cùng Soleil" />
                    <div class="p-8 text-center">{"Comming Soon..."}</div>
                </div>

                <div class="mb-6">
                    <SectionTitle title="Chém gió cùng Soleil" />
                    <div class="p-8 text-center">{"Comming Soon..."}</div>
                </div>
            </div>
        }
    }
}
