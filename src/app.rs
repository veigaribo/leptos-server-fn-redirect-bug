use leptos::prelude::*;

pub fn shell(_options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[server(RedirectMe)]
pub async fn redirect_me() -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr_actix")]
    leptos_actix::redirect("/foo");

    #[cfg(feature = "ssr_axum")]
    leptos_axum::redirect("/foo");

    Ok(())
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <HomePage/>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    let action = ServerAction::<RedirectMe>::new();

    view! {
        <ActionForm action>
            <input type="submit"/>
        </ActionForm>
    }
}
