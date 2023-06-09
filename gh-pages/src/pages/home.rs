use crate::components::*;
use leptos::*;
use leptos_router::{use_navigate, use_query_map};
use melt_ui::*;

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    let query_map = use_query_map(cx).get();
    if let Some(path) = query_map.get("path") {
        let path = store_value(cx, path.clone());
        request_animation_frame(move || {
            let navigate = use_navigate(cx);
            _ = navigate(&path.get_value(), Default::default());
        });
    }
    view! { cx,
        <Layout position=LayoutPosition::ABSOLUTE>
            <SiteHeader />
            <Layout position=LayoutPosition::ABSOLUTE style="top: 54px; display: flex; align-items: center; justify-content: center; flex-direction: column;">
                <p>"A Leptos UI Library"</p>
                <Button on:click=move |_| {
                    let navigate = use_navigate(cx);
                    _ = navigate("/components/menu", Default::default());
                }>
                    "Read the docs"
                </Button>
            </Layout>
        </Layout>
    }
}
