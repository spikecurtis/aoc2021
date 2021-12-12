use std::io;

type State = [u64; 9];

fn next(old: State) -> State {
  let mut new: State = [0; 9];
  new[8] = old[0];
  new[7] = old[8];
  new[6] = old[7] + old[0];
  new[5] = old[6];
  new[4] = old[5];
  new[3] = old[4];
  new[2] = old[3];
  new[1] = old[2];
  new[0] = old[1];
  return new;
}

fn main() -> io::Result<()> {
  let mut b = String::new();
  io::stdin().read_line(&mut b)?;
  b.pop();
  let mut s: State = [0; 9];
  for t in b.split(",").map(|s| s.parse::<u8>().unwrap()) {
    s[t as usize] += 1; 
  }
  for _ in 0..256 {
    s = next(s);
  }
  println!("Total fish {}", s.iter().sum::<u64>());
  Ok(())
}
