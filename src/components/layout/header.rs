use crate::app::AppRoutes;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew_router::components::RouterAnchor;

pub struct Header {}

pub enum Msg {}

impl Component for Header {
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
            <div>
                {"header"}
                <RouterAnchor<AppRoutes> route=AppRoutes::Home>
                    { "home" }
                </RouterAnchor<AppRoutes>>

                <RouterAnchor<AppRoutes> route=AppRoutes::Post>
                    { "post" }
                </RouterAnchor<AppRoutes>>
            </div>
        }
    }
}
