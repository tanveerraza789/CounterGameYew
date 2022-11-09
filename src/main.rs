use rand::Rng;
use yew::prelude::*;

enum Msg {
    AddOne,
    AddTwo,
    AddFour,
    AddEight,
    AddSixteen,
    AddThirtyTwo,
    AddSixtyFour,
    AddOneTwoEight,
}
struct CounterComponent {
    count: u64,
    target: u64,
    msg: String,
}

impl Component for CounterComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let mut obj = Self {
            count: 0,
            target: 0,
            msg: "Target Unmatched".to_owned(),
        };
        obj.target = rand::thread_rng().gen_range(1, 1000);
        obj
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.count += 1;
            }
            Msg::AddTwo => {
                self.count += 2;
            }
            Msg::AddFour => {
                self.count += 4;
            }
            Msg::AddEight => {
                self.count += 8;
            }
            Msg::AddSixteen => {
                self.count += 16;
            }
            Msg::AddThirtyTwo => {
                self.count += 32;
            }
            Msg::AddSixtyFour => {
                self.count += 64;
            }
            Msg::AddOneTwoEight => {
                self.count += 128;
            }
        }
        if self.target == self.count {
            self.msg = "Target Matched".to_owned();
        } else if self.target < self.count {
            self.msg = "Game Lost".to_owned();
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <div class="container">
                <p>
                    <span>{"Target = "}{self.target}</span>
                    <span>{&self.msg}</span>
                </p>
                <p>{self.count}</p>
                <button onclick={link.callback(|_| Msg::AddOne)}>
                    {"+1"}
                </button>
                <hr/>
                <button onclick={link.callback(|_| Msg::AddTwo)}>
                    {"+2"}
                </button>
                <hr/>
                <button onclick={link.callback(|_| Msg::AddFour)}>
                    {"+4"}
                </button>
                <hr/>
                <button onclick={link.callback(|_| Msg::AddEight)}>
                    {"+8"}
                </button>
                <hr/>
                <button onclick={link.callback(|_| Msg::AddSixteen)}>
                    {"+16"}
                </button>
                <hr/>
                <button onclick={link.callback(|_| Msg::AddThirtyTwo)}>
                    {"+32"}
                </button>
                <hr/>
                <button onclick={link.callback(|_| Msg::AddSixtyFour)}>
                    {"+64"}
                </button>
                <hr/>
                <button onclick={link.callback(|_| Msg::AddOneTwoEight)}>
                    {"+128"}
                </button>
            </div>
        }
    }
}
fn main() {
    yew::start_app::<CounterComponent>();
}
