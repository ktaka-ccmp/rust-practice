use anyhow::{Context, Result};
use async_session::{MemoryStore, Session, SessionStore};
use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts, Query, State},
    http::{header::SET_COOKIE, HeaderMap},
    response::{IntoResponse, Redirect, Response},
    routing::get,
    RequestPartsExt, Router,
};
use axum_extra::{headers, typed_header::TypedHeaderRejectionReason, TypedHeader};
use http::{header, request::Parts, StatusCode};
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthUrl, AuthorizationCode, ClientId,
    ClientSecret, CsrfToken, RedirectUrl, ResponseType, Scope, TokenResponse, TokenUrl,
};
use serde::{Deserialize, Serialize};
use std::env;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use reqwest::Client;

static COOKIE_NAME: &str = "SESSION";

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_oauth=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // `MemoryStore` is just used as an example. Don't use this in production.
    let store = MemoryStore::new();
    let oauth_client = oauth_client().unwrap();
    let app_state = AppState {
        store,
        oauth_client,
    };

    let app = Router::new()
        .route("/", get(index))
        .route("/auth/google", get(google_auth))
        .route("/auth/authorized", get(login_authorized))
        .route("/protected", get(protected))
        .route("/logout", get(logout))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .context("failed to bind TcpListener")
        .unwrap();

    tracing::debug!(
        "listening on {}",
        listener
            .local_addr()
            .context("failed to return local address")
            .unwrap()
    );

    axum::serve(listener, app).await.unwrap();
}

#[derive(Clone)]
struct AppState {
    store: MemoryStore,
    oauth_client: BasicClient,
}

impl FromRef<AppState> for MemoryStore {
    fn from_ref(state: &AppState) -> Self {
        state.store.clone()
    }
}

impl FromRef<AppState> for BasicClient {
    fn from_ref(state: &AppState) -> Self {
        state.oauth_client.clone()
    }
}

fn oauth_client() -> Result<BasicClient, AppError> {
    let client_id = env::var("CLIENT_ID").context("Missing CLIENT_ID!")?;
    let client_secret = env::var("CLIENT_SECRET").context("Missing CLIENT_SECRET!")?;
    let origin = env::var("ORIGIN").context("Missing ORIGIN!")?;
    let redirect_url = format!("{}/auth/authorized", origin);

    let auth_url = env::var("AUTH_URL")
        .unwrap_or_else(|_| "https://accounts.google.com/o/oauth2/v2/auth".to_string());

    let token_url =
        env::var("TOKEN_URL").unwrap_or_else(|_| "https://oauth2.googleapis.com/token".to_string());

    Ok(BasicClient::new(
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
        AuthUrl::new(auth_url).context("failed to create new authorization server URL")?,
        Some(TokenUrl::new(token_url).context("failed to create new token endpoint URL")?),
    )
    .set_redirect_uri(
        RedirectUrl::new(redirect_url).context("failed to create new redirection URL")?,
    ))
}

// The user data we'll get back from Discord.
// https://discord.com/developers/docs/resources/user#user-object-user-structure
#[derive(Debug, Serialize, Deserialize)]
struct User {
    family_name: String,
    name: String,
    picture: String,
    email: String,
    given_name: String,
    id: String,
    hd: String,
    verified_email: bool,
}

// Session is optional
async fn index(user: Option<User>) -> impl IntoResponse {
    match user {
        Some(u) => format!(
            "Hey {}! You're logged in!\nYou may now access `/protected`.\nLog out with `/logout`.",
            u.name
        ),
        None => "You're not logged in.\nVisit `/auth/google` to do so.".to_string(),
    }
}

use urlencoding::encode;

async fn google_auth(State(client): State<BasicClient>) -> impl IntoResponse {
    let (_auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        // .set_response_type(&ResponseType::new("code id_token".to_string()))
        .set_response_type(&ResponseType::new("code".to_string()))
        .add_scope(Scope::new("openid".to_string()))
        .add_scope(Scope::new("email".to_string()))
        .add_scope(Scope::new("profile".to_string()))
        .add_extra_param("prompt", "consent")
        .add_extra_param("access_type", "online")
        .add_extra_param("nonce", "some_nonce")
        .add_extra_param("response_mode", "query")
        .url();

    println!("Auth URL query: {:#?}", _auth_url.query());
    println!("CSRF Token: {:#?}", csrf_token.secret());

    let __auth_url = reqwest::Client::new()
        .get(client.auth_url().to_string())
        .query(&[
            ("client_id", client.client_id().to_string()),
            ("redirect_uri", client.redirect_url().unwrap().as_str().to_string()),
            ("scope", "openid email profile".to_string()),
            ("prompt", "consent".to_string()),
            ("access_type", "online".to_string()),
            ("nonce", "some_nonce".to_string()),
            ("response_mode", "query".to_string()),
        ])
        .build();

    let auth_url = format!(
        "{}?response_type={}&client_id={}&redirect_uri={}&scope={}&prompt=consent&access_type=online&nonce=some_nonce&response_mode=query",
        client.auth_url().to_string(),
        "code",
        client.client_id().to_string(),
        encode(client.redirect_url().unwrap().as_str()),
        "openid+email+profile",
    );

    println!("Auth URL: {:#?}", auth_url);

    // Redirect to Discord's oauth service
    Redirect::to(auth_url.as_str())
}

// Valid user session required. If there is none, redirect to the auth page
async fn protected(user: User) -> impl IntoResponse {
    format!("Welcome to the protected area :)\nHere's your info:\n{user:?}")
}

async fn logout(
    State(store): State<MemoryStore>,
    TypedHeader(cookies): TypedHeader<headers::Cookie>,
) -> Result<impl IntoResponse, AppError> {
    let cookie = cookies
        .get(COOKIE_NAME)
        .context("unexpected error getting cookie name")?;

    let session = match store
        .load_session(cookie.to_string())
        .await
        .context("failed to load session")?
    {
        Some(s) => s,
        // No session active, just redirect
        None => return Ok(Redirect::to("/")),
    };

    store
        .destroy_session(session)
        .await
        .context("failed to destroy session")?;

    Ok(Redirect::to("/"))
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct AuthRequest {
    code: String,
    state: String,
    id_token: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct OidcTokenResponse {
    access_token: String,
    token_type: String,
    expires_in: u64,
    refresh_token: Option<String>,
    scope: String,
    id_token: Option<String>,
}


async fn login_authorized(
    Query(query): Query<AuthRequest>,
    // State(store): State<MemoryStore>,
    State(oauth_client): State<BasicClient>,
) -> Result<impl IntoResponse, AppError> {
    println!("Query: {:#?}", query);
    println!("code: {:#?}", query.code);
    println!("oauth_client: {:#?}", oauth_client);

    println!("Client ID: {:#?}", oauth_client.client_id());
    println!("Redirect URI: {:#?}", oauth_client.redirect_url());
    println!("Token URL: {:#?}", oauth_client.token_url());

    let client_secret = env::var("CLIENT_SECRET").context("Missing CLIENT_SECRET!")?;

    println!("Client Secret: {:#?}", client_secret);

    // println!("Grant Type: {:#?}", oauth_client.grant_type());

    let response = reqwest::Client::new()
        .post(oauth_client.token_url().unwrap().as_str())
        .form(&[
            ("code", query.code.clone()),
            ("client_id", oauth_client.client_id().to_string()),
            ("client_secret", client_secret),
            ("redirect_uri", oauth_client.redirect_url().unwrap().as_str().to_string()),
            ("grant_type", "authorization_code".to_string()),
        ])
        .send()
        .await
        .context("failed in sending request request to authorization server")?;

    println!("Response: {:#?}", response);
    // println!("Response body: {:#?}", response.text().await);
    println!("Response json: {:#?}", response.json::<OidcTokenResponse>().await);

    Ok(Redirect::to("/"))
}

async fn _login_authorized(
    Query(query): Query<AuthRequest>,
    State(store): State<MemoryStore>,
    State(oauth_client): State<BasicClient>,
) -> Result<impl IntoResponse, AppError> {
    println!("Query: {:#?}", query);
    println!("code: {:#?}", query.code);
    // Get an auth token
    let token = oauth_client
        .exchange_code(AuthorizationCode::new(query.code.clone()))
        .request_async(async_http_client)
        .await
        .context("failed in sending request request to authorization server")?;

    println!("Token: {:#?}", token);
    println!("access_token: {:#?}", token.access_token().secret());

    // Fetch user data from discord
    let client = reqwest::Client::new();
    let user_data: User = client
        .get("https://www.googleapis.com/userinfo/v2/me")
        .bearer_auth(token.access_token().secret())
        .send()
        .await
        .context("failed in sending request to target Url")?
        .json::<User>()
        .await
        .context("failed to deserialize response as JSON")?;

    // Create a new session filled with user data
    let mut session = Session::new();
    session
        .insert("user", &user_data)
        .context("failed in inserting serialized value into session")?;

    println!("User data: {:#?}", user_data);
    println!("Session: {:#?}", session);

    // Store session and get corresponding cookie
    let cookie = store
        .store_session(session)
        .await
        .context("failed to store session")?
        .context("unexpected error retrieving cookie value")?;

    // Build the cookie
    let cookie = format!("{COOKIE_NAME}={cookie}; SameSite=Lax; Path=/");

    println!("Cookie: {:#?}", cookie);

    // Set cookie
    let mut headers = HeaderMap::new();
    headers.insert(
        SET_COOKIE,
        cookie.parse().context("failed to parse cookie")?,
    );

    Ok((headers, Redirect::to("/")))
}

struct AuthRedirect;

impl IntoResponse for AuthRedirect {
    fn into_response(self) -> Response {
        Redirect::temporary("/auth/google").into_response()
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for User
where
    MemoryStore: FromRef<S>,
    S: Send + Sync,
{
    // If anything goes wrong or no session is found, redirect to the auth page
    type Rejection = AuthRedirect;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let store = MemoryStore::from_ref(state);

        let cookies = parts
            .extract::<TypedHeader<headers::Cookie>>()
            .await
            .map_err(|e| match *e.name() {
                header::COOKIE => match e.reason() {
                    TypedHeaderRejectionReason::Missing => AuthRedirect,
                    _ => panic!("unexpected error getting Cookie header(s): {e}"),
                },
                _ => panic!("unexpected error getting cookies: {e}"),
            })?;
        let session_cookie = cookies.get(COOKIE_NAME).ok_or(AuthRedirect)?;

        let session = store
            .load_session(session_cookie.to_string())
            .await
            .unwrap()
            .ok_or(AuthRedirect)?;

        let user = session.get::<User>("user").ok_or(AuthRedirect)?;

        Ok(user)
    }
}

// Use anyhow, define error and enable '?'
// For a simplified example of using anyhow in axum check /examples/anyhow-error-response
#[derive(Debug)]
struct AppError(anyhow::Error);

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        tracing::error!("Application error: {:#}", self.0);

        (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
