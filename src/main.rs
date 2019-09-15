use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

struct Model {
    style: Style
}

enum Style {
    BlueStyle,
    RedStyle
}

enum Msg {
    SetBlue,
    SetRed,
    Rerender
}

impl Component for Model {
    // Some details omitted. Explore the examples to see more.

    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model { style: Style::BlueStyle }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetBlue => {
                self.style = Style::BlueStyle;
                true
            }
            Msg::SetRed => {
                self.style = Style::RedStyle;
                true
            }
            Msg::Rerender => true
        }
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        let style = match self.style {
            Style::BlueStyle => "background-color: blue; width: 100px; height: 100px",
            Style::RedStyle => "background-color: red; width: 100px; height: 100px"
        };

        html! {
            <>
                <div style=style></div>
                // Render your model here
                <button onclick=|_| Msg::SetBlue>{ "Set Blue" }</button>
                <button onclick=|_| Msg::SetRed>{ "Set Red" }</button>
                <button onclick=|_| Msg::Rerender>{ "Rerender" }</button>
            </>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
