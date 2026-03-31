use leptos::prelude::*;

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <nav class="nav" id="nav">
            <div class="nav-inner">
                <div class="links">
                    <a href="/">Index</a>
                    <a href="/home">Home</a>
                    <a href="/profile">Profile</a>
                </div>
            </div>
        </nav>
        <script>
            {r#"
                const nav = document.getElementById('nav');
                let pinned = false;
                function onScroll(){
                    const shouldPin = window.scrollY > 0;
                    if(shouldPin !== pinned){
                        pinned = shouldPin;
                        nav.classList.toggle('pinned', pinned);
                    }
                }
                document.addEventListener('scroll', onScroll, { passive: true });
            "#}
        </script>
    }
}
