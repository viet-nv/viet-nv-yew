mod footer;
mod header;

use footer::Footer;
use header::Header;
use yew::prelude::*;
use yew::{html, Classes, Component, ComponentLink, Html, Properties, ShouldRender};

/// Site layout.
pub struct Layout {
    props: Props,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: String,
}

pub enum Msg {}

impl Component for Layout {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <>
                <Header />
                <div class=Classes::from(self.props.class.clone())>
                    { self.props.children.render() }
                </div>
                <Footer />
            </>
        }
    }
}
