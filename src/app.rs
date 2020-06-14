use crate::components::Layout;
use crate::pages::home::Home;
use crate::pages::post::Post;

use log::*;
use yew::prelude::{html, Component, ComponentLink, Html, ShouldRender};
use yew_router::switch::Permissive;
use yew_router::Switch;
use yew_router::{route::Route, router::Router as YewRouter};

pub struct App {}

pub enum Msg {}

/// App routes
#[derive(Switch, Debug, Clone)]
pub enum AppRoutes {
    #[to = "/#/post"]
    Post,
    #[to = "/#/404"]
    NotFound(Permissive<String>),
    #[to = "/"]
    Home,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
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
            <Layout>
                <YewRouter<AppRoutes>
                    render=YewRouter::render(|switch: AppRoutes| {
                        info!{"router: {:?}", switch};
                        match switch {
                            AppRoutes::Post => html!{<Post />},
                            AppRoutes::Home => html!{<Home />},
                            AppRoutes::NotFound(Permissive(None)) => html!{"Page not found"},
                            AppRoutes::NotFound(Permissive(Some(missed_route))) => html!{format!("Page '{}' not found", missed_route)}
                        }
                    })
                    redirect = YewRouter::redirect(|route: Route| {
                        AppRoutes::NotFound(Permissive(Some(route.route)))
                    })
                />
            </Layout>
        }
    }
}
