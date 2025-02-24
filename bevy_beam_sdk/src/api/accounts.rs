use crate::prelude::*;
use beam_autogen_rs::apis::default_api::*;
use beam_autogen_rs::*;
use bevy::prelude::*;
use models::{AccountPlayerView, AttachExternalIdentityApiResponse};

#[derive(Debug, BeamCommand)]
#[beam_command(GetAccountMeCompletedEvent, AccountPlayerView, apis::Error<BasicAccountsMeGetError>, basic_accounts_me_get)]
pub struct GetAccountMe(pub BasicAccountsMeGetParams, pub Entity);

#[derive(Debug, BeamCommand)]
#[beam_command(AttachFederatedIdentityCompletedEvent, AttachExternalIdentityApiResponse, apis::Error<BasicAccountsExternalIdentityPostError>, basic_accounts_external_identity_post)]
pub struct AttachFederatedIdentity(pub BasicAccountsExternalIdentityPostParams, pub Entity);
