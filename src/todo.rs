// NOTE: Possible only thanks to: 
// https://github.com/leptos-rs/leptos/blob/main/examples/todo_app_sqlite_axum/src/todo.rs
use leptos::*;
use leptos_server::ServerFnError;

#[cfg(feature = "ssr")]
pub mod ssr {
    use leptos::server_fn::ServerFnError;
    use sqlx::{Connection, SqliteConnection};
    pub async fn db() -> Result<SqliteConnection, ServerFnError> {
        Ok(SqliteConnection::connect("my_database.db").await?)
    }
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
pub fn BusyButton() -> impl IntoView {
    view! {
        <button on:click=move |_| {
            spawn_local(async {
                let _ = add_todo("So much to do!".to_string()).await;
            });
        }>
            "Add Todo"
        </button>
    }
}
