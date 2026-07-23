use crate::agent::Agent;
use crate::world_objects::FoodSource;
use crate::world_objects::WaterSource;

pub struct World {
    pub day: i32,
    pub agents: Vec<Agent>,
    pub food_sources: Vec<FoodSource>,
    pub water_sources: Vec<WaterSource>,
}

pub fn createDebugWorld() -> World{
    let mut  world = World {
        day: 0,
        agents: Vec::new(),
        food_sources: Vec::new(),
        water_sources: Vec::new(),
    };

    world.food_sources.push(FoodSource {
        position: (5, 5),
        food: 100.0,
    });

    world.water_sources.push(WaterSource {
        position: (10, 10),
        water: 100.0,
    });

    return world;
}