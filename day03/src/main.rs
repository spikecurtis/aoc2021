use std::io;

struct InputReader;

impl Iterator for InputReader {
  type Item = Vec<u8>;

  fn next(&mut self) -> Option<Self::Item> {
    let mut b = String::new();
    let r = io::stdin().read_line(&mut b).ok()?;
    if r == 0 {
      None
    } else {
      b.pop(); // remove newline;
      let mut v = Self::Item::new();
      for c in b.chars() {
        v.push(match c {
          '0' => 0,
          '1' => 1,
          _ => panic!("unknown input")
        })
      }
      Some(v)
    }
  }
}

fn filter_bit_criteria(v: &Vec<Vec<u8>>, pos: usize, most: bool) -> Vec<Vec<u8>> {
  let mut ones: Vec<Vec<u8>> = Vec::new();
  let mut zeros: Vec<Vec<u8>> = Vec::new();
  for bitstring in v {
    if bitstring[pos] == 1 {
      ones.push((*bitstring).clone());
    } else {
      zeros.push((*bitstring).clone());
    }
  }
  if most {
    if ones.len() >= zeros.len() {
      ones
    } else {
      zeros
    }
  } else {
    if zeros.len() <= ones.len() {
      zeros
    } else {
      ones
    }
  }
}

fn bitstring_to_int(bitstring: &[u8]) -> u64 {
  let mut out: u64 = 0;
  for bit in bitstring {
    out = out << 1;
    out += *bit as u64;
  }
  out
}

fn main() -> io::Result<()> {
    let r: Vec<Vec<u8>> = InputReader.collect();
    let mut counts: Vec<usize> = Vec::new();
    let inputs = r.len();
    for v in &r[..] {
      let l = v.len();
      while counts.len() < l {
        counts.push(0);
      }
      for i in 0..l {
        counts[i] += v[i] as usize;
      }
    }
    // find most common
    let mut gamma = 0;
    let mut epsilon = 0;
    for c in counts {
      gamma = gamma << 1;
      epsilon = epsilon << 1;
      if c > (inputs - c) {
        gamma += 1;
      } else {
        epsilon += 1; 
      }
    }
    println!{"gamma: {}, epsilon {}, product {}", gamma, epsilon, gamma*epsilon};

    // oxygen generator
    let mut oxy_strings = r.clone();
    let mut pos: usize = 0;
    while oxy_strings.len() > 1 {
      oxy_strings = filter_bit_criteria(&oxy_strings, pos, true);
      pos += 1;
    }
    let oxy_generator = bitstring_to_int(oxy_strings[0].as_slice());

    // co2 scrubber
    let mut co2_strings = r.clone();
    pos = 0;
    while co2_strings.len() > 1 {
      co2_strings = filter_bit_criteria(&co2_strings, pos, false);
      pos += 1;
    }
    let co2_scrubber = bitstring_to_int(co2_strings[0].as_slice());
    println!("oxygen generator: {}, CO2 scrubber: {}, product: {}",
             oxy_generator, co2_scrubber, oxy_generator*co2_scrubber);
    Ok(())
}
