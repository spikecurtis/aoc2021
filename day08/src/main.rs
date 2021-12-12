use std::io;



type SegmentDisplay = [bool; 7];

trait Countable {
  fn count(&self) -> usize;
}

impl Countable for SegmentDisplay {
  fn count(&self) -> usize {
    self.iter().filter(|&b| *b).count()
  }
}

fn new_segment_display(s: &str) -> SegmentDisplay {
  let mut sd: SegmentDisplay = [false; 7];
  let a: u8 = "a".as_bytes()[0];
  for c in s.trim().as_bytes() {
    sd[(c-a) as usize] = true;
  }
  return sd;
}

#[derive(Debug, Copy, Clone)]
struct Entry {
  patterns: [SegmentDisplay; 10],
  outputs: [SegmentDisplay; 4]
}

impl Entry {
  fn from_input(s: &String) -> Entry {
    let mut parts = s.split(" | ");
    let patterns: Vec<SegmentDisplay> = parts.next().unwrap().split(" ").map(|p| new_segment_display(p)).collect();
    let outputs: Vec<SegmentDisplay> = parts.next().unwrap().split(" ").map(|p| new_segment_display(p)).collect();
    let mut entry = Entry{patterns: [[false; 7]; 10], outputs: [[false; 7]; 4]};
    for i in 0..10 {
      entry.patterns[i] = patterns[i];
    }
    for i in 0..4 {
      entry.outputs[i] = outputs[i];
    }
    return entry;
  }

  fn wire_map(&self) -> [u8; 7] {
    
  }
}

struct InputReader;

impl Iterator for InputReader {
  type Item = Entry;

  fn next(&mut self) -> Option<Self::Item> {
    let mut b = String::new();
    let r = io::stdin().read_line(&mut b).ok()?;
    if r == 0 {
      None
    } else {
      b.pop(); // remove newline;
      Some(Entry::from_input(&b))
    }
  }
}

fn main() -> io::Result<()> {
  let mut easy_digits = 0;
  for e in InputReader {
    for sd in &e.outputs {
      match sd.count() {
        2 => {easy_digits+=1;},
        3 => {easy_digits+=1;},
        4 => {easy_digits+=1;},
        7 => {easy_digits+=1;},
        _ => {},
      }
    }
  }
  println!{"Easy digits {}", easy_digits};
  Ok(())
}
