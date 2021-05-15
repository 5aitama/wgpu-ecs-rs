use std::any::{Any, TypeId};

use crate::core::{
    world::World,
    entity::Entity,
};

pub struct ComponentManager;

impl ComponentManager {

    /// Add a component to an entity.
    pub fn add_component<T: Any>(component: T, entity: Entity, world: &mut World) {
        
        if !world.living_entities.contains(&entity) {
            return
        }

        let type_id = TypeId::of::<T>();
        
        // Ensure that the component array of the current component type exist.
        if world.components_index.get(&type_id).is_none() {
            world.components_index.insert(type_id, world.components.len());
            world.components.push(Vec::with_capacity(world.living_entities.capacity()));
        }

        // Retrieve the component array index...
        let index = *world.components_index.get(&type_id).unwrap();

        if world.components[index].is_empty() {
            for _ in 0..world.components[index].capacity() {
                world.components[index].push(None);
            }
        }

        world.components[index][entity] = Some(Box::new(component));

        // Update the entity signature.
        world.entities_signature[entity].set(index, true);
    }

    /// Remove a component to an entity
    pub fn del_component<T: Any>(entity: Entity, world: &mut World) -> bool {
        if !world.living_entities.contains(&entity) {
            return false
        }

        let type_id = TypeId::of::<T>();

        // Check if the component exist
        if world.components_index.get(&type_id).is_none() {
            return false
        }

        let index = *world.components_index.get(&type_id).unwrap();

        // Check if the entity has this component.
        if !world.entities_signature[entity].test(index) {
            return false
        }

        // Update the entity signature.
        world.entities_signature[entity].set(index, false);

        // NOTE: We don't drop() the component!

        true
    }

    pub fn get_component<T: Any>(entity: Entity, world: &World) -> Option<&T> {

        if !world.living_entities.contains(&entity) {
            return None
        }

        let type_id = TypeId::of::<T>();
        
        let index = match world.components_index.get(&type_id) {
            Some(it) => it,
            _ => return None,
        };

        let component_box = match &world.components[*index][entity] {
            Some(boxed) => boxed,
            _ => return None,
        };

        component_box.downcast_ref::<T>()
    }

    pub fn get_component_mut<T: Any>(entity: Entity, world: &mut World) -> Option<&mut T> {

        if !world.living_entities.contains(&entity) {
            return None
        }

        let type_id = TypeId::of::<T>();
        
        let index = match world.components_index.get(&type_id) {
            Some(it) => it,
            _ => return None,
        };

        let component_box = match &mut world.components[*index][entity] {
            Some(boxed) => boxed,
            _ => return None,
        };

        component_box.downcast_mut::<T>()
    }
}