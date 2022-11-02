use yew::{function_component, html, Children, Html, Properties};

#[derive(Clone, PartialEq, Properties)]
struct RootProps {
    css: String,
    children: Children,
    use_livereload: bool,
    port: i32,
}

#[function_component]
fn Root(props: &RootProps) -> Html {
    html! {
        <html>
            <head>
                <title>{ "Teal" }</title>
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <style>{&props.css}</style>
            </head>
            <body>
                { props.children.clone() }

                if props.use_livereload {
                    <script data-port={props.port.to_string()} src="hotreload.js"></script>
                }
            </body>
        </html>
    }
}

pub async fn render<F>(component: F) -> axum::response::Html<String>
where
    F: FnOnce() -> Html + Send + 'static,
{
    let css = include_str!("tailwind.css").to_string();
    let port = std::env::var("PORT").unwrap();

    axum::response::Html(
        yew::ServerRenderer::<Root>::with_props(move || RootProps {
            children: Children::new(vec![component()]),
            css,
            use_livereload: cfg!(debug_assertions),
            port: port.parse().unwrap(),
        })
        .hydratable(false)
        .render()
        .await,
    )
}
