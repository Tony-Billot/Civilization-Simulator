mod agent;
use agent::Agent;
use agent::starve_agent;

mod world;
use crate::agent::decide_agent;

mod world_objects;

use std::fs;
use serde_json::Value;


fn main() {
    let mut world: world::World = world::createDebugWorld();

    let content = fs::read_to_string("simulation.config").unwrap();
    print!("Reading simulation.config file...\n");
    print!("File content: {}\n", content);
    let json: serde_json::Value = serde_json::from_str(&content).unwrap();

    println!("{}", json["agents"]);

    let days: i32 = json["days"].as_i64().unwrap() as i32;

    let agents = json["agents"].as_array().unwrap();
    for agent in agents {
        let position = agent["position"].as_array().unwrap();
        let x = position[0].as_i64().unwrap() as i32;
        let y = position[1].as_i64().unwrap() as i32;
        let food = agent["food"].as_f64().unwrap() as f32;
        let water = agent["water"].as_f64().unwrap() as f32;

        world.agents.push(Agent {
            position: (x, y),
            food,
            water,
        });
    }

    loop {
        world.day += 1;
        println!("We are in the day: {}", world.day);
        for agent in &mut world.agents {
            println!("Agent at position: {:?}, food: {}, water: {}", agent.position, agent.food, agent.water);
            starve_agent(agent);
            decide_agent(agent, &world.food_sources, &world.water_sources);
        }


        world.agents.retain(|agent| {
        if agent.food <= 0.0 || agent.water <= 0.0 {
            println!("Agent at position: {:?} has died.",agent.position);
            false
        } else {
            true
        }
        });

        if(world.agents.is_empty()) {
            println!("All agents have died.");
            println!("Simulation ended on day: {}", world.day);
            break;
        }

        if world.day == days {
            println!("Simulation ended after {} days. Everyone survived !", days);
            break;
        }
    }
}