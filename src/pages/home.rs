use leptos::prelude::*;

use crate::components::ui::{button::*, card::*, theme_toggle::*};

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <Card class="max-w-lg lg:max-w-2xl">
            <CardHeader>
                <CardTitle>"Home Page"</CardTitle>
            </CardHeader>

            <CardContent>
                <CardDescription>
                    "This is just a dummy home page, anyone should be able to view this page. Click the login button to login."
                    <br/>
                    <br/>
                    <ThemeToggle/>
                </CardDescription>
            </CardContent>

            <CardFooter class="justify-end">
                <Button href="/login">"Login"</Button>
            </CardFooter>
        </Card>
    }
}
