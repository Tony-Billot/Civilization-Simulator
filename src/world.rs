use crate::agent::Agent;
use crate::world_objects::FoodSource;

pub struct World {
    pub day: i32,
    pub agents: Vec<Agent>,
    pub food_sources: Vec<FoodSource>,
}

pub fn createDebugWorld() -> World{
    let mut  world = World {
        day: 0,
        agents: Vec::new(),
        food_sources: Vec::new(),
    };

    world.food_sources.push(FoodSource {
        position: (5, 5),
        food: 100.0,
    });

    world
}