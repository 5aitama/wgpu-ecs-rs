use std::any::TypeId;
use crate::core::{
    world::World,
    entity::Entity,
};

pub trait EntityQuery {
    /// Get entities where each of them have specified components.
    fn all(world: &World) -> Vec<Entity>;
    
    /// Get entities where each of them have minimun one of specified components.
    fn any(world: &World) -> Vec<Entity>;
}

macro_rules! entity_query {
    ( $( $name:ident )+ ) => {
        impl<$($name: std::any::Any),+> EntityQuery for ($($name,)+)
        {
            fn any(world: &World) -> Vec<Entity> {
                let type_ids = vec![$(TypeId::of::<$name>()),+];
                let mut entities: Vec<Entity> = Vec::with_capacity(type_ids.len());

                for (i, signature) in world.entities_signature.iter().enumerate() {
                    let mut entity: Option<Entity> = None;

                    for type_id in type_ids.iter() { 

                        // The component index.
                        let cindex = match world.components_index.get(&type_id) {
                            Some(index) => *index,
                            _ => {
                                continue;
                            },
                        };
    
                        if signature.test(cindex) {
                            entity = Some(i);
                            continue;
                        }
                    }

                    if entity.is_some() {
                        entities.push(entity.unwrap());
                    }
                }

                entities
            }

            fn all(world: &World) -> Vec<Entity> {
                let type_ids = vec![$(TypeId::of::<$name>()),+];
                let mut entities: Vec<Entity> = Vec::with_capacity(type_ids.len());

                for (i, signature) in world.entities_signature.iter().enumerate() {
                    let mut entity = Some(i);

                    for type_id in type_ids.iter() { 

                        // The component index.
                        let cindex = match world.components_index.get(&type_id) {
                            Some(index) => *index,
                            _ => {
                                entity = None;
                                break
                            },
                        };
    
                        if !signature.test(cindex) {
                            entity = None;
                            break;
                        }   
                    }

                    if entity.is_some() {
                        entities.push(entity.unwrap());
                    }
                }

                entities
            }
        }
    };
}

entity_query! { A }
entity_query! { A B }
entity_query! { A B C }
entity_query! { A B C D }
entity_query! { A B C D E }
entity_query! { A B C D E F }
entity_query! { A B C D E F G }
entity_query! { A B C D E F G H }
