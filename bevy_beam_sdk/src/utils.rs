pub mod macros {
    #[macro_export]
    macro_rules! beam_request {
        (
            $task_name: ident,
            $command: ident,
            $event_completed_name: ident,
            $call_from_open_api: expr,
            $result_type_success: ident,
            $result_type_error: ident,
            $call_args_type: ty
        ) => {
            #[derive(Debug)]
            pub struct $command {
                pub data: $call_args_type,
                pub entity: Option<Entity>,
            }

            impl bevy::ecs::world::Command for $command {
                fn apply(self, world: &mut World) {
                    let thread_pool = bevy::tasks::IoTaskPool::get();
                    let default_context = $crate::context::BeamContext::default();
                    let context = world
                        .get_resource::<$crate::context::BeamContext>()
                        .unwrap_or(&default_context);

                    let request_client = world
                        .get_resource::<$crate::requests::ReqwestClient>()
                        .unwrap();
                    let key = world
                        .get_resource::<$crate::config::BeamableConfig>()
                        .unwrap()
                        .get_x_beam_scope();

                    let api_key = Some(apis::configuration::ApiKey { key, prefix: None });
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
                    let request_data = self.data.clone();

                    let (tx, task) = crossbeam_channel::bounded(1);

                    thread_pool
                        .spawn(async move {
                            #[cfg(target_family = "wasm")]
                            let r = $call_from_open_api(&config, request_data).await;

                            #[cfg(not(target_family = "wasm"))]
                            let r = async_compat::Compat::new(async {
                                $call_from_open_api(&config, request_data).await
                            })
                            .await;
                            println!("{:#?}", r);
                            tx.send(r).ok();
                        })
                        .detach();
                    if let Some(entity) = self.entity {
                        if let Ok(mut entity) = world.get_entity_mut(entity) {
                            entity.insert($task_name(task));
                        }
                    } else {
                        world.spawn($task_name(task));
                    }
                }
            }

            #[derive(Event, Debug, Deref, DerefMut)]
            pub struct $event_completed_name(
                pub Result<$result_type_success, apis::Error<$result_type_error>>,
            );

            #[derive(Component, Deref, DerefMut)]
            #[component(storage = "SparseSet")]
            pub struct $task_name(
                pub  crossbeam_channel::Receiver<
                    Result<$result_type_success, apis::Error<$result_type_error>>,
                >,
            );

            impl $task_name {
                fn poll(
                    &mut self,
                ) -> Option<Result<$result_type_success, apis::Error<$result_type_error>>> {
                    if let Ok(v) = self.0.try_recv() {
                        Some(v)
                    } else {
                        None
                    }
                }

                fn handle_request(
                    mut commands: Commands,
                    mut q: Query<(Entity, &mut $task_name)>,
                    mut ev: EventWriter<$event_completed_name>,
                ) {
                    for (e, mut request) in q.iter_mut() {
                        if let Some(result) = request.poll() {
                            commands.entity(e).despawn();
                            ev.send($event_completed_name(result));
                        }
                    }
                }

                pub(crate) fn register(app: &mut App) {
                    app.add_event::<$event_completed_name>()
                        .add_systems(Update, Self::handle_request);
                }
            }
        };
        (
            $task_name: ident,
            $command: ident,
            $event_completed_name: ident,
            $call_from_open_api: expr,
            $result_type_success: ty,
            $result_type_error: ty
        ) => {
            #[derive(Debug, Default)]
            pub struct $command;

            impl bevy::ecs::world::Command for $command {
                fn apply(self, world: &mut World) {
                    let request_client = world
                        .get_resource::<$crate::requests::ReqwestClient>()
                        .unwrap();
                    let default_context = $crate::context::BeamContext::default();
                    let context = world
                        .get_resource::<$crate::context::BeamContext>()
                        .unwrap_or(&default_context);
                    let key = world
                        .get_resource::<$crate::config::BeamableConfig>()
                        .unwrap()
                        .get_x_beam_scope();
                    let thread_pool = bevy::tasks::IoTaskPool::get();

                    let api_key = Some(apis::configuration::ApiKey { key, prefix: None });
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
                    let (tx, task) = crossbeam_channel::bounded(1);

                    thread_pool
                        .spawn(async move {
                            let x_beam_scope = config.api_key.clone().unwrap().key;

                            #[cfg(target_family = "wasm")]
                            let r = $call_from_open_api(&config).await;

                            #[cfg(not(target_family = "wasm"))]
                            let r = async_compat::Compat::new(async {
                                $call_from_open_api(&config).await
                            })
                            .await;
                            println!("{:#?}", r);
                            tx.send(r).ok();
                        })
                        .detach();

                    world.spawn($task_name(task));
                }
            }

            #[derive(Event, Debug, Deref, DerefMut)]
            pub struct $event_completed_name(
                pub Result<$result_type_success, apis::Error<$result_type_error>>,
            );

            #[derive(Component, Deref, DerefMut)]
            #[component(storage = "SparseSet")]
            pub struct $task_name(
                pub  crossbeam_channel::Receiver<
                    Result<$result_type_success, apis::Error<$result_type_error>>,
                >,
            );

            impl $task_name {
                fn poll(
                    &mut self,
                ) -> Option<Result<$result_type_success, apis::Error<$result_type_error>>> {
                    if let Ok(v) = self.0.try_recv() {
                        Some(v)
                    } else {
                        None
                    }
                }

                fn handle_request(
                    mut commands: Commands,
                    mut q: Query<(Entity, &mut $task_name)>,
                    mut ev: EventWriter<$event_completed_name>,
                ) {
                    for (e, mut request) in q.iter_mut() {
                        if let Some(result) = request.poll() {
                            commands.entity(e).despawn();
                            ev.send($event_completed_name(result));
                        }
                    }
                }

                pub(crate) fn register(app: &mut App) {
                    app.add_event::<$event_completed_name>()
                        .add_systems(Update, Self::handle_request);
                }
            }
        };
    }

    pub use beam_request;
}
