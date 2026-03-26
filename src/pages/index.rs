use leptos::prelude::*;

use crate::components::hero::Hero;

#[component]
pub fn IndexPage() -> impl IntoView {
    view! {
        <Hero src="https://images.unsplash.com/photo-1549880338-65ddcdfd017b?q=80&w=1920&auto=format&fit=crop">
        <div class="space-y-6">
            <h1 class="text-6xl md:text-7xl lg:text-8xl font-extrabold tracking-tight">
                "Welcome"
            </h1>
            <p class="text-xl md:text-2xl opacity-90">
                "OpenSMH HomePage."
            </p>
        </div>
        </Hero>
    }
}
