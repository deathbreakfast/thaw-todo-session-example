use leptos::{
    prelude::*,
    either::Either,
    ev::MouseEvent,
};
use leptos_router::hooks::use_navigate;
use thaw::*;
use thaw_utils::mount_style;


#[component]
pub fn TitleBar(
    is_guest: bool,
    title: String,
) -> impl IntoView {
    mount_style("title-bar", include_str!("./title_bar.css"));
    let navigate_home = use_navigate();
    let navigate = use_navigate();

    let theme = Theme::use_rw_theme();
    let theme_name = Memo::new(move |_| {
        theme.with(|theme| {
            if theme.name == *"light" {
                "Dark".to_string()
            } else {
                "Light".to_string()
            }
        })
    });
    let change_theme = move |_| {
        if theme_name.get_untracked() == "Light" {
            theme.set(Theme::light());
        } else {
            theme.set(Theme::dark());
        }
    };


    view! {
        <Flex vertical=true>
            <Flex align=FlexAlign::Center justify=FlexJustify::SpaceBetween>
                <Flex 
                    align=FlexAlign::Center 
                    class="title-home"
                    on:click=move |_| {navigate_home("/", Default::default())}>
                    <Icon icon=icondata::LuCheckCircle width="36px" height="36px" />
                    <Text class="title-text" tag=TextTag::H1>{title}</Text>
                </Flex>
                <Flex>
                    <Menu position=MenuPosition::BottomEnd on_select=move |value: String| match value.as_str() {
                        "settings" => navigate("/settings", Default::default()),
                        "login" => navigate("/login", Default::default()),
                        "signup" => navigate("/signup", Default::default()),
                        "Dark" => change_theme(MouseEvent::new("click").unwrap()),
                        "Light" => change_theme(MouseEvent::new("click").unwrap()),
                        _ => {}
                    }>
                        <MenuTrigger slot>
                            <Button 
                                appearance=ButtonAppearance::Subtle 
                                icon=icondata::AiUnorderedListOutlined 
                            />
                        </MenuTrigger>
                        {move || {
                            theme.with(|theme| {
                                if theme.name == *"light" {
                                    view! {
                                        <MenuItem icon=icondata::LuMoon value=theme_name>
                                            {move || theme_name.get()}
                                        </MenuItem>
                                    }
                                } else {
                                    view! {
                                        <MenuItem icon=icondata::LuSun value=theme_name>
                                            {move || theme_name.get()}
                                        </MenuItem>
                                    }
                                }
                            })
                        }}
                        {match is_guest {
                            true => Either::Left(view! { <MenuItem icon=icondata::LuSettings value="settings">Settings</MenuItem> }),
                            false => Either::Right(view! {
                                <>
                                    <MenuItem icon=icondata::LuLogIn value="login">Login</MenuItem>
                                    <MenuItem icon=icondata::LuLogIn value="signup">Signup</MenuItem>
                                </>
                            })
                        }}
                    </Menu>
                </Flex>
            </Flex>
            <Divider />
        </Flex>
    }
}
