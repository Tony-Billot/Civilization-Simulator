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

    let content = fs::read_to_string("simulation.config");
    print!("Reading simulation.config file...\n");
    print!("File content: {}\n", content.as_ref().unwrap());

    world.agents.push(Agent {
        position: (0, 0),
        food: 10.0,
        water: 10.0,
    });

    world.agents.push(Agent {
        position: (1, 2),
        food: 10.0,
        water: 10.0,
    });

    world.agents.push(Agent {
        position: (3, 4),
        food: 10.0,
        water: 10.0,
    });

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

        if world.day == 30 {
            println!("Simulation ended after 30 days. Everyone survived !");
            break;
        }
    }
}