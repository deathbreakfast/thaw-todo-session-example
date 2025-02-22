use leptos::prelude::*;
use thaw::*;
use thaw_utils::mount_style;

use crate::auth::*;
use crate::apps::layout::AppLayout;

#[component]
pub fn SignupPage(action: ServerAction<Signup>) -> impl IntoView {
    view! {
        <AppLayout is_guest=true title="Todos".to_string()>
            <SignupCard action=action />
        </AppLayout>
    }
}


#[component]
pub fn SignupCard(action: ServerAction<Signup>) -> impl IntoView {
    mount_style("account", include_str!("./account.css"));
    let username = RwSignal::new(String::from(""));
    let password = RwSignal::new(String::from(""));
    let confirm_password = RwSignal::new(String::from(""));
    let remember_me: RwSignal<bool> = RwSignal::new(false);

    view! {
        <Card class="auth-card">
            <CardHeader>
                <Text class="card-header-title" tag=TextTag::H1>"Sign Up"</Text>
            </CardHeader>
            <Flex vertical=true>
                <Flex justify=FlexJustify::SpaceBetween>
                    <Label weight=LabelWeight::Semibold>"Username:"</Label>
                    <Input value=username />
                </Flex>
                <Flex justify=FlexJustify::SpaceBetween>
                    <Label weight=LabelWeight::Semibold>"Password:"</Label>
                    <Input input_type=InputType::Password value=password />
                </Flex>
                <Flex justify=FlexJustify::SpaceBetween>
                    <Label weight=LabelWeight::Semibold>"Confirm Password:"</Label>
                    <Input input_type=InputType::Password value=confirm_password />
                </Flex>
                <Flex justify=FlexJustify::SpaceBetween>
                    <div />
                    <Checkbox label="Remember me?" checked=remember_me />
                </Flex>
            </Flex>
            <CardFooter>
                <Button appearance=ButtonAppearance::Primary on_click=move |_| {
                    action.dispatch(Signup {
                        username: username.get(),
                        password: password.get(),
                        password_confirmation: confirm_password.get(),
                        remember: match remember_me.get() {
                            true => Some("".to_string()),
                            false => None,
                        },
                    });
                } >"Sign Up"</Button>
            </CardFooter>
        </Card>
    }
}