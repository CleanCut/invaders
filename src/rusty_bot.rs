// An Agent that anlyses invaders position and takes a decision based on its heuritics for next move.
//
// States an agent can be in:
//   - For a Tabular States we do something like
//       The state of Agent is defined by considering following items
//       a. Invader informations: 
//         1. Invader count: ( 18(cols) * 4(rows) = 72(invaders))
//         2. Invader can be dead or alive so this becomes 2 ^ 72 = huge number!, 
//            so we can't use this for a tabular state
//         3. We can categorize state of invader wrt available invaders rows(i.e 4) and get tuple
//            which becomes (2^18, 2^18, 2^18, 2^18) still too much!
//         4. What we want is to give a sense of a way to let agent know where is the high density of invaders
//            present, this might not be perfect but shall approximate a state where our player is,
//            so now we can define this is by reducing possible invader state to 3 values
//            state 1: count of alive invaders to left of agent > count of alive invaders to right of agent
//            state 2: count of alive  invaders to left of agent == count of alive invaders to right of agent
//            state 3: count of alive invaders to left of agent < count of alive invaders to right of agent
//            This reduces number of states to 3 we loose a lot of information but give a approximate sense
//            to agent where our target is.
//       b. Direction in which invaders are going towards
//       c. Vertical Position of Invader:
//         1. Vertical distance of invaders from player that can be taken as 
//           vertical distance of last alive invader to player 
//           To keep this simple we can divide vertical distance into 4 or 5 checkpoints? i.e.  
//           if an alive invader crosses certain `y` height,this state updates.
//           so now we can track urgency/criticality of situation in which our player is not perfect but should work
//           but we still loose the information of how many invaders are there to be killed.
//
//   - For a continous State we can modify above states
//       Invader information:
//         1. Euclidean distance of each alive invador to player, if an invader is dead we make this state as 0(or -1?)
//       This much information should be enough for a continous state for invaders, but not going with this one
//       first just trying out tabular based agent.
//      
//    For states of environment we can proceed with a.4, b and c.1 which shall make a total number of possible states as
//    30 ( 5*3*2 )
//
// Actions an agent can take:
//   1. Make player shoot.
//   2. Move player Left.
//   3. Move Player Right.

use crate::{invaders::Invaders, player::Player};

pub struct Agent {
  q_table: Vec<Vec<f32>>,
  learning_rate: f32,
  gamma: f32, 
}

impl Agent {
    pub fn new(learning_rate: f32, gamma: f32) -> Self {
        Self { q_table: vec![vec![0.0; 3]; 30], learning_rate, gamma }
    }

    pub fn get_state(&self, invaders: &Invaders, player: &Player) -> i32 {
      /*
        extracts state of invaders wrt to agent, we need
        1. a way to derive state based on position of invaders wrt to player that should be either {0,1,2}
            0 -> more alive invaders to left of player
            1 -> equal number of alive player to left and right of player
            2 -> more alive invaders to right of player
        2. Direction in which agent is moving towards {0: left, 1: right}
        3. vertical cut off / location of invader wrt to playscreen
            5 cutoffs that can be defined as are
            so currently we have a hard cut limit on 9 so our last alive invader is at 9,
            arbitarily defining 5 more states as
            0 -> `>=9 && < 13`
            1 -> `>=13 && < 17`
            2 -> `>=17 && < 21`
            3 -> `>=21 && < 25`
            4 -> `>=25 && < 30`
        there are 15 states that an agent can be in, we can get the final using following formula
          15 * direction + (5 * state_player_relative_position + state_invader_vertical_postion)
      */
      let player_position = (*player).get_x();
      // Get the player relative position wrt maximum alive invaders
      // if a invader is dead we it is already remvoed from army vector
      let alive_right_count = invaders.army.iter().filter(
        // Get all the invaders to the right of the player
        |invader| {invader.x > player_position}
      ).count();
      let alive_left_count = invaders.army.iter().filter(
        // Get all the invaders to the left of the player
        |invader| {invader.x < player_position}
      ).count();
      let state_player_relative_position;
      if alive_left_count == alive_right_count {
        state_player_relative_position = 0;
      } else if alive_left_count <= alive_right_count {
        state_player_relative_position = 1;
      } else {
        state_player_relative_position = 2;
      }
      // let state_player_relative_position = max(alive_left_count, alive_right_count) as i32;
      let direction = match invaders.get_direction() {
        x => {
          if x == -1 {0}
          else {1}
        },
      };
      
      let state_invader_vertical_postion = match invaders.army.iter().map(
        |invader| {invader.y}
      ).max() {
        Some(x) => {
          if x >=9 && x < 13 {0}
          else if x >=13 && x < 17 {1}
          else if x >=17 && x < 21 {2}
          else if x >=21 && x < 25 {3}
          else {4}
        },
        None => 0,
      };
      15 * direction + (5 * state_player_relative_position + state_invader_vertical_postion)
    }
    
    pub fn act(&self, current_state: i32, attempt_number: i32) -> i32 {
      // figure out best possible action to take
      // self.q_table[current_state as usize].iter().map(
      //   |value| {value + (((1 / (1 + attempt_number)) as f32) + rand::random::<f32>())}
      // ).into_iter().enumerate().max_by_key(|(_, value)| value).map(|(idx, _)| idx);
      let mut best_action = 0;
      let mut max_value = f32::MIN;
      for value in self.q_table[current_state as usize].iter().map(
        |value| {value + (((1 / (1 + attempt_number)) as f32) + rand::random::<f32>())}
      ).into_iter().enumerate() {
        if value.1 > max_value {
          max_value = value.1;
          best_action = value.0 as i32;
        }
      }
      // let max_position = self.q_table[current_state as usize].iter().position(
      //   |&r| { r == max_value}
      // ).unwrap_or(usize::MAX);
      // assert!(max_position >= 0 && max_position <= 2, "{} expected to be in range on 0..=2", max_position);
      best_action
    }

    pub fn learn(&mut self, current_state: i32, action: i32, reward: f32, new_state: i32) {
      let max_future_reward = self.q_table[new_state as usize].iter().map(
        |value| {*value}
      ).into_iter().reduce(f32::max).unwrap();
      self.q_table[current_state as usize][action as usize] += self.learning_rate * (
        reward + (
          self.gamma * (max_future_reward - self.q_table[current_state as usize][action as usize]
          )
        )
      ); 
    }

    pub fn reset(&self) -> Self {
      Agent::new(self.learning_rate, self.gamma)
    }

}