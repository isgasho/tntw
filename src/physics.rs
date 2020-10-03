use bevy::prelude::*;
use bevy_rapier2d::{
    physics::RigidBodyHandleComponent, rapier::dynamics::JointSet,
    rapier::dynamics::RigidBodyHandle, rapier::dynamics::RigidBodySet,
    rapier::geometry::BroadPhase, rapier::geometry::ColliderSet, rapier::geometry::NarrowPhase,
    rapier::pipeline::PhysicsPipeline,
};

use bevy_rapier2d::physics::{EventQueue};
use bevy_rapier2d::rapier::dynamics::{RigidBodyBuilder};
use bevy_rapier2d::rapier::geometry::{Proximity, ColliderBuilder};

use std::collections::HashMap;
pub struct BodyHandleToEntity(pub HashMap<RigidBodyHandle, Entity>);
pub struct EntityToBodyHandle(pub HashMap<Entity, RigidBodyHandle>);

use crate::DebugTimer;


pub fn unit_proximity_interaction_system(
    events: Res<EventQueue>,
    // unit_query: 
) {
    // we can can ignore contact events because we are only using sensors, not
    // rigid contactors
    while let Ok(contact_event) = events.contact_events.pop() {
        panic!();
        log::warn!("ASD");
    }
    
    // prox events are triggered between sensors and colliders (sensor or not)
    while let Ok(prox_event) = events.proximity_events.pop() {  
        panic!();
        log::warn!("ASDaaa");
        // we can ignore WithinMargin because we don't need any special behaviour for that case
        // new_status is guaranteed to be != prev_status
        match prox_event.new_status {
            Proximity::Disjoint => {
                log::warn!("disjoint");
            },
            Proximity::Intersecting => {
                log::warn!("intersection");
            },
            Proximity::WithinMargin => (),
        } 
    }
}

/// Keeps BodyHandleToEntity resource in sync.
// TODO: handle removals.
pub fn body_to_entity_system(
    mut bh_to_e: ResMut<BodyHandleToEntity>,
    mut e_to_bh: ResMut<EntityToBodyHandle>,
    mut added: Query<(Entity, Added<RigidBodyHandleComponent>)>,
) {
    for (entity, body_handle) in &mut added.iter() {
        log::debug!("new rigid body");
        bh_to_e.0.insert(body_handle.handle(), entity);
        e_to_bh.0.insert(entity, body_handle.handle());
    }
}


pub fn physics_debug_system(
    time: Res<Time>,
    mut debug_timer: ResMut<DebugTimer>,
    mut bodies: ResMut<RigidBodySet>,
    mut colliders: ResMut<ColliderSet>,
    mut query: Query<(Entity, &RigidBodyHandleComponent)>,
) {
    debug_timer.0.tick(time.delta_seconds);
    if debug_timer.0.finished {
        // log::debug!("asd");
        for (entity, body_handle) in &mut query.iter() {
            let mut body = bodies.get_mut(body_handle.handle()).expect("body");
            // body positions appear to be correct, and we have two colliders in existance...
            // why u no collide?
            log::trace!("entity {:?} at ({}, {})", entity, body.position.translation.x, body.position.translation.y);
        }
        log::trace!("#colliders: {}", colliders.len());
        log::trace!("#bodies: {}", bodies.len());
        for (idx, collider) in colliders.iter() {
            log::trace!("collider {:?} at ({}, {})", idx, collider.position().translation.x, collider.position().translation.y);
        }
    }
}

/// Detects when a RigidBodyHandle is removed from an entity, as it despawns
/// And inform rapier about the removal
pub fn remove_rigid_body_system(
    mut pipeline: ResMut<PhysicsPipeline>,
    mut broad_phase: ResMut<BroadPhase>,
    mut narrow_phase: ResMut<NarrowPhase>,
    mut bodies: ResMut<RigidBodySet>,
    mut colliders: ResMut<ColliderSet>,
    mut joints: ResMut<JointSet>,
    mut e_to_bh: ResMut<EntityToBodyHandle>,
    mut bh_to_e: ResMut<BodyHandleToEntity>,
    query: Query<&RigidBodyHandleComponent>,
) {
    for entity in query.removed::<RigidBodyHandleComponent>().iter() {
        log::debug!("removed rigid body");
        let handle = e_to_bh.0.get(entity).unwrap();
        pipeline.remove_rigid_body(
            *handle,
            &mut broad_phase,
            &mut narrow_phase,
            &mut bodies,
            &mut colliders,
            &mut joints,
        );
        bh_to_e.0.remove(handle);
        e_to_bh.0.remove(entity);
    }
}