use leptos::{component, use_context, view, IntoView, SignalGet};
use leptos_meta::{provide_meta_context, Body, Html, Link, Meta, Stylesheet, Title};
use leptos_router::{
    use_location, Router, RouterIntegrationContext, Routes, StaticParamsMap, StaticRoute,
};

use crate::{
    components::{Footer, Header},
    pages::{Aprende, Communidad, Contributors, Index},
};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    let (assets_folder, bg_in_dark_mode) = if cfg!(debug_assertions) {
        ("./assets", "dark:bg-kaku-dev")
    } else {
        (".", "dark:bg-kaku")
    };

    let preview = if use_context::<RouterIntegrationContext>()
        .is_some_and(|ctx| ctx.0.location().get().value.contains("/aprende"))
    {
        "aprende_preview.webp"
    } else {
        "rustlanges_preview.webp"
    };

    view! {
        <Router>
            <Html lang="es"/>
            <Meta charset="utf-8"/>
            <Meta name="viewport" content="width=device-width, initial-scale=1"/>
            <Stylesheet id="fonts" href=format!("{}/fonts.css", assets_folder)/>
            <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>
            <Title text="Rust Lang en Español"/>
            <Meta
                name="description"
                content="Únete a nuestra comunidad de Rust en español. Promovemos el aprendizaje y la difusión de Rust, compartiendo conocimientos y proyectos emocionantes."
            />

            <Meta name="og:site_name" content="Rust Lang en Español"/>
            <Meta name="og:title" content="Bienvenidos a Rust Lang en Español"/>
            <Meta
                name="og:description"
                content="Únete a nuestra comunidad de Rust en español. Promovemos el aprendizaje y la difusión de Rust, compartiendo conocimientos y proyectos emocionantes."
            />
            <Meta name="og:url" content="https://rustlang-es.org"/>
            <Meta name="og:image" content=format!("https://rustlang-es.org/{preview}")/>
            <Meta name="twitter:image" content=format!("https://rustlang-es.org/{preview}")/>
            <Meta name="twitter:card" content="summary_large_image"/>
            <Meta name="twitter:site" content="@rustlang"/>
            <Meta
                name="google-site-verification"
                content="OntIe2SKuQalaapGvxdded9tU4G2p57h0A6e0Rkoni0"
            />

            {
                let location = use_location().pathname;
                view! {
                    <Link
                        rel="canonical"
                        href=format!("https://www.rustlang-es.org{}", location.get().as_str())
                    />
                }
            }

            <Body class=format!(
                "bg-orange-200 dark:bg-[#131313]/90 bg-center bg-fixed {} dark:bri dark:bg-cover dark:bg-blend-darken dark:backdrop-blur-xl overflow-x-hidden dark:text-[#e2cea9]",
                bg_in_dark_mode,
            )/>
            <Header/>
            <main>
                <Routes>
                    <StaticRoute
                        path="/"
                        view=Index
                        static_params=move || Box::pin(async move { StaticParamsMap::default() })
                    />
                    <StaticRoute
                        path="/comunidad"
                        view=Communidad
                        static_params=move || Box::pin(async move { StaticParamsMap::default() })
                    />
                    <StaticRoute
                        path="/colaboradores"
                        view=Contributors
                        static_params=move || Box::pin(async move { StaticParamsMap::default() })
                    />
                    <StaticRoute
                        path="/aprende"
                        view=Aprende
                        static_params=move || Box::pin(async move { StaticParamsMap::default() })
                    />
                    <StaticRoute
                        path="404"
                        view=Index
                        static_params=move || Box::pin(async move { StaticParamsMap::default() })
                    />
                </Routes>
            </main>
            <Footer/>
        </Router>
    }
}
