use std::fs;

pub fn count_increases(samples:&Vec<i16>) -> u16 { 
  let mut last: i16 = samples[0];
  let mut count: u16 = 0;

  for sample in samples {
    if *sample > last {
      count = count + 1
    }
    last = *sample
  }

  count
}

pub fn sliding_window(samples:&Vec<i16>) -> Vec<i16> {
  let mut outputs : Vec<i16> = Vec::new();

  let mut i = 0;
  let mut window:[i16; 3] = [0; 3];

  for sample in samples {
    window[i%3] = *sample;
    i = i + 1;
    if i >= 3 {
      outputs.push(window[0]+window[1]+window[2])
    }
  }
  return outputs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_a() {
      let samples: Vec<i16> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
      assert_eq!(count_increases(&samples), 7)
    }

    #[test]
    fn test_example_b() {
      let samples: Vec<i16> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
      assert_eq!(count_increases(&sliding_window(&samples)), 5)
    }

    #[test]
    fn test_sliding_window() {
      let samples: Vec<i16> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
      let expected: Vec<i16> = vec![607, 618, 618, 617, 647, 716, 769, 792];
      assert_eq!(sliding_window(&samples), expected)
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

  print!("A: {}\n", count_increases(&samples));
  print!("B: {}\n", count_increases(&sliding_window(&samples)));
}

