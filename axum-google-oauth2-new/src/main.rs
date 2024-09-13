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

use serde::{Deserialize, Serialize};
use std::env;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};


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

    let oauth2_params = OAuth2Params {
        client_id: env::var("CLIENT_ID").expect("Missing CLIENT_ID!"),
        client_secret: env::var("CLIENT_SECRET").expect("Missing CLIENT_SECRET!"),
        redirect_uri: format!("{}/auth/authorized", env::var("ORIGIN").expect("Missing ORIGIN!")),
        auth_url: "https://accounts.google.com/o/oauth2/v2/auth".to_string(),
        token_url: "https://oauth2.googleapis.com/token".to_string(),
        response_type: ResponseType::Code.as_str().to_string(),
        scope: "openid+email+profile".to_string(),
        nonce: None,
        state: None,
        csrf_token: None,
        response_mode: Some(ResponseMode::Query),   // "query",
        prompt: Some(Prompt::Consent),              // "consent",
        access_type: Some(AccessType::Online),      // "online",
    };

    let app_state = AppState {
        store,
        oauth2_params,
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

#[derive(Debug, Clone)]
enum ResponseMode {
    Query,
    Fragment,
    FormPost,
}

impl ResponseMode {
    fn as_str(&self) -> &str {
        match self {
            Self::Query => "query",
            Self::Fragment => "fragment",
            Self::FormPost => "form_post",
        }
    }
}

#[derive(Debug, Clone)]
enum Prompt {
    None,
    Consent,
    SelectAccount,
    Login,
    ConsentSelectAccount,
    ConsentLogin,
    SelectAccountLogin,
    ConsentSelectAccountLogin,
}

impl Prompt {
    fn as_str(&self) -> &str {
        match self {
            Self::None => "none",
            Self::Consent => "consent",
            Self::SelectAccount => "select_account",
            Self::Login => "login",
            Self::ConsentSelectAccount => "consent select_account",
            Self::ConsentLogin => "consent login",
            Self::SelectAccountLogin => "select_account login",
            Self::ConsentSelectAccountLogin => "consent select_account login",
        }
    }
}

#[derive(Debug, Clone)]
enum AccessType {
    Online,
    Offline,
}

impl AccessType {
    fn as_str(&self) -> &str {
        match self {
            Self::Online => "online",
            Self::Offline => "offline",
        }
    }
}

enum ResponseType {
    None = 0b000,
    Code = 0b001,
    Token = 0b010,
    IdToken = 0b100,
    CodeToken = 0b011,
    CodeIdToken = 0b101,
    TokenIdToken = 0b110,
    CodeTokenIdToken = 0b111,
}

impl ResponseType {
    fn as_str(&self) -> &str {
        match self {
            Self::None => "",
            Self::Code => "code",
            Self::Token => "token",
            Self::IdToken => "id_token",
            Self::CodeToken => "code token",
            Self::CodeIdToken => "code id_token",
            Self::TokenIdToken => "token id_token",
            Self::CodeTokenIdToken => "code token id_token",
        }
    }
}

#[derive(Clone, Debug)]
struct OAuth2Params {
    client_id: String,
    client_secret: String,
    redirect_uri: String,
    auth_url: String,
    token_url: String,
    response_type: String,
    scope: String,
    nonce: Option<String>,
    state: Option<String>,
    csrf_token: Option<String>,
    response_mode: Option<ResponseMode>,
    prompt: Option<Prompt>,
    access_type: Option<AccessType>,
}

#[derive(Clone)]
struct AppState {
    store: MemoryStore,
    oauth2_params: OAuth2Params,
}

impl FromRef<AppState> for MemoryStore {
    fn from_ref(state: &AppState) -> Self {
        state.store.clone()
    }
}

impl FromRef<AppState> for OAuth2Params {
    fn from_ref(state: &AppState) -> Self {
        state.oauth2_params.clone()
    }
}

// The user data we'll get back from Google
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

async fn google_auth(State(mut params): State<OAuth2Params>) -> impl IntoResponse {

    params.nonce = Some("some_nonce".to_string());
    params.csrf_token = Some("some_csrf_token".to_string());
    params.state = Some("some_state".to_string());

    let auth_url = format!(
        "{}?client_id={}&redirect_uri={}&response_type={}&scope={}&state={}&nonce={}&prompt={}&access_type={}&response_mode={}",
        params.auth_url,
        params.client_id,
        encode(params.redirect_uri.as_str()),
        encode(params.response_type.as_str()),
        params.scope,
        params.state.as_ref().unwrap(),
        params.nonce.as_ref().unwrap(),
        params.prompt.as_ref().unwrap().as_str(),
        params.access_type.as_ref().unwrap().as_str(),
        params.response_mode.as_ref().unwrap().as_str(),
    );
    // Need to investigate how to use nonce, state, csrf_token.

    println!("Auth URL: {:#?}", auth_url);
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
    State(store): State<MemoryStore>,
    State(params): State<OAuth2Params>,
) -> Result<impl IntoResponse, AppError> {
    println!("Query: {:#?}", query);
    println!("code: {:#?}", query.code);
    println!("Params: {:#?}", params);

    // Exchange code for access_token and id_token
    let response = reqwest::Client::new()
        .post(params.token_url)
        .form(&[
            ("code", query.code.clone()),
            ("client_id", params.client_id.clone()),
            ("client_secret", params.client_secret.clone()),
            ("redirect_uri", params.redirect_uri.clone()),
            ("grant_type", "authorization_code".to_string()),
        ])
        .send()
        .await
        .context("failed in sending request request to authorization server")?;

    let response_body = response.text().await.context("failed to get response body")?;
    let response_json: OidcTokenResponse = serde_json::from_str(&response_body).context("failed to deserialize response body")?;
    let access_token = response_json.access_token.clone();
    let _id_token = response_json.id_token.clone().unwrap();
    println!("Response JSON: {:#?}", response_json);
    // println!("Access Token: {:#?}", access_token);
    // println!("ID Token: {:#?}", id_token);

    // Get user data from Google API using access_token
    let response = reqwest::Client::new()
        .get("https://www.googleapis.com/userinfo/v2/me")
        .bearer_auth(access_token)
        .send()
        .await
        .context("failed in sending request to target Url")?;

    let response_body = response.text().await.context("failed to get response body")?;
    let user_data: User = serde_json::from_str(&response_body).context("failed to deserialize response body")?;
    // println!("Response Body: {:#?}", response_body);
    println!("User data: {:#?}", user_data);

    // Insert user data into session
    let mut session = Session::new();
    session
        .insert("user", &user_data)
        .context("failed in inserting serialized value into session")?;

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

        // Retrieve session from store
        let session = store
            .load_session(session_cookie.to_string())
            .await
            .unwrap()
            .ok_or(AuthRedirect)?;

        // Retrieve user data from session
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
