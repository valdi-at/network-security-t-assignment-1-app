use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

use crate::{
    components::hooks::use_theme_mode::ThemeMode, components::navbar::Navbar,
    pages::err404::Err404Page, pages::home::HomePage,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <script>
                    {r#"
                    (function () {
                    try {
                        const stored = localStorage.getItem("darkmode");
                        const system = window.matchMedia("(prefers-color-scheme: dark)").matches;
                        const isDark = stored !== null ? stored === "true" : system;

                        if (!isDark) {
                        document.documentElement.classList.add("dark");
                        }
                    } catch (_) {}
                    })();
                    "#}
                </script>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body class="bg-background text-foreground min-h-screen">
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    let theme = ThemeMode::init();

    // 👇 THIS is what you're missing
    Effect::new(move |_| {
        use web_sys::window;

        let document = window().unwrap().document().unwrap();
        let html = document.document_element().unwrap();

        if theme.get() {
            html.class_list().remove_1("dark").unwrap();
        } else {
            html.class_list().add_1("dark").unwrap();
        }
    });

    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/network-security-t-assignment-1-app.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        <Navbar/>

        // content for this welcome page
        <Router>
            <main class="flex items-center justify-center min-h-screen">
                <Routes fallback=move || Err404Page>
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}
