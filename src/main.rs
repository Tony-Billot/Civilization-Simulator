mod agent;
use agent::Agent;
use agent::move_agent;
use agent::starve_agents;

fn main() {
    let mut day: i32 = 0;
    let mut agents: Vec<Agent> = Vec::new();

    agents.push(Agent {
        position: (0, 0),
        food: 10.0,
        water: 10.0,
    });

    loop {
        day += 1;
        println!("We are in the day: {}", day);

        for agent in &mut agents {
            println!("Agent at position: {:?}, food: {}, water: {}", agent.position, agent.food, agent.water);
        }
        starve_agents(&mut agents);

        if day == 30 {
            break;
        }
    }
}
