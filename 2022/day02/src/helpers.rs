use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum PlayableOption {
  Rock,
  Paper,
  Scissors,
}

impl PlayableOption {
  pub fn get_points(self: &Self) -> i32 {
    match self {
      PlayableOption::Rock => 1,
      PlayableOption::Paper => 2,
      PlayableOption::Scissors => 3,
    }
  }

  pub fn pick_option_to_achieve_result(opponent_option: &PlayableOption, round_result: &RoundResult) -> Self {
    match (opponent_option, round_result) {
      (PlayableOption::Rock, RoundResult::Defeat)
      | (PlayableOption::Scissors, RoundResult::Draw)
      | (PlayableOption::Paper, RoundResult::Victory) => PlayableOption::Scissors,

      (PlayableOption::Paper, RoundResult::Defeat)
      | (PlayableOption::Rock, RoundResult::Draw)
      | (PlayableOption::Scissors, RoundResult::Victory) => PlayableOption::Rock,

      (PlayableOption::Scissors, RoundResult::Defeat)
      | (PlayableOption::Paper, RoundResult::Draw)
      | (PlayableOption::Rock, RoundResult::Victory) => PlayableOption::Paper,
    }
  }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParsePlayableOptionError;

impl FromStr for PlayableOption {
  type Err = ParsePlayableOptionError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "A" | "X" => Ok(PlayableOption::Rock),
      "B" | "Y" => Ok(PlayableOption::Paper),
      "C" | "Z" => Ok(PlayableOption::Scissors),
      _ => Err(ParsePlayableOptionError),
    }
  }
}

#[derive(Debug, PartialEq)]
pub enum RoundResult {
  Defeat,
  Draw,
  Victory,
}

impl RoundResult {
  pub fn get_points(self: &Self) -> i32 {
    match self {
      RoundResult::Defeat => 0,
      RoundResult::Draw => 3,
      RoundResult::Victory => 6,
    }
  }

  pub fn get_round_result(me: &PlayableOption, opponent: &PlayableOption) -> Self {
    match (me, opponent) {
      (PlayableOption::Rock, PlayableOption::Paper)
      | (PlayableOption::Paper, PlayableOption::Scissors)
      | (PlayableOption::Scissors, PlayableOption::Rock) => RoundResult::Defeat,

      (PlayableOption::Rock, PlayableOption::Scissors)
      | (PlayableOption::Paper, PlayableOption::Rock)
      | (PlayableOption::Scissors, PlayableOption::Paper) => RoundResult::Victory,

      (PlayableOption::Rock, PlayableOption::Rock)
      | (PlayableOption::Paper, PlayableOption::Paper)
      | (PlayableOption::Scissors, PlayableOption::Scissors) => RoundResult::Draw,
    }
  }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseRoundResultError;

impl FromStr for RoundResult {
  type Err = ParseRoundResultError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    if s.len() > 1 {
      let options: Vec<&str> = s.split(' ').collect();
      let opponent_option: PlayableOption = options[0].parse().unwrap();
      let my_option: PlayableOption = options[1].parse().unwrap();

      return Ok(RoundResult::get_round_result(&my_option, &opponent_option));
    }

    match s {
      "X" => Ok(RoundResult::Defeat),
      "Y" => Ok(RoundResult::Draw),
      "Z" => Ok(RoundResult::Victory),
      _ => Err(ParseRoundResultError),
    }
  }
}