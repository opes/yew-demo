use yew::prelude::*;

enum Msg {
    Square,
    AddOne,
    Reset,
    SubtractOne,
}

struct Model {
    link: ComponentLink<Self>,
    value: i64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, value: 0 }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                true
            }

            Msg::Reset => {
                self.value = 0;
                true
            }

            Msg::SubtractOne => {
                self.value -= 1;
                true
            }

            Msg::Square => {
                self.value = self.value * self.value;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <button onclick=self.link.callback(|_| Msg::AddOne)>{ "+1" }</button>
                <button onclick=self.link.callback(|_| Msg::Reset)>{ "Reset" }</button>
                <button onclick=self.link.callback(|_| Msg::SubtractOne)>{ "-1" }</button>
                <button onclick=self.link.callback(|_| Msg::Square)>{ "**" }</button>
                <p>{ self.value }</p>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
