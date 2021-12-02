use std::fs;

pub fn count_increases(samples:Vec<i16>) -> u16 { 
  let mut last: i16 = samples[0];
  let mut count: u16 = 0;

  for sample in samples {
    if sample > last {
      count = count + 1
    }
    last = sample
  }

  count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_a() {
      let samples: Vec<i16> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
      assert_eq!(count_increases(samples), 7)
    }
}

fn main() {
  let contents = fs::read_to_string("input.txt")
      .expect("Something went wrong reading the input file");

  let mut samples : Vec<i16> = Vec::new();

  for line in contents.split("\n") {
    if line != "" {
      samples.push(line.parse::<i16>().unwrap());
    }
  }

  print!("A: {}\n", count_increases(samples))
}

