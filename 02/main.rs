use std::fs;

enum Direction {
  Up,
  Down,
  Forward,
  Invalid,
}

struct Instruction(Direction, i32);

fn string_to_instruction(input: String) -> Instruction {
  let mut split = input.split_whitespace();

  let direction = match split.next() {
    Some("forward") => Direction::Forward,
    Some("up") => Direction::Up,
    Some("down") => Direction::Down,
    _ => Direction::Invalid,
  };

  return Instruction(direction, split.next().unwrap().parse::<i32>().unwrap());
}

fn navigate(instructions:&Vec<Instruction>) -> (i32, i32) {
  let mut x: i32 = 0;
  let mut z: i32 = 0;

  for instruction in instructions {
    match instruction.0 {
      Direction::Forward => x = x + instruction.1,
      Direction::Up => z = z - instruction.1,
      Direction::Down => z = z + instruction.1,
      Direction::Invalid => print!("Invalid Direction"),
    };
  }

  return (x, z);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_string_to_instruction() {
    assert!(matches!(string_to_instruction("forward 5".to_string()), Instruction(Direction::Forward, 5)));
    assert!(matches!(string_to_instruction("down 3".to_string()), Instruction(Direction::Down, 3)));
    assert!(matches!(string_to_instruction("up 9".to_string()), Instruction(Direction::Up, 9)));
  }

  #[test]
  fn test_example_a() {
    let instructions: Vec<Instruction> = vec![Instruction(Direction::Forward, 5), Instruction(Direction::Down, 5), Instruction(Direction::Forward, 8), Instruction(Direction::Up, 3), Instruction(Direction::Down, 8), Instruction(Direction::Forward, 2)];
    assert_eq!(navigate(&instructions), (15, 10));
  }
}

fn main() {
  let contents = fs::read_to_string("input.txt")
      .expect("Something went wrong reading the input file");

  let mut instructions: Vec<Instruction> = Vec::new();

  for line in contents.split("\n") {
    if line != "" {
      instructions.push(string_to_instruction(line.to_string()))
    }
  }

  let result = navigate(&instructions);

  print!("A: {}\n", result.0 * result.1);
}
