use yew::prelude::*;

pub struct ListItemAction {
    props: Props,
}

pub struct Msg {}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
}

impl Component for ListItemAction {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let mut classes = Classes::from("v-list-item__action");
        html! {
            <div class=classes>
                { self.props.children.render() }
            </div>
        }
    }
}
