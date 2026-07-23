use crate::{world_objects::FoodSource, world_objects::WaterSource};
use rand::RngExt;

pub struct Agent {
    pub position: (i32, i32),
    pub food: f32,
    pub water: f32,
}

pub fn move_agent(agent: &mut Agent, direction: (i32, i32)) {
    agent.position = (
        agent.position.0 + direction.0,
        agent.position.1 + direction.1,
    );
}

pub fn starve_agent(agent: &mut Agent) {
    agent.food -= 1.0;
    agent.water -= 2.0;
}

pub fn decide_agent(agent: &mut Agent, food_sources: &[FoodSource], water_sources: &[WaterSource]) {

    // Look for food
    if agent.food < 5.0 {
        println!("Agent at position: {:?} is hungry and will search for food.", agent.position);
        search_for_food(agent, food_sources);

        if(agent.food < 5.0) {
            println!("Agent at position: {:?} could not find food and is starving.", agent.position);
        }
        else {
            println!("Agent at position: {:?} found food and is no longer starving.", agent.position);
        }

    // Look for water
    } else if agent.water < 5.0 {
        println!("Agent at position: {:?} is thirsty and will search for water.", agent.position);
        search_for_water(agent, water_sources);

    // Random exploring
    } else {
        println!("Agent at position: {:?} is satisfied and will continue exploring.", agent.position);
        random_move(agent);
    }
}

pub fn search_for_food(agent: &mut Agent, food_sources: &[FoodSource]) {
    for food_source in food_sources {
        if agent.position == food_source.position {
            agent.food += food_source.food;
            println!("Agent at position: {:?} found food and now has {} food.", agent.position, agent.food);
            return;
        }
    }
    println!("Agent at position: {:?} did not find any food.", agent.position);

    random_move(agent);
}

pub fn search_for_water(agent: &mut Agent, water_sources: &[WaterSource]) {
    for water_source in water_sources {
        if agent.position == water_source.position {
            agent.water += water_source.water;
            println!("Agent at position: {:?} found water and now has {} water.", agent.position, agent.water);
            return;
        }
    }
    println!("Agent at position: {:?} did not find any water.", agent.position);
    random_move(agent);
}

pub fn random_move(agent: &mut Agent) {
    let mut rng = rand::rng();
    let direction = match rng.random_range(0..4) {
        0 => (1, 0),
        1 => (-1, 0),
        2 => (0, 1),
        _ => (0, -1),
    };
    move_agent(agent, direction);
}