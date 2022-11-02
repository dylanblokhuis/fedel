use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
}

pub struct AdminLayout;

impl Component for AdminLayout {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
          <main class="h-screen flex overflow-hidden">
            <aside class="bg-slate-900 w-64 h-full overflow-auto flex-none">
            </aside>
            <div class="overflow-auto px-6 py-4 w-full">
              { for ctx.props().children.iter() }
            </div>
          </main>
        }
    }
}
