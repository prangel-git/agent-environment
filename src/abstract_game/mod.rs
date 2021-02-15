pub mod environment;
pub mod agent;




// Plays a game in Envirnment 'env', and agents in 'agents'.
pub fn play_game<Action, AgentId, T, R> (
  env: &mut T, 
  agents: &mut Vec<&mut R>
) -> Vec<(AgentId, Action)> 
where
AgentId: Eq, 
T: environment::Environment<Action, AgentId>,
R: agent::Agent<Action, AgentId, T>
{
  let mut game_log = Vec::new();

  while !env.is_terminal() {    
    
    for agent in agents.iter_mut() {
      let identity = agent.identity();
      if identity == env.turn() {
        let action = agent.action(env);
        env.update(&identity, &action);

        game_log.push((identity, action));

        if env.is_terminal() { break }
      }
    }
  }
  return game_log;
}