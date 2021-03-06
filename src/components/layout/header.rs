use crate::app::AppRoutes;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use yew::{html, prelude::*, Bridge, Component, ComponentLink, Html, ShouldRender};
use yew_router::components::RouterAnchor;
use yew_router::prelude::*;
use yew_router::service::RouteService;

pub struct Header {
    _router: Box<dyn Bridge<RouteAgent>>,
}

pub enum Msg {
    NoOp,
}

impl Component for Header {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let callback = _link.callback(|_| Msg::NoOp); // TODO use a dispatcher instead.
        let router = RouteAgent::bridge(callback);

        let window: web_sys::Window = web_sys::window().expect("window not available");
        let closure = Closure::wrap(Box::new(move |_: web_sys::MouseScrollEvent| {
            let window: web_sys::Window = web_sys::window().expect("window not available");
            let document = web_sys::window().unwrap().document().unwrap();
            let header = document.get_element_by_id("header").unwrap();
            let scroll_y = window.scroll_y();
            let scroll_y = match scroll_y.ok() {
                Some(number) => number,
                None => 0.0,
            };
            if scroll_y > 0.0 {
                header.class_list().add_1("shadow").unwrap();
            } else {
                header.class_list().remove_1("shadow").unwrap();
            }
        }) as Box<dyn FnMut(_)>);

        window
            .add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();

        Self { _router: router }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let route_service: RouteService<()> = RouteService::new();
        let route = route_service.get_route().route;

        let current_route = AppRoutes::switch(Route::new_no_state(&route)).unwrap();

        let nav_class = "text-gray-600 hover:text-gray-900 m-4";
        let nav_class_active = "font-medium text-gray-900 m-4";

        html! {
            <div class="sticky top-0 bg-white" id="header">
                <div class="container m-auto px-4 flex justify-between items-center h-12 sm:h-16">
                    <div class="block md:hidden cursor-pointer p-3">
                        <svg class="fill-current w-5" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><title>{"Menu"}</title><path d="M0 3h20v2H0V3zm0 6h20v2H0V9zm0 6h20v2H0v-2z"/></svg>
                    </div>
                    <RouterAnchor<AppRoutes> route=AppRoutes::Home>
                        <img src="/assets/logo.png" alt="Logo" class="h-5 sm:h-6"/>
                    </RouterAnchor<AppRoutes>>

                    <div class="flex items-center">
                        <div class="hidden md:flex mr-4">
                            <RouterAnchor<AppRoutes> route=AppRoutes::Home>
                                <span class=if current_route == AppRoutes::Home {nav_class_active} else {nav_class}>
                                    { "Home" }
                                </span>
                            </RouterAnchor<AppRoutes>>
                            <RouterAnchor<AppRoutes> route=AppRoutes::Post>
                                <span class=if current_route == AppRoutes::Post {nav_class_active} else {nav_class}>
                                    { "Mini Game" }
                                </span>
                            </RouterAnchor<AppRoutes>>
                            <RouterAnchor<AppRoutes> route=AppRoutes::Post>
                                <span class=nav_class>
                                    { "CV" }
                                </span>
                            </RouterAnchor<AppRoutes>>
                            <RouterAnchor<AppRoutes> route=AppRoutes::Post>
                                <span class=nav_class>
                                    { "About" }
                                </span>
                            </RouterAnchor<AppRoutes>>
                        </div>
                        <div class="p-3 cursor-pointer">
                            <svg stroke="currentColor" fill="currentColor" stroke-width="0" viewBox="0 0 512 512" height="23px" width="23px" xmlns="http://www.w3.org/2000/svg"><path d="M443.5 420.2L336.7 312.4c20.9-26.2 33.5-59.4 33.5-95.5 0-84.5-68.5-153-153.1-153S64 132.5 64 217s68.5 153 153.1 153c36.6 0 70.1-12.8 96.5-34.2l106.1 107.1c3.2 3.4 7.6 5.1 11.9 5.1 4.1 0 8.2-1.5 11.3-4.5 6.6-6.3 6.8-16.7.6-23.3zm-226.4-83.1c-32.1 0-62.3-12.5-85-35.2-22.7-22.7-35.2-52.9-35.2-84.9 0-32.1 12.5-62.3 35.2-84.9 22.7-22.7 52.9-35.2 85-35.2s62.3 12.5 85 35.2c22.7 22.7 35.2 52.9 35.2 84.9 0 32.1-12.5 62.3-35.2 84.9-22.7 22.7-52.9 35.2-85 35.2z"></path></svg>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
