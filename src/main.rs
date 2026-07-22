mod agent;
use agent::Agent;
use agent::starve_agent;

mod world;
use world::createDebugWorld;

use crate::agent::decide_agent;
use crate::agent::move_agent;
use crate::agent::random_move;

mod world_objects;


fn main() {
    let mut day: i32 = 0;

    let mut world: world::World = world::createDebugWorld();

    world.agents.push(Agent {
        position: (0, 0),
        food: 10.0,
        water: 10.0,
    });

    loop {
        day += 1;
        println!("We are in the day: {}", day);
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
            println!("Simulation ended on day: {}", day);
            break;
        }

        if day == 30 {
            println!("Simulation ended after 30 days. Everyone survived !");
            break;
        }
    }
}