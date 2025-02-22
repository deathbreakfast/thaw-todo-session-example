use leptos::prelude::*;
use thaw::*;

use crate::apps::components::title_bar::*;

#[component]
pub fn AppLayout(
    children: Children,
    is_guest: bool,
    title: String,
) -> impl IntoView {
    view! {
        <Layout position=LayoutPosition::Absolute>
            <LayoutHeader>
                <TitleBar is_guest title=title />
            </LayoutHeader>
            <Layout position=LayoutPosition::Absolute attr:style="top: 64px">
                {children()}
            </Layout>
        </Layout>
    }
}
