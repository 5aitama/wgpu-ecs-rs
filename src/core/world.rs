use std::{any::{Any, TypeId}, collections::{HashMap, HashSet}};

use bitset::BitSet;

use crate::core::{
    entity::Entity,
};

pub struct World {
    /// The list of all available entities in the world.
    pub available_entities: HashSet<Entity>,
    /// The list of all living entities in the world.
    pub living_entities: HashSet<Entity>,

    /// The list of all entities signatures in the world.
    pub entities_signature: Vec<BitSet>,

    /// The list that contains a component list.
    pub components: Vec<Vec<Option<Box<dyn Any>>>>,
    /// Hashmap that contain the index of component list by their type.
    pub components_index: HashMap<TypeId, usize>,
}

impl World {
    /// Create new `World`.
    /// # Arguments
    /// * `max_entities` - The maximum amount of entities in the world.
    /// * `max_components_type` - The maximum amount of component types in the world.
    pub fn new(max_entities: usize, max_components_type: usize) -> Self {

        let mut available_entities: HashSet<Entity> = HashSet::with_capacity(max_entities);
        let living_entities: HashSet<Entity> = HashSet::with_capacity(max_entities);

        let mut entities_signature: Vec<BitSet> = Vec::with_capacity(max_entities);

        for i in 0..max_entities {
            available_entities.insert(i);
            entities_signature.push(BitSet::with_capacity(max_components_type))
        }

        let mut components: Vec<Vec<Option<Box<dyn Any>>>> = Vec::with_capacity(max_components_type);
        
        for _ in 0..max_components_type {
            components.push(Vec::with_capacity(max_entities));
        }

        Self {
            available_entities,
            living_entities,
            entities_signature,
            components,
            components_index: HashMap::with_capacity(max_components_type),
        }
    }
}