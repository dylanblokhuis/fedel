use crate::components::admin_layout::AdminLayout;
use axum::{response::IntoResponse, Form};
use axum_macros::debug_handler;
use serde::Deserialize;
use yew::prelude::*;

use crate::ssr::render;

pub async fn get_handler() -> impl IntoResponse {
    render(|| component("Hi from the get_handler".to_string())).await
}

#[derive(Deserialize)]
pub struct FormData {
    name: String,
}

#[debug_handler]
pub async fn post_handler(form: Form<FormData>) -> impl IntoResponse {
    let message = format!("Hi from the post_handler {}", form.name);
    render(|| component(message)).await
}

fn component(message: String) -> Html {
    html! {
      <AdminLayout>
        <h1 class="font-bold text-xl mb-4">{ message }</h1>
        <form class="flex flex-col items-start" method="post">
            <input class="border border-solid mb-4" name="name" type="text" />
            <button class="bg-blue-600 font-medium text-lg px-4 py-2 text-white">{ "Submit" }</button>
        </form>
      </AdminLayout>
    }
}
