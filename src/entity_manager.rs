use std::collections::VecDeque;

pub type Entity = usize;

pub const MAX_ENTITIES: usize = 1000;

pub struct EntityManager {
    available_entities: VecDeque<Entity>,
}

impl EntityManager {
    pub fn new() -> Self {
        let mut available_entities = VecDeque::new();

        for i in 0..MAX_ENTITIES {
            available_entities.push_back(i as Entity);
        }

        EntityManager { available_entities }
    }

    pub fn create_entity(&mut self) -> Option<Entity> {
        self.available_entities.pop_front()
    }

    pub fn destroy_entity(&mut self, entity: Entity) {
        self.available_entities.push_back(entity)
    }
}