use std::collections::HashMap;
use std::any::{Any, TypeId};

// Simplified ECS for maximum chaos, minimum abstraction
pub type EntityId = u32;

pub struct World {
    next_entity_id: EntityId,
    entities: Vec<EntityId>,
    components: HashMap<TypeId, Box<dyn ComponentStorage>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            next_entity_id: 0,
            entities: Vec::new(),
            components: HashMap::new(),
        }
    }

    pub fn create_entity(&mut self) -> EntityId {
        let id = self.next_entity_id;
        self.next_entity_id += 1;
        self.entities.push(id);
        id
    }

    pub fn add_component<T: 'static>(&mut self, entity: EntityId, component: T) {
        let type_id = TypeId::of::<T>();
        let storage = self.components
            .entry(type_id)
            .or_insert_with(|| Box::new(ComponentVec::<T>::new()));

        storage.as_any_mut()
            .downcast_mut::<ComponentVec<T>>()
            .unwrap()
            .insert(entity, component);
    }

    pub fn get_component<T: 'static>(&self, entity: EntityId) -> Option<&T> {
        let type_id = TypeId::of::<T>();
        self.components.get(&type_id)?
            .as_any()
            .downcast_ref::<ComponentVec<T>>()?
            .get(entity)
    }

    pub fn get_component_mut<T: 'static>(&mut self, entity: EntityId) -> Option<&mut T> {
        let type_id = TypeId::of::<T>();
        self.components.get_mut(&type_id)?
            .as_any_mut()
            .downcast_mut::<ComponentVec<T>>()?
            .get_mut(entity)
    }

    pub fn query<T: 'static>(&self) -> Vec<(EntityId, &T)> {
        let type_id = TypeId::of::<T>();
        if let Some(storage) = self.components.get(&type_id) {
            storage.as_any()
                .downcast_ref::<ComponentVec<T>>()
                .unwrap()
                .iter()
                .collect()
        } else {
            Vec::new()
        }
    }

    pub fn query_mut<T: 'static>(&mut self) -> Box<dyn Iterator<Item = (EntityId, &mut T)> + '_> {
        let type_id = TypeId::of::<T>();
        if let Some(storage) = self.components.get_mut(&type_id) {
            let storage = storage.as_any_mut()
                .downcast_mut::<ComponentVec<T>>()
                .unwrap();
            Box::new(storage.iter_mut())
        } else {
            Box::new(std::iter::empty())
        }
    }
}

trait ComponentStorage {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

struct ComponentVec<T> {
    data: HashMap<EntityId, T>,
}

impl<T> ComponentVec<T> {
    fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    fn insert(&mut self, entity: EntityId, component: T) {
        self.data.insert(entity, component);
    }

    fn get(&self, entity: EntityId) -> Option<&T> {
        self.data.get(&entity)
    }

    fn get_mut(&mut self, entity: EntityId) -> Option<&mut T> {
        self.data.get_mut(&entity)
    }

    fn iter(&self) -> impl Iterator<Item = (EntityId, &T)> {
        self.data.iter().map(|(&id, component)| (id, component))
    }

    fn iter_mut(&mut self) -> impl Iterator<Item = (EntityId, &mut T)> {
        self.data.iter_mut().map(|(&id, component)| (id, component))
    }
}

impl<T: 'static> ComponentStorage for ComponentVec<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}