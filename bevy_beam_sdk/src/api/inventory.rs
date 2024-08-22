use beam_autogen_rs::apis::default_api::*;
use beam_autogen_rs::*;
use bevy::ecs::world::Command;
use bevy::prelude::*;
use models::{CommonResponse, InventoryUpdateRequest, InventoryView, ItemCreateRequest};

#[derive(Event, Debug, Deref, DerefMut)]
pub struct InventoryAddCompletedEvent(
    pub Result<CommonResponse, apis::Error<ObjectInventoryObjectIdPutError>>,
);

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct InventoryAdd {
    res: crossbeam_channel::Receiver<
        Result<CommonResponse, apis::Error<ObjectInventoryObjectIdPutError>>,
    >,
}
impl InventoryAdd {
    fn poll(
        &mut self,
    ) -> Option<Result<CommonResponse, apis::Error<ObjectInventoryObjectIdPutError>>> {
        if let Ok(v) = self.res.try_recv() {
            Some(v)
        } else {
            None
        }
    }

    pub(crate) fn new(
        res: crossbeam_channel::Receiver<
            Result<CommonResponse, apis::Error<ObjectInventoryObjectIdPutError>>,
        >,
    ) -> Self {
        Self { res }
    }
    fn handle_request(
        mut commands: Commands,
        mut q: Query<(Entity, &mut InventoryAdd)>,
        mut ev: EventWriter<InventoryAddCompletedEvent>,
    ) {
        for (e, mut request) in q.iter_mut() {
            if let Some(result) = request.poll() {
                commands.entity(e).despawn();
                ev.send(InventoryAddCompletedEvent(result));
            }
        }
    }
    pub(crate) fn register(app: &mut App) {
        app.add_event::<InventoryAddCompletedEvent>()
            .add_systems(Update, Self::handle_request);
    }
}

pub struct InventoryAddCommand {
    pub new_items: Vec<String>,
}

impl Command for InventoryAddCommand {
    fn apply(self, world: &mut World) {
        let request_client = world
            .get_resource::<crate::requests::ReqwestClient>()
            .unwrap();
        let context = world
            .get_resource::<crate::context::BeamContext>()
            .unwrap();
        let config = world.get_resource::<crate::config::BeamableConfig>().unwrap();
        let thread_pool = bevy::tasks::IoTaskPool::get();

        let api_key = Some(apis::configuration::ApiKey {
            key: format!("{}.{}", config.cid, config.pid),
            prefix: None,
        });
        let mut config = apis::configuration::Configuration {
            client: (**request_client).clone(),
            api_key: api_key,
            ..Default::default()
        };
        if let Some(token) = &context.token {
            if let Some(access_token) = &token.access_token {
                config.bearer_access_token = Some(access_token.clone());
            }
        }

        let id = context.get_gamer_tag().unwrap_or(0).to_string();
        let data = InventoryUpdateRequest {
            new_items: Some(
                self.new_items
                    .iter()
                    .map(|i| ItemCreateRequest {
                        content_id: i.clone(),
                        properties: vec![],
                    })
                    .collect(),
            ),
            apply_vip_bonus: None,
            currencies: None,
            currency_content_ids: vec![],
            currency_properties: None,
            transaction: None,
            delete_items: None,
            empty: false,
            item_content_ids: vec![],
            update_items: None,
        };
        let (tx, task) = crossbeam_channel::bounded(1);

        thread_pool
            .spawn(async move {
                #[cfg(target_family = "wasm")]
                let r =
                    (apis::default_api::object_inventory_object_id_put)(&config, &id, Some(data))
                        .await;

                #[cfg(not(target_family = "wasm"))]
                let r = async_compat::Compat::new(async {
                    let x_beam_scope = config.api_key.clone().unwrap().key;
                    let id = id.clone();
                    (apis::default_api::object_inventory_object_id_put)(&config, &x_beam_scope, &id, Some(&id),Some(data))
                        .await
                })
                .await;
                println!("{:#?}", r);
                tx.send(r).ok();
            })
            .detach();
        world.spawn(InventoryAdd::new(task));
    }
}

#[derive(Event, Debug, Deref, DerefMut)]
pub struct InventoryGetCompletedEvent(
    pub Result<InventoryView, apis::Error<ObjectInventoryObjectIdGetError>>,
);

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct InventoryGet {
    res: crossbeam_channel::Receiver<
        Result<InventoryView, apis::Error<ObjectInventoryObjectIdGetError>>,
    >,
}
impl InventoryGet {
    fn poll(
        &mut self,
    ) -> Option<Result<InventoryView, apis::Error<ObjectInventoryObjectIdGetError>>> {
        if let Ok(v) = self.res.try_recv() {
            Some(v)
        } else {
            None
        }
    }

    pub(crate) fn new(
        res: crossbeam_channel::Receiver<
            Result<InventoryView, apis::Error<ObjectInventoryObjectIdGetError>>,
        >,
    ) -> Self {
        Self { res }
    }
    fn handle_request(
        mut commands: Commands,
        mut q: Query<(Entity, &mut InventoryGet)>,
        mut ev: EventWriter<InventoryGetCompletedEvent>,
    ) {
        for (e, mut request) in q.iter_mut() {
            if let Some(result) = request.poll() {
                commands.entity(e).despawn();
                ev.send(InventoryGetCompletedEvent(result));
            }
        }
    }
    pub(crate) fn register(app: &mut App) {
        app.add_event::<InventoryGetCompletedEvent>()
            .add_systems(Update, Self::handle_request);
    }
}

pub struct InventoryGetCommand {
    pub scope: String,
}

impl Command for InventoryGetCommand {
    fn apply(self, world: &mut World) {
        let request_client = world
            .get_resource::<crate::requests::ReqwestClient>()
            .unwrap();
        let context = world
            .get_resource::<crate::context::BeamContext>()
            .unwrap();
        let config = world.get_resource::<crate::config::BeamableConfig>().unwrap();
        let thread_pool = bevy::tasks::IoTaskPool::get();

        let api_key = Some(apis::configuration::ApiKey {
            key: format!("{}.{}", config.cid, config.pid),
            prefix: None,
        });
        let mut config = apis::configuration::Configuration {
            client: (**request_client).clone(),
            api_key,
            ..Default::default()
        };
        if let Some(token) = &context.token {
            if let Some(access_token) = &token.access_token {
                config.bearer_access_token = Some(access_token.clone());
            }
        }

        let id = context.get_gamer_tag().unwrap_or(0).to_string();
        let (tx, task) = crossbeam_channel::bounded(1);

        thread_pool
            .spawn(async move {
                let x_beam_scope = config.api_key.clone().unwrap().key;
                let id = id.clone();
                #[cfg(target_family = "wasm")]
                let r = (apis::default_api::object_inventory_object_id_get)(
                    &config,
                    &id,
                    Some(&self.scope),
                )
                .await;

                #[cfg(not(target_family = "wasm"))]
                let r = async_compat::Compat::new(async {
                    (apis::default_api::object_inventory_object_id_get)(
                        &config,
                                    &x_beam_scope,
                        &id,
                        None,
                        Some(&self.scope),
                    )
                    .await
                })
                .await;
                tx.send(r).ok();
            })
            .detach();
        world.spawn(InventoryGet::new(task));
    }
}
