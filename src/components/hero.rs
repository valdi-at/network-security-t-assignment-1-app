use leptos::prelude::*;

#[component]
pub fn Hero(#[prop(into, optional)] src: String, children: Children) -> impl IntoView {
    view! {
        <div class="hero">
            <img
                id="hero"
                class="hero-image"
                src=src
                alt="Hero image"
            />

            <div class="hero-overlay">
                {children()}
            </div>
        </div>
    }
}
