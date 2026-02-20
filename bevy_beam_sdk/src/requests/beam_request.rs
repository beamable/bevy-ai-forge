use crate::config::BeamableConfigResource;
use crate::slot::prelude::{BeamSlot, TokenStorage};
use bevy::prelude::*;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

#[derive(Deref, DerefMut)]
pub struct BeamReceiver<S, E>
where
    S: Debug + Send + 'static,
    E: Debug + Send + 'static,
{
    #[deref]
    pub receiver: crossbeam_channel::Receiver<Result<S, E>>,
    pub user_tag: Option<i64>,
    pub entity: Entity,
}

pub trait BeamEventFactory<S, E>:
    EntityEvent + Deref<Target = Result<S, E>> + for<'a> Event<Trigger<'a>: Default>
where
    S: Debug + Send + 'static,
    E: Debug + Send + 'static,
{
    fn for_entity(value: Result<S, E>, entity: Entity) -> Self;
}

pub trait BeamRequestResource<S, E>:
    Resource + FromWorld + Sized + DerefMut<Target = Vec<BeamReceiver<S, E>>>
where
    S: Debug + Send + 'static,
    E: Debug + Send + 'static,
{
    type Event: BeamEventFactory<S, E>;

    fn add(
        &mut self,
        receiver: crossbeam_channel::Receiver<Result<S, E>>,
        user_tag: Option<i64>,
        entity: Entity,
    ) {
        self.push(BeamReceiver::<S, E> {
            receiver,
            user_tag,
            entity,
        })
    }

    fn handle_requests(mut commands: Commands, mut r: ResMut<Self>) {
        r.retain_mut(|request| {
            if let Ok(result) = request.try_recv() {
                trace!("{:?}: {:?}", &request.user_tag, &result);
                let event = Self::Event::for_entity(result, request.entity);
                commands.trigger(event);
                false // Remove from vector
            } else {
                true // Keep in vector
            }
        });
    }
}

pub trait BeamRequest<S, E>
where
    S: Debug + Send + 'static,
    E: Debug + Send + 'static,
{
    type Config: Debug + Clone + Send + 'static;
    type ArgsType: Clone + Send + 'static;
    fn config_from(client: &super::client::ReqwestClient) -> Self::Config;

    fn call<T>(
        data: Self::ArgsType,
        request_client: &super::client::ReqwestClient,
        handler: &mut T,
        entity: Entity,
    ) where
        T: BeamRequestResource<S, E>;

    fn make_request_client(world: &World, entity: Entity) -> super::client::ReqwestClient {
        let token = match world.get_entity(entity).map(|s| s.get::<TokenStorage>()) {
            Ok(Some(s)) => s.to_owned(),
            _ => Default::default(),
        };
        let gamer_tag = match world.get_entity(entity).map(|s| s.get::<BeamSlot>()) {
            Ok(Some(s)) => *s.gamer_tag,
            _ => Default::default(),
        };
        let config = world
            .get_resource::<BeamableConfigResource>()
            .expect("No Beamable Config available");
        let routing = world.get_resource::<crate::config::RoutingMapKey>();

        super::client::ReqwestClient::new(config, routing, gamer_tag, &token)
    }
}
