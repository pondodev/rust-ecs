use crate::entity_manager::{ MAX_ENTITIES, Entity };
use crate::components::*;

pub struct ComponentManager {
    positions: [ PositionComponent; MAX_ENTITIES ],
    velocities: [ VelocityComponent; MAX_ENTITIES ],
    colors: [ ColorComponent; MAX_ENTITIES ],
}

impl ComponentManager {
    pub fn new() -> Self {
        ComponentManager {
            positions: [ PositionComponent { x: 0.0, y: 0.0 }; MAX_ENTITIES ],
            velocities: [ VelocityComponent { x: 0.0, y: 0.0 }; MAX_ENTITIES ],
            colors: [ ColorComponent { r: 0, g: 0, b: 0 }; MAX_ENTITIES ],
        }
    }

    pub fn register_position(&mut self, entity: Entity, x: f32, y: f32) {
        self.positions[entity] = PositionComponent { x, y };
    }

    pub fn register_velocity(&mut self, entity: Entity, x: f32, y: f32) {
        self.velocities[entity] = VelocityComponent { x, y };
    }

    pub fn register_color(&mut self, entity: Entity, r: u8, g: u8, b: u8) {
        self.colors[entity] = ColorComponent { r, g, b };
    }

    pub fn get_position(&mut self, entity: Entity) -> PositionComponent {
        self.positions[entity]
    }

    pub fn get_velocity(&mut self, entity: Entity) -> VelocityComponent {
        self.velocities[entity]
    }

    pub fn get_color(&mut self, entity: Entity) -> ColorComponent {
        self.colors[entity]
    }

    pub fn set_position(&mut self, entity: Entity, component: PositionComponent) {
        self.positions[entity] = component;
    }

    pub fn set_velocity(&mut self, entity: Entity, component: VelocityComponent) {
        self.velocities[entity] = component;
    }

    pub fn set_color(&mut self, entity: Entity, component: ColorComponent) {
        self.colors[entity] = component;
    }
}
