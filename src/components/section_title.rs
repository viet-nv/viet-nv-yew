use css_in_rust::Style;
use yew::{html, Classes, Component, ComponentLink, Html, Properties, ShouldRender};

pub struct SectionTitle {
    style: Style,
    props: Props,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub title: String,
}

pub enum Msg {}

impl Component for SectionTitle {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let style = Style::create(
            String::from("section-title"),
            String::from(
                r#"
                &::after {
                    content: "";
                    width: 68px;
                    height: 1px;
                    display: block;
                    margin-top: 8px;
                    background: rgb(41, 41, 41);
                }
                "#,
            ),
        )
        .expect("An error occured while creating the style.");

        Self { props, style }
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
            <div class=Classes::from("tracking-widest text-base font-medium mb-6").extend(self.style.clone())>
                {self.props.title.clone()}
            </div>
        }
    }
}
