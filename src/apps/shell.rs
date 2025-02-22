use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, *};
use thaw::{ssr::SSRMountStyleProvider, ConfigProvider};

use crate::apps::account::*;
use crate::auth::*;
use crate::todo_app::*;

pub fn app_shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <SSRMountStyleProvider>
            <!DOCTYPE html>
            <html lang="en">
                <head>
                    <meta charset="utf-8"/>
                    <meta name="viewport" content="width=device-width, initial-scale=1"/>
                    <AutoReload options=options.clone() />
                    <HydrationScripts options/>
                    <Stylesheet id="leptos" href="/pkg/todo.css"/>
                    <link rel="shortcut icon" type="image/ico" href="/favicon.ico"/>
                    <MetaTags/>
                </head>
                <body>
                    <App />
                </body>
            </html>
        </SSRMountStyleProvider>
    }
}

#[component]
pub fn App() -> impl IntoView {
    let login = ServerAction::<Login>::new();
    let logout = ServerAction::<Logout>::new();
    let signup = ServerAction::<Signup>::new();

    let user = Resource::new(
        move || {
            (
                login.version().get(),
                signup.version().get(),
                logout.version().get(),
            )
        },
        move |_| get_user(),
    );
    provide_meta_context();

    view! {
        <ConfigProvider>
                <Router>
                    <FlatRoutes fallback=|| "Not found.">
                        <Route path=path!("") view=move ||
                            view! {
                                <Transition fallback=move || view! {
                                    <p>"Loading..."</p>
                                }>
                                   {move || match user.get().map(|r| r.ok().flatten().is_none()) {
                                        Some(true) => view!{<Todos is_guest=false />},
                                        _ => view!{<Todos is_guest=true />}
                                    }
                                }
                                </Transition>
                            }
                        />
                        <ProtectedRoute
                            path=path!("signup")
                            condition=move || user.get().map(|r| r.ok().flatten().is_none())
                            redirect_path=|| "/"
                            view=move || view! { <SignupPage action=signup/> }
                        />
                        <ProtectedRoute
                            path=path!("login")
                            condition=move || user.get().map(|r| r.ok().flatten().is_none())
                            redirect_path=|| "/"
                            view=move || { view! { <LoginPage action=login/> } }
                        />
                        <ProtectedRoute
                            path=path!("settings")
                            condition=move || user.get().map(|r| r.ok().flatten().is_some())
                            redirect_path=|| "/"
                            view=move || { view! { <SettingsPage logout_action=logout/> } }
                        />
                    </FlatRoutes>
                </Router>
        </ConfigProvider>
    }
}
