use crate::utils::macros::create_request_with_args;
use apis::default_api::{ObjectInventoryObjectIdGetError, ObjectInventoryObjectIdGetParams, ObjectInventoryObjectIdPutError, ObjectInventoryObjectIdPutParams};
use beam_autogen_rs::*;
use bevy::prelude::*;
use models::{CommonResponse, InventoryView};

create_request_with_args!(
    InventoryAddTask,
    InventoryAdd,
    InventoryAddCompletedEvent,
    beam_autogen_rs::apis::default_api::object_inventory_object_id_put,
    ObjectInventoryObjectIdPutParams,
    CommonResponse,
    ObjectInventoryObjectIdPutError
);

create_request_with_args!(
    InventoryGetTask,
    InventoryGet,
    InventoryGetCompletedEvent,
    beam_autogen_rs::apis::default_api::object_inventory_object_id_get,
    ObjectInventoryObjectIdGetParams,
    InventoryView,
    ObjectInventoryObjectIdGetError
);
