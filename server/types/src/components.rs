mod marker;
mod network;
mod physics;
mod serialize;

pub use marker::*;
pub use serialize::*;

pub use feather_core::inventory::Inventory;
pub use network::{Network, ServerToWorkerMessage, WorkerToServerMessage};
pub use physics::{AABBExt, Physics, PhysicsBuilder};
pub use uuid::Uuid;

use ahash::AHashSet;
use dashmap::DashMap;
use feather_core::inventory::SlotIndex;
use feather_core::util::{ChunkPosition, Position};
use fecs::Entity;

/// The item an entity is currently holding.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct HeldItem(pub SlotIndex);

/// An entity's name.
#[derive(Debug, Clone, Default)]
pub struct Name(pub String);

/// Position of an entity on the previous tick.
#[derive(Copy, Clone, Debug)]
pub struct PreviousPosition(pub Position);

/// An entity's velocity.
#[derive(Copy, Clone, Debug)]
pub struct Velocity(pub glm::DVec3);

impl Default for Velocity {
    fn default() -> Self {
        Velocity(glm::vec3(0.0, 0.0, 0.0))
    }
}

/// Velocity of an entity on the previous tick.
#[derive(Copy, Clone, Debug)]
pub struct PreviousVelocity(pub glm::DVec3);

impl Default for PreviousVelocity {
    fn default() -> Self {
        PreviousVelocity(Velocity::default().0)
    }
}

/// Network ID of an entity.
#[derive(Copy, Clone, Debug)]
pub struct NetworkId(pub i32);

/// Component which stores which
/// chunks a given entity has a holder
/// on.
///
/// Although this information is also
/// stored in the `ChunkHolders` resource,
/// using this component allows for efficiently
/// finding which chunks a given entity has
/// a hold on, rather than having
/// to linear search all chunks (obviously ridiculous).
#[derive(Default)]
pub struct ChunkHolder {
    pub holds: AHashSet<ChunkPosition>,
}

/// Component containing the last sent positions of all entities for a given client.
/// This component is used to determine
/// the relative movement for an entity.
#[derive(Default, Debug)]
pub struct LastKnownPositions(pub DashMap<Entity, Position>);

/// Profile properties of a player.
#[derive(Debug, Clone)]
pub struct ProfileProperties(pub Vec<mojang_api::ProfileProperty>);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct ParticleCount(pub u32);
