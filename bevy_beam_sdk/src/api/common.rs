use crate::prelude::*;
use beam_autogen_rs::apis::default_api::*;
use beam_autogen_rs::*;
use bevy::prelude::*;
use models::{RealmConfiguration, Token, TokenResponse};

#[derive(Debug, BeamCommand)]
#[beam_command(GetTokenEvent, Token, apis::Error<BasicAuthTokenGetError>, basic_auth_token_get)]
pub struct GetToken(pub BasicAuthTokenGetParams, pub Entity);

#[derive(Debug, BeamCommand)]
#[beam_command(UserAuthenticationEvent, TokenResponse, apis::Error<BasicAuthTokenPostError>, basic_auth_token_post)]
pub struct AuthenticateUser(pub BasicAuthTokenPostParams, pub Entity);

#[derive(Debug, BeamCommand)]
#[beam_command(RealmsConfigEvent, RealmConfiguration, apis::Error<BasicRealmsClientDefaultsGetError>, basic_realms_client_defaults_get)]
pub struct RealmsConfig(pub BasicRealmsClientDefaultsGetParams, pub Entity);
