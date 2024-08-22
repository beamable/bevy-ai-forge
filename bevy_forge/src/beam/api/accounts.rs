use super::super::utils::macros::{create_request_with_scope, create_request_no_args};
use beam_autogen_rs::apis::default_api::*;
use beam_autogen_rs::*;
use bevy::prelude::*;
use models::{AccountPlayerView, AttachExternalIdentityApiRequest, AttachExternalIdentityApiResponse};

create_request_no_args!(
    GetAccountMeTask,
    GetAccountMe,
    GetAccountMeCompletedEvent,
    beam_autogen_rs::apis::default_api::basic_accounts_me_get,
    AccountPlayerView,
    BasicAccountsMeGetError
);

create_request_with_scope!(
    AttachFederatedIdentityTask,
    AttachFederatedIdentity,
    AttachFederatedIdentityCompletedEvent,
    beam_autogen_rs::apis::default_api::basic_accounts_external_identity_post,
    AttachExternalIdentityApiRequest,
    AttachExternalIdentityApiResponse,
    BasicAccountsExternalIdentityPostError
);
