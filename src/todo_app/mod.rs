
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use thaw::*;

use crate::{auth::*, error_template::ErrorTemplate, apps::layout::AppLayout};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Todo {
    id: u32,
    user: Option<User>,
    title: String,
    created_at: String,
    completed: bool,
}

#[cfg(feature = "ssr")]
pub mod ssr {
    use super::Todo;
    use crate::auth::{ssr::AuthSession, User};
    use leptos::prelude::*;
    use sqlx::SqlitePool;

    pub fn pool() -> Result<SqlitePool, ServerFnError> {
        use_context::<SqlitePool>()
            .ok_or_else(|| ServerFnError::ServerError("Pool missing.".into()))
    }

    pub fn auth() -> Result<AuthSession, ServerFnError> {
        use_context::<AuthSession>().ok_or_else(|| {
            ServerFnError::ServerError("Auth session missing.".into())
        })
    }

    #[derive(sqlx::FromRow, Clone)]
    pub struct SqlTodo {
        id: u32,
        user_id: i64,
        title: String,
        created_at: String,
        completed: bool,
    }

    impl SqlTodo {
        pub async fn into_todo(self, pool: &SqlitePool) -> Todo {
            Todo {
                id: self.id,
                user: User::get(self.user_id, pool).await,
                title: self.title,
                created_at: self.created_at,
                completed: self.completed,
            }
        }
    }
}

#[server(GetTodos, "/api")]
pub async fn get_todos() -> Result<Vec<Todo>, ServerFnError> {
    use self::ssr::{pool, SqlTodo};
    use futures::future::join_all;

    let pool = pool()?;

    Ok(join_all(
        sqlx::query_as::<_, SqlTodo>("SELECT * FROM todos")
            .fetch_all(&pool)
            .await?
            .iter()
            .map(|todo: &SqlTodo| todo.clone().into_todo(&pool)),
    )
    .await)
}

#[server(AddTodo, "/api")]
pub async fn add_todo(title: String) -> Result<(), ServerFnError> {
    use self::ssr::*;

    let user = get_user().await?;
    let pool = pool()?;

    let id = match user {
        Some(user) => user.id,
        None => -1,
    };

    // fake API delay
    std::thread::sleep(std::time::Duration::from_millis(1250));

    Ok(sqlx::query(
        "INSERT INTO todos (title, user_id, completed) VALUES (?, ?, false)",
    )
    .bind(title)
    .bind(id)
    .execute(&pool)
    .await
    .map(|_| ())?)
}

// The struct name and path prefix arguments are optional.
#[server]
pub async fn delete_todo(id: u16) -> Result<(), ServerFnError> {
    use self::ssr::*;

    let pool = pool()?;

    Ok(sqlx::query("DELETE FROM todos WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await
        .map(|_| ())?)
}

#[component]
pub fn Todos(is_guest: bool) -> impl IntoView {
    let add_todo = ServerMultiAction::<AddTodo>::new();
    let delete_todo = ServerAction::<DeleteTodo>::new();
    let submissions = add_todo.submissions();

    // list of todos is loaded from the server in reaction to changes
    let todos = Resource::new(
        move || (add_todo.version().get(), delete_todo.version().get()),
        move |_| get_todos(),
    );

    let title = RwSignal::new(String::from(""));

    view! {
        <AppLayout is_guest=is_guest title="Todos".to_owned()>
            <Flex vertical=true>
                <Transition fallback=move || view! { <p>"Loading..."</p> }>
                    <Flex justify=FlexJustify::Center gap=FlexGap::Small>
                        <Text>"Add a Todo"</Text>
                        <Input value=title />
                        <Button appearance=ButtonAppearance::Primary on_click=move |_| {
                            add_todo.dispatch(AddTodo { title: title.get() });
                            title.set(String::from(""));
                        }>"Add"</Button>
                    </Flex>
                </Transition>
                <Transition fallback=move || view! { <p>"Loading..."</p> }>
                    <ErrorBoundary fallback=|errors| {
                        view! { <ErrorTemplate errors=errors/> }
                    }>
                        <Card>
                            <Table>
                                <TableHeader>
                                    <TableRow>
                                        <TableHeaderCell resizable=true min_width=100.0>"Title"</TableHeaderCell>
                                        <TableHeaderCell resizable=true max_width=100.0>"Creator"</TableHeaderCell>
                                        <TableHeaderCell max_width=100.0>"Close"</TableHeaderCell>
                                    </TableRow>
                                </TableHeader>
                                <TableBody>
                                    {move || {
                                        todos.get().map(move |todos| match todos {
                                            Err(e) => {
                                                view! {
                                                    <pre class="error">"Server Error: " {e.to_string()}</pre>
                                                }
                                                    .into_any()
                                            },
                                            Ok(todos) => {
                                                if todos.is_empty() {
                                                    view! { <p>"No tasks were found."</p> }.into_any()
                                                } else {
                                                    todos
                                                        .into_iter()
                                                        .map(move |todo| {
                                                            view! {
                                                                <TableRow>
                                                                    <TableCell>
                                                                        <TableCellLayout truncate=true>
                                                                            {todo.title}
                                                                        </TableCellLayout>
                                                                    </TableCell>
                                                                    <TableCell>
                                                                        <TableCellLayout truncate=true>
                                                                            {todo.user.unwrap_or_default().username}
                                                                        </TableCellLayout>
                                                                    </TableCell>
                                                                    <TableCell>
                                                                        <Button 
                                                                            on_click=move |_| {
                                                                                delete_todo.dispatch(DeleteTodo { id: todo.id as u16 });
                                                                            } 
                                                                            icon=icondata::AiCloseCircleOutlined
                                                                        />
                                                                    </TableCell>
                                                                </TableRow>
                                                            }
                                                        })
                                                        .collect_view()
                                                        .into_any()
                                                }
                                            }
                                        })
                                    }}
                                    {move || {
                                        submissions
                                            .get()
                                            .into_iter()
                                            .filter(|submission| submission.pending().get())
                                            .map(|submission| {
                                                view! {
                                                    <TableRow>
                                                        <TableCell>
                                                            <TableCellLayout truncate=true>
                                                                {submission.input().get().map(|data| data.title)}
                                                            </TableCellLayout>
                                                        </TableCell>
                                                        <TableCell>
                                                            <TableCellLayout truncate=true>
                                                                <Spinner size=SpinnerSize::Small label="Pending..." />
                                                            </TableCellLayout>
                                                        </TableCell>
                                                        <TableCell>
                                                            <Button disabled=true icon=icondata::AiCloseCircleOutlined />
                                                        </TableCell>
                                                    </TableRow>
                                                }
                                            })
                                            .collect_view()
                                            .into_any()
                                    }}
                                </TableBody>
                            </Table>
                    </Card>
                </ErrorBoundary>
            </Transition>
            </Flex>
        </AppLayout>
    }
}
