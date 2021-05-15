
use crate::core::{
    world::World,
    entity::Entity,
};

pub struct EntityManager;

/// Manage entities of a world.
impl EntityManager {

    /// Create an entity.
    /// # Arguments
    /// * `world` - The world
    pub fn create_entity(world: &mut World) -> Option<Entity> {
        let entity = match world.available_entities.iter().next() {
            Some(e) => { Some(*e) },
            _ => None,
        };

        if entity.is_none() {
            return None
        }

        if world.living_entities.insert(entity.unwrap()) {
            world.available_entities.take(&entity.unwrap())
        } else {
            None
        }
    }

    /// Destroy an entity.
    /// # Arguments
    /// * `entity` - The entity to destroy.
    /// * `world` - The entity world.
    pub fn destroy_entity(entity: Entity, world: &mut World) -> Option<()> {

        if !world.living_entities.contains(&entity) {
            return None
        }

        match world.living_entities.take(&entity) {
            Some(_) => { 
                if world.available_entities.insert(entity) {
                    world.entities_signature[entity].reset();
                    Some(())
                } else {
                    None
                }
            },
            _ => None,
        }
    }

}