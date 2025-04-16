use crate::prelude::*;
use beam_autogen_rs::apis::default_api::*;
use beam_autogen_rs::models::ClientManifestJsonResponse;
use beam_autogen_rs::*;
use bevy::prelude::*;

#[derive(Debug, BeamCommand)]
#[beam_command(GotManifestEvent, ClientManifestJsonResponse, apis::Error<BasicContentManifestPublicJsonGetError>, basic_content_manifest_public_json_get)]
pub struct GetManifest(pub BasicContentManifestPublicJsonGetParams, pub Entity);
