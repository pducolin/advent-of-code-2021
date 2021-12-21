use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Player {
  position: i64,
  points: i64,
}

impl Player {
  fn parse(data: &String) -> Self {
    Self {
      position: data.chars().last().unwrap().to_digit(10).unwrap() as i64,
      points: 0
    }
  }

  fn play(&mut self, dice: &mut Dice) {
    let mut sum = 0;
    let mut sums: Vec<String> = Vec::with_capacity(3);
    for _ in 0..3{
      sum += dice.roll();
      sums.push(dice.value.to_string());
    }
    println!("{:?}", sums.join("+"));
    self.position += sum;
    if self.position > 10 {
      self.position %= 10;
      if self.position == 0 {
        self.position = 10;
      } 
    }
    self.points += self.position;
  }

  fn play_quantum(&self, dice: &QuantumDice) -> HashMap<Player, u64> {
    let mut quantum_players: HashMap<Player, u64> = HashMap::new();

    for (dice_sum, count) in dice.values.clone() {
      let mut position = self.position + dice_sum;
      if position > 10 {
        position %= 10;
        if position == 0 {
          position = 10;
        } 
      }
      let points = self.points + position;
      let player = Player{
        points,
        position,
      };
      *quantum_players.entry(player).or_insert(0) += count;
    }
    return quantum_players;
  }
}

struct QuantumDice {
  values: HashMap<i64, u64>,
}

impl QuantumDice {
  fn new() -> Self {
    let mut values: HashMap<i64, u64> = HashMap::new();
    for i in 0..3 {
      for j in 0..3 {
        for v in 0..3 {
          let sum = i + 1 + j + 1 + v + 1;
          *values.entry(sum).or_insert(0) += 1;
        }
      }
    }
    Self {
      values
    }
  }
}

#[derive(Debug)]
struct Dice {
  value: i64,
  tot_rolls: i64,
}

impl Dice {
  fn new() -> Self {
    Self {
      value: 0,
      tot_rolls: 0,
    }
  }

  fn roll(&mut self) -> i64 {
    self.value += 1;
    if self.value > 100 {
      self.value -= 100;
    }
    self.tot_rolls += 1;
    return self.value;
  }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Universe {
  player_1: Player,
  player_2: Player,
}

fn solution_1(data: &Vec<String>) -> i64 {
  let mut players: Vec<Player> = Vec::with_capacity(2);

  for d in data {
    players.push(Player::parse(d));
  }

  for p in &players {
    println!("{:?}", p);
  }

  let mut dice = Dice::new();

  loop {
    for i in 0..players.len() {
      let player = players.get_mut(i).unwrap();
      player.play(&mut dice);
      println!("Player {}: {:?}", i + 1, player);
      if player.points >= 1000 {
        println!("Winner !");
        for p in &players {
          println!("{:?}", p);
        }
        println!("{:?}", dice);
        return players.iter().filter(|&p|p.points < 1000).next().unwrap().points * dice.tot_rolls;
      }
    }
  }
}

fn solution_2(data: &Vec<String>) -> u64 {
  let mut players: Vec<Player> = Vec::with_capacity(2);

  for d in data {
    players.push(Player::parse(d));
  }

  for p in &players {
    println!("{:?}", p);
  }

  let quantum_dice = QuantumDice::new();
  let mut winner_count: Vec<u64> = vec![0;2];

  let mut universes: HashMap<Universe, u64> = HashMap::new();

  universes.insert(Universe{player_1: players[0].clone(), player_2: players[1].clone()}, 1);

  while !universes.is_empty() {
    // play player 1
    let mut new_universes: HashMap<Universe, u64> = HashMap::new();
    for (universe, count) in universes.clone() {
      let player = universe.player_1;
      let parallel_players = player.play_quantum(&quantum_dice);
      for (p, p_count) in parallel_players {
        if p.points >= 21 {
          // winner, universe is over
          winner_count[0] += p_count*count;
          continue;
        }
        let uni = Universe {
          player_1: p.clone(),
          player_2: universe.player_2.clone() 
        };
        *new_universes.entry(uni).or_insert(0) += p_count * count;
      }
    } 

    // play player 2
    universes.clear();
    for (universe, count) in new_universes {
      let player = universe.player_2.clone();
      let parallel_players = player.play_quantum(&quantum_dice);
      for (p, p_count) in parallel_players {
        if p.points >= 21 {
          // winner, universe is over
          winner_count[1] += p_count*count;
          continue;
        }
        let uni = Universe {
          player_2: p.clone(),
          player_1: universe.player_1.clone() 
        };
        *universes.entry(uni).or_insert(0) += p_count * count;
      }
    }
  }
  winner_count.sort();
  return winner_count.last().unwrap().clone();
}

fn main() {
  let test = false;
  let mut file_path: String = "inputs/day21".to_string();
  let mut emoji: String = "🎉".to_string();
  if test {
    file_path += ".test";
    emoji = "🧪".to_string();
  }
  file_path += ".txt";

  let file = File::open(file_path).unwrap();
  let data: Vec<String> = io::BufReader::new(file)
              .lines()
              .map(|line|line.unwrap())
              .collect();

  println!(r#"{:?} Part 1 result is {:?}"#, emoji, solution_1(&data));

  println!(r#"{:?} Part 2 test result is {:?}"#, emoji, solution_2(&data));
}
