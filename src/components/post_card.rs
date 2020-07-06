use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

pub struct PostCard {
    props: Props,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub title: String,
    pub image: String,
    pub tags: Vec<String>,
}

pub enum Msg {}

impl Component for PostCard {
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
            <div>
                {self.props.title.clone()}
            </div>
        }
    }
}
