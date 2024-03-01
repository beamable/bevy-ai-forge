use crate::beam::utils::macros::create_request;
use beam_common::apis::default_api::*;
use beam_common::models::*;
use beam_common::*;
use bevy::prelude::*;

create_request!(
    CreateAnononymousUserTask,
    CreateAnononymousUser,
    CreateAnononymousUserCompletedEvent,
    beam_common::apis::default_api::basic_auth_token_post,
    TokenRequestWrapper,
    TokenResponse,
    BasicAuthTokenPostError
);

create_request!(
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
    configuration: &beam_common::apis::configuration::Configuration,
    token: Option<String>,
) -> Result<models::Token, beam_common::apis::Error<BasicAuthTokenGetError>> {
    basic_auth_token_get(configuration, &token.unwrap()).await
}
