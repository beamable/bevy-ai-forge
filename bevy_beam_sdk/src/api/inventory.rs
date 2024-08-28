use crate::utils::macros::beam_request;
use apis::default_api::{
    ObjectInventoryObjectIdGetError, ObjectInventoryObjectIdGetParams,
    ObjectInventoryObjectIdPutError, ObjectInventoryObjectIdPutParams,
};
use beam_autogen_rs::*;
use bevy::prelude::*;
use models::{CommonResponse, InventoryView};

beam_request!(
    InventoryAddTask,
    InventoryAdd,
    InventoryAddCompletedEvent,
    beam_autogen_rs::apis::default_api::object_inventory_object_id_put,
    CommonResponse,
    ObjectInventoryObjectIdPutError,
    ObjectInventoryObjectIdPutParams
);

beam_request!(
    InventoryGetTask,
    InventoryGet,
    InventoryGetCompletedEvent,
    beam_autogen_rs::apis::default_api::object_inventory_object_id_get,
    InventoryView,
    ObjectInventoryObjectIdGetError,
    ObjectInventoryObjectIdGetParams
);
