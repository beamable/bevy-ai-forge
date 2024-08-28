use super::super::utils::macros::beam_request;
use beam_autogen_rs::apis::default_api::*;
use beam_autogen_rs::*;
use bevy::prelude::*;
use models::{AccountPlayerView, AttachExternalIdentityApiResponse};

beam_request!(
    GetAccountMeTask,
    GetAccountMe,
    GetAccountMeCompletedEvent,
    beam_autogen_rs::apis::default_api::basic_accounts_me_get,
    AccountPlayerView,
    BasicAccountsMeGetError,
    BasicAccountsMeGetParams
);

beam_request!(
    AttachFederatedIdentityTask,
    AttachFederatedIdentity,
    AttachFederatedIdentityCompletedEvent,
    beam_autogen_rs::apis::default_api::basic_accounts_external_identity_post,
    AttachExternalIdentityApiResponse,
    BasicAccountsExternalIdentityPostError,
    BasicAccountsExternalIdentityPostParams
);
