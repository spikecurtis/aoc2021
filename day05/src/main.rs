use std::collections::HashMap;
use std::io;
use std::cmp::max;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Point(i64, i64);

impl Point {
  fn from_input(s: &str) -> Point {
    let xy: Vec<i64> = s.split(",").map(|x| x.trim().parse::<i64>().unwrap()).collect();
    Point(xy[0], xy[1])
  }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Line {
  start: Point,
  end: Point
}

impl Line {
  fn from_input(s: &String) -> Line {
    let se: Vec<Point> = s.split("->").map(|p| Point::from_input(p.trim())).collect();
    Self{start: se[0], end: se[1]}
  }

  fn points(&self) -> Vec<Point> {
    let mut out: Vec<Point> = Vec::new();
    let x_diff = self.end.0 - self.start.0;
    let mut x_step = x_diff;
    if x_step != 0 {
      x_step /= x_diff.abs()
    } 
    let y_diff = self.end.1 - self.start.1;
    let mut y_step = y_diff;
    if y_step != 0 {
      y_step /= y_diff.abs();
    }
    let num_steps = max(x_diff.abs(), y_diff.abs()) + 1;
    for i in 0..num_steps {
      out.push(Point(self.start.0+i*x_step, self.start.1+i*y_step));
    }
    return out;
  }
}

struct InputReader;

impl Iterator for InputReader {
  type Item = Line;

  fn next(&mut self) -> Option<Self::Item> {
    let mut b = String::new();
    let r = io::stdin().read_line(&mut b).ok()?;
    if r == 0 {
      None
    } else {
      b.pop(); // remove newline;
      Some(Line::from_input(&b))
    }
  }
}

fn main() -> io::Result<()>{
  let mut m = HashMap::new();
  for l in InputReader {
    //println!("line {:?}", l);
    for p in l.points() {
      //println!("  point {:?}", p);
      let count = m.entry(p).or_insert(0);
      *count += 1;
    }
  }
  let mut c: usize = 0;
  for (_, n) in &m {
    if *n > 1 {
      c += 1;
    }
  }
  println!("{}", c);
  Ok(())
 }
