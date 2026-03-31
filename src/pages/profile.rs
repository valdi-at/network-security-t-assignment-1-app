use crate::{
    components::ui::{button::*, card::*, theme_toggle::*},
    controller::auth::auth_get_username,
};
use leptos::prelude::*;

#[component]
pub fn ProfilePage() -> impl IntoView {
    let auth = Resource::new(
        || (),
        |_| async { auth_get_username().await.unwrap_or(Some("".to_owned())) },
    );

    view! {
        <Card class="max-w-lg lg:max-w-2xl">
            <CardHeader>
                <CardTitle>"Profile"</CardTitle>
            </CardHeader>

            <CardContent>
                <CardDescription>
                    <Suspense fallback=|| view! { "Checking..." }>
                        {move || match auth.get() {
                            Some(Some(username)) => view! {
                                <h3>{username}</h3>
                            }.into_any(),
                            Some(None) => view! {
                                <h1>"Not logged in"</h1>
                             }.into_any(),
                            None => view! {
                                <h1>"Uhh..."</h1>
                            }.into_any(),
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
