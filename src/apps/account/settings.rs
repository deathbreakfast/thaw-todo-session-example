use leptos::prelude::*;
use thaw::*;
use thaw_utils::mount_style;

use crate::auth::*;
use crate::apps::layout::AppLayout;


#[component]
pub fn SettingsPage(logout_action: ServerAction<Logout>) -> impl IntoView {
    view! {
        <AppLayout is_guest=true title="Todos".to_string()>
            <Settings logout_action=logout_action />
        </AppLayout>
    }
}

#[component]
pub fn Settings(logout_action: ServerAction<Logout>) -> impl IntoView {
    mount_style("account", include_str!("./account.css"));
    view! {
        <Card class="auth-card">
            <CardHeader>
                <Text class="card-header-title" tag=TextTag::H1>"Settings"</Text>
            </CardHeader>
            <Logout action=logout_action/>
        </Card>
    }
}

#[component]
fn Logout(action: ServerAction<Logout>) -> impl IntoView {
    view! {
        <Button appearance=ButtonAppearance::Secondary 
            icon=icondata::LuLogOut
            on_click=move |_| {
                action.dispatch(Logout {});
            }
        >
            "Log Out"
        </Button>
    }
}