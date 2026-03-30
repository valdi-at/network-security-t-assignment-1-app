use crate::{
    components::ui::{button::*, card::*, theme_toggle::*},
    controller::auth::auth_check,
};
use leptos::prelude::*;

#[component]
pub fn ProfilePage() -> impl IntoView {
    let auth = Resource::new(|| (), |_| async { auth_check().await.unwrap_or(false) });

    view! {
        <Card class="max-w-lg lg:max-w-2xl">
            <CardHeader>
                <CardTitle>"Home Page"</CardTitle>
            </CardHeader>

            <CardContent>
                <CardDescription>
                    <Suspense fallback=|| view! { "Checking..." }>
                        {move || match auth.get() {
                            Some(true) => view! { "Logged in" }.into_view(),
                            Some(false) => view! { "Not logged in" }.into_view(),
                            None => view! {"uh.."}.into_view(),
                        }}
                    </Suspense>
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
