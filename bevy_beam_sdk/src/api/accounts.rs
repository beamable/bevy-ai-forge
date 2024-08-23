use super::super::utils::macros::create_request_with_args;
use beam_autogen_rs::apis::default_api::*;
use beam_autogen_rs::*;
use bevy::prelude::*;
use models::{AccountPlayerView, AttachExternalIdentityApiResponse};

create_request_with_args!(
    GetAccountMeTask,
    GetAccountMe,
    GetAccountMeCompletedEvent,
    beam_autogen_rs::apis::default_api::basic_accounts_me_get,
    BasicAccountsMeGetParams,
    AccountPlayerView,
    BasicAccountsMeGetError
);

create_request_with_args!(
    AttachFederatedIdentityTask,
    AttachFederatedIdentity,
    AttachFederatedIdentityCompletedEvent,
    beam_autogen_rs::apis::default_api::basic_accounts_external_identity_post,
    BasicAccountsExternalIdentityPostParams,
    AttachExternalIdentityApiResponse,
    BasicAccountsExternalIdentityPostError
);
