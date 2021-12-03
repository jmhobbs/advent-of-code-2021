use std::fs;

pub fn read_input() -> Vec<String> {
  let contents = fs::read_to_string("input.txt")
    .expect("Something went wrong reading the input file");

  let mut lines: Vec<String> = Vec::new();

  for line in contents.split("\n") {
    if line != "" {
      lines.push(line.to_string())
    }
  }

  return lines;
}