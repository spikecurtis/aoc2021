use std::io;

type Board = [[u8; 5]; 5];

struct MarkedBoard {
    board: Board,
    marked: [[bool; 5]; 5],
    done: bool
}

impl MarkedBoard {
    fn new(board: Board) -> MarkedBoard {
        MarkedBoard{
            board,
            marked: [
                [false, false, false, false, false],
                [false, false, false, false, false],
                [false, false, false, false, false],
                [false, false, false, false, false],
                [false, false, false, false, false]],
            done: false,
        }
    }

    // mark a number on the board, if it exists.  Returns Some score if this results in a bingo.
    // Returns None if not a bingo.
    fn mark(&mut self, v: u8) -> Option<u64> {
        for i in 0..5 {
            for j in 0..5 {
                if self.board[i][j] == v {
                    self.marked[i][j] = true;
                    if self.check_bingo(i, j) {
                        self.done = true;
                        return Some(self.score(v));
                    }
                }
            }
        }
        None
    }

    fn check_bingo(&self, i: usize, j: usize) -> bool {
        if self.marked[i].iter().all(|&b| b) {
            return true;
        }
        for k in 0..5 {
            if !self.marked[k][j] {
                return false;
            }
        }
        return true;
    }

    fn score(&self, v: u8) -> u64 {
        let mut s: u64 = 0;
        for i in 0..5 {
            for j in 0..5 {
                if !self.marked[i][j] {
                    s += self.board[i][j] as u64;
                }
            }
        }
        return s * v as u64;
    }
}

struct BoardReader;

impl Iterator for BoardReader {
  type Item = Board;

  fn next(&mut self) -> Option<Board> {
    let mut b = String::new();
    let r = io::stdin().read_line(&mut b).ok()?;
    if r == 0 {
      None
    } else {
      let mut brd: Board = [[0; 5]; 5];   
      for i in 0..5 {
        b.clear();
        io::stdin().read_line(&mut b).ok()?;
        b.pop();
        for j in 0..5 {
          let k = j*3;
          let nstr = b[k..k+2].trim();
          brd[i][j] = nstr.parse::<u8>().unwrap();
        }
      }
      Some(brd)
    }
  }
}

fn main() -> io::Result<()> {
  let mut b = String::new();
  io::stdin().read_line(&mut b)?;
  b.pop();
  let calls: Vec<u8> = b.split(",").map(|s| s.parse::<u8>().unwrap()).collect();
  let mut boards: Vec<MarkedBoard> = BoardReader.map(|b| MarkedBoard::new(b)).collect();
  for call in calls {
    for board in &mut boards {
      if board.done {
        continue;
      }
      let score = board.mark(call);
      match score {
        Some(x) => {
          println!("Winning board score: {}", x);
        }
        None => {}
      }
    }
  }
  Ok(())
}
