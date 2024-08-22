use crate::utils::macros::create_request_with_scope;
use beam_autogen_rs::apis::default_api::*;
use beam_autogen_rs::*;
use bevy::prelude::*;
use models::{Token, TokenRequestWrapper, TokenResponse};

create_request_with_scope!(
    CreateAnononymousUserTask,
    CreateAnononymousUser,
    CreateAnononymousUserCompletedEvent,
    beam_autogen_rs::apis::default_api::basic_auth_token_post,
    TokenRequestWrapper,
    TokenResponse,
    BasicAuthTokenPostError
);

create_request_with_scope!(
    GetTokenTask,
    GetToken,
    GetTokenEvent,
    basic_token_get,
    String,
    Token,
    BasicAuthTokenGetError
);

// I made this so I don't need to create third macro right now
async fn basic_token_get(
    configuration: &beam_autogen_rs::apis::configuration::Configuration,
    x_beam_scope: &str,
    x_beam_gamertag: Option<&str>,
    token: Option<String>,
) -> Result<models::Token, beam_autogen_rs::apis::Error<BasicAuthTokenGetError>> {
    basic_auth_token_get(
        configuration,
        x_beam_scope,
        &token.unwrap(),
        x_beam_gamertag,
    )
    .await
}

create_request_with_scope!(
    PostTokenTask,
    PostToken,
    PostTokenEvent,
    basic_token_post,
    String,
    TokenResponse,
    BasicAuthTokenPostError
);

// I made this so I don't need to create third macro right now
async fn basic_token_post(
    configuration: &beam_autogen_rs::apis::configuration::Configuration,
    x_beam_scope: &str,
    x_beam_gamertag: Option<&str>,
    token: Option<String>,
) -> Result<models::TokenResponse, beam_autogen_rs::apis::Error<BasicAuthTokenPostError>> {
    let mut wrapper = TokenRequestWrapper::new("refresh_token".to_owned());
    wrapper.refresh_token = token;
    basic_auth_token_post(configuration, x_beam_scope, x_beam_gamertag, Some(wrapper)).await
}