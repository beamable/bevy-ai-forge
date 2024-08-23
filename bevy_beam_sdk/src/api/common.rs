use crate::utils::macros::create_request_with_args;
use beam_autogen_rs::apis::default_api::*;
use beam_autogen_rs::*;
use bevy::prelude::*;
use models::{Token, TokenResponse};

create_request_with_args!(
    CreateAnononymousUserTask,
    CreateAnononymousUser,
    CreateAnononymousUserCompletedEvent,
    beam_autogen_rs::apis::default_api::basic_auth_token_post,
    BasicAuthTokenPostParams,
    TokenResponse,
    BasicAuthTokenPostError
);

create_request_with_args!(
    GetTokenTask,
    GetToken,
    GetTokenEvent,
    beam_autogen_rs::apis::default_api::basic_auth_token_get,
    BasicAuthTokenGetParams,
    Token,
    BasicAuthTokenGetError
);

create_request_with_args!(
    PostTokenTask,
    PostToken,
    PostTokenEvent,
    beam_autogen_rs::apis::default_api::basic_auth_token_post,
    BasicAuthTokenPostParams,
    TokenResponse,
    BasicAuthTokenPostError
);