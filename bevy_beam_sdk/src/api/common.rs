use crate::utils::macros::beam_request;
use beam_autogen_rs::apis::default_api::*;
use beam_autogen_rs::*;
use bevy::prelude::*;
use models::{RealmConfiguration, Token, TokenResponse};

beam_request!(
    CreateAnononymousUserTask,
    CreateAnononymousUser,
    CreateAnononymousUserCompletedEvent,
    beam_autogen_rs::apis::default_api::basic_auth_token_post,
    TokenResponse,
    BasicAuthTokenPostError,
    BasicAuthTokenPostParams
);

beam_request!(
    GetTokenTask,
    GetToken,
    GetTokenEvent,
    beam_autogen_rs::apis::default_api::basic_auth_token_get,
    Token,
    BasicAuthTokenGetError,
    BasicAuthTokenGetParams
);

beam_request!(
    PostTokenTask,
    PostToken,
    PostTokenEvent,
    beam_autogen_rs::apis::default_api::basic_auth_token_post,
    TokenResponse,
    BasicAuthTokenPostError,
    BasicAuthTokenPostParams
);

beam_request!(
    RealmsConfigTask,
    RealmsConfig,
    RealmsConfigEvent,
    beam_autogen_rs::apis::default_api::basic_realms_client_defaults_get,
    RealmConfiguration,
    BasicRealmsClientDefaultsGetError,
    BasicRealmsClientDefaultsGetParams
);
