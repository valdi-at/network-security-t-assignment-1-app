use leptos::prelude::*;

use crate::components::ui::{button::*, card::*};

#[component]
pub fn Err404Page() -> impl IntoView {
    let go_back = move |_| {
        window().history().unwrap().back().unwrap();
    };

    view! {
        <div class="text-center max-w-md animate-fade-up motion-reduce:animate-none">
            <h1 class="text-7xl font-extrabold text-primary">404</h1>

            <h2 class="mt-4 text-2xl font-semibold">
                "Page Not Found"
            </h2>

            <p class="mt-2 text-muted-foreground">
                "The page you’re looking for doesn’t exist or has been moved."
            </p>

            <div class="mt-6 flex flex-col sm:flex-row gap-3 justify-center">
                <Button href="/" size=ButtonSize::Lg>
                    "Go Home"
                </Button>
                <Button on:click=go_back variant=ButtonVariant::Outline size=ButtonSize::Lg>
                    "Go Back"
                </Button>
            </div>

            <Card class="mt-10 max-w-lg lg:max-w-2xl">
                <CardContent>
                    <CardDescription>
                        "Error code: "<span class="font-mono">"404_NOT_FOUND"</span>
                    </CardDescription>
                </CardContent>
            </Card>
        </div>
    }
}
