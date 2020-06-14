use crate::components::footer::Footer;
use crate::components::header::Header;
use crate::pages::home::Home;
use log::*;
use yew::{prelude::*, virtual_dom::VNode};
use yew_router::switch::{AllowMissing, Permissive};
use yew_router::{route::Route, router::Router, service::RouteService, Switch};

pub struct App {}

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/page-not-found"]
    PageNotFound(Permissive<String>),
    #[to = "/"]
    Index,
}

pub enum Msg {}

impl Component for App {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {}
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> VNode {
        info!("rendered!");
        html! {
            <>
                <Header />
                <div>
                    <Router<AppRoute>
                        render = Router::render(|switch: AppRoute| {
                            match switch {
                                AppRoute::Index => {
                                    info!("hehe {:?}", switch);
                                    return html!{<Home />};
                                },
                                AppRoute::PageNotFound(Permissive(None)) => html!{"Page not found"},
                                AppRoute::PageNotFound(Permissive(Some(missed_route))) => html!{format!("Page '{}' not found", missed_route)}
                            }
                        })
                        redirect = Router::redirect(|route: Route| {
                            AppRoute::PageNotFound(Permissive(Some(route.route)))
                        })
                    />
                </div>
                <Footer />
            </>
        }
    }
}
