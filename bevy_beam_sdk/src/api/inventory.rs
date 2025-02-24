use crate::prelude::*;
use apis::default_api::{
    ObjectInventoryObjectIdGetError, ObjectInventoryObjectIdGetParams,
    ObjectInventoryObjectIdPutError, ObjectInventoryObjectIdPutParams,
};
use beam_autogen_rs::apis::default_api::{
    object_inventory_object_id_get, object_inventory_object_id_put,
};
use beam_autogen_rs::*;
use bevy::prelude::*;
use models::{CommonResponse, InventoryView};

#[derive(Debug, BeamCommand)]
#[beam_command(InventoryAddCompletedEvent, CommonResponse, apis::Error<ObjectInventoryObjectIdPutError>, object_inventory_object_id_put)]
pub struct InventoryAdd(pub ObjectInventoryObjectIdPutParams, pub Entity);

#[derive(Debug, BeamCommand)]
#[beam_command(InventoryGetCompletedEvent, InventoryView, apis::Error<ObjectInventoryObjectIdGetError>, object_inventory_object_id_get)]
pub struct InventoryGet(pub ObjectInventoryObjectIdGetParams, pub Entity);
