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
    agent.food -= 0.1;
    agent.water -= 0.1;
}