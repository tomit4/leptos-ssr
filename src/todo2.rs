use leptos::*;
use leptos_router::*;
use leptos_server::ServerFnError;

#[cfg(feature = "ssr")]
pub mod ssr {
    use leptos::server_fn::ServerFnError;
    use sqlx::{Connection, Row, SqliteConnection};
    pub async fn db() -> Result<SqliteConnection, ServerFnError> {
        Ok(SqliteConnection::connect("my_database.db").await?)
    }
    pub async fn get_todos() -> Result<Vec<String>, ServerFnError> {
        let mut conn = db().await?;

        let rows = sqlx::query("SELECT title FROM todos")
            .fetch_all(&mut conn)
            .await?;
        Ok(rows
            .iter()
            .map(|row| row.get::<String, _>("title"))
            .collect())
    }
}

#[server(GetTodos, "/api/")]
pub async fn get_todos() -> Result<Vec<String>, ServerFnError> {
    use self::ssr::*;
    get_todos().await
}

#[server(AddTodo, "/api")]
pub async fn add_todo(title: String) -> Result<(), ServerFnError> {
    use self::ssr::*;
    let mut conn = db().await?;

    match sqlx::query("INSERT INTO todos (title, completed) VALUES ($1, false)")
        .bind(title)
        .execute(&mut conn)
        .await
    {
        Ok(_row) => Ok(()),
        Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    }
}

#[component]
pub fn TodoComponent() -> impl IntoView {
    let add_todo = create_server_action::<AddTodo>();
    let (todos, set_todos) = create_signal(Vec::<String>::new());

    let input_element: NodeRef<html::Input> = create_node_ref();

    let fetch_and_update_todos = async move {
        match get_todos().await {
            Ok(new_todos) => {
                set_todos.set(new_todos);
            }
            Err(e) => {
                println!("Error fetching todos: {}", e);
            }
        }
    };

    spawn_local(fetch_and_update_todos);

    // let value = add_todo.value();
    // let has_error = move || value.with(|val| matches!(val, Some(Err(_))));

    let on_submit = move |ev: ev::SubmitEvent| {
        let data = AddTodo::from_event(&ev);
        if data.is_err() || data.unwrap().title == "nope!" {
            ev.prevent_default();
        }
    };

    view! {
        <ActionForm action=add_todo on:submit=on_submit>
            <label>
                "Add a Todo"
                // `title` matches the `title` argument to `add_todo`
                <input type="text" name="title" />
            </label>
            <input type="submit" value="Add" node_ref=input_element/>
        </ActionForm>
        <Suspense fallback=move || view! { <p>"Loading todos..."</p> }>
            <ul>
            { move || {
                todos
                    .get()
                    .into_iter()
                    .map(|todo| view! {<li>{todo}</li>})
                    .collect_view()
                }
            }
            </ul>
        </Suspense>
    }
}
