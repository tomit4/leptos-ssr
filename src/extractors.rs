use axum_extra::extract::cookie::Cookie;
// use axum_macros::FromRef;
use http::header;
use http::header::*;
use http::StatusCode;
use leptos::*;
use leptos_server::ServerFnError;
use serde::Deserialize;
// use sqlx::SqlitePool;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct MyQuery {
    foo: String,
}

/// Derive FromRef to allow multiple items in state, using Axum's SubStates pattern.
/*
#[derive(FromRef, Debug, Clone)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub pool: SqlitePool,
}

#[allow(dead_code)]
struct SomeStateExtractor(usize);

#[server]
pub async fn axum_extract() -> Result<String, ServerFnError> {
    use axum::{extract::Query, http::Method};
    use leptos_axum::extract;
    // use leptos_axum::{extract, extract_with_state};

    let (method, query): (Method, Query<MyQuery>) = extract().await?;

    let _state = expect_context::<AppState>();

    // let SomeStateExtractor(data) = extract_with_state::<SomeStateExtractor>(&state).await?;

    Ok(format!("{method:?} and {query:?}"))
}
*/

#[server]
pub async fn tea_and_cookies() -> Result<(), ServerFnError> {
    let response = expect_context::<leptos_axum::ResponseOptions>();

    // response.set_status(StatusCode::IM_A_TEAPOT);
    response.set_status(StatusCode::OK);

    // ::build deprecated
    // let mut cookie = Cookie::build(("biscuits", "yes")).finish();
    let cookie = Cookie::new("biscuits", "yes");

    let cookie_header_value = match HeaderValue::from_str(&cookie.to_string()) {
        Ok(value) => value,
        Err(e) => return Err(ServerFnError::ServerError(e.to_string())),
    };

    response.insert_header(header::SET_COOKIE, cookie_header_value);

    Ok(())
}

#[component]
pub fn CookieTestComponent() -> impl IntoView {
    let (response_message, set_response_message) = create_signal(String::new());

    create_effect(move |_| {
        spawn_local(async move {
            match tea_and_cookies().await {
                Ok(_) => set_response_message.set("Cookie has been set successfully".into()),
                Err(err) => set_response_message.set(format!("Error setting cookie: {:?}", err)),
            }
        });
    });

    // Render the component view
    view! {
        <div>
            <h1>"Testing Cookie Setting"</h1>
            <p>{move || response_message.get()}</p>
        </div>
    }
}

/* Example of Redirect
 * see: https://book.leptos.dev/server/27_response.html#redirect
#[server(Login, "/api")]
pub async fn login(
    username: String,
    password: String,
    remember: Option<String>,
) -> Result<(), ServerFnError> {
    // pull the DB pool and auth provider from context
    let pool = pool()?;
    let auth = auth()?;

    // check whether the user exists
    let user: User = User::get_from_username(username, &pool)
        .await
        .ok_or_else(|| {
            ServerFnError::ServerError("User does not exist.".into())
        })?;

    // check whether the user has provided the correct password
    match verify(password, &user.password)? {
        // if the password is correct...
        true => {
            // log the user in
            auth.login_user(user.id);
            auth.remember_user(remember.is_some());

            // and redirect to the home page
            leptos_axum::redirect("/");
            Ok(())
        }
        // if not, return an error
        false => Err(ServerFnError::ServerError(
            "Password does not match.".to_string(),
        )),
    }
}
*/
