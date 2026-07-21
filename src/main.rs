mod agent;
use agent::Agent;

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
            println!(
                "Agent at position: {:?}, food: {}, water: {}",
                agent.position, agent.food, agent.water
            );
        }

        if day == 30 {
            break;
        }
    }
}
