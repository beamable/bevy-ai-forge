use super::super::utils::macros::{create_request, create_request_no_args};
use beam_common_accounts::apis::default_api::*;
use beam_common_accounts::models::*;
use beam_common_accounts::*;
use bevy::prelude::*;

create_request_no_args!(
    GetAccountMeTask,
    GetAccountMe,
    GetAccountMeCompletedEvent,
    beam_common_accounts::apis::default_api::basic_accounts_me_get,
    AccountPlayerView,
    BasicAccountsMeGetError
);
create_request!(
    AttachFederatedIdentityTask,
    AttachFederatedIdentity,
    AttachFederatedIdentityCompletedEvent,
    beam_common_accounts::apis::default_api::basic_accounts_external_identity_post,
    AttachExternalIdentityApiRequest,
    AttachExternalIdentityApiResponse,
    BasicAccountsExternalIdentityPostError
);
