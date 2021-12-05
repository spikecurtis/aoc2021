type Board = [[u8; 5]; 5];

struct MarkedBoard {
    board: Board,
    marked: [[bool; 5]; 5]
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
                [false, false, false, false, false]]
        }
    }

    // mark a number on the board, if it exists.  Returns Some score if this results in a bingo.
    // Returns None if not a bingo.
    fn mark(&mut self, v: u8) -> Option<u64> {
        for i in 0..4 {
            for j in 0..4 {
                if self.board[i][j] == v {
                    self.marked[i][j] = true;
                    if self.check_bingo(i, j) {
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
        for k in 0..4 {
            if !self.marked[k][j] {
                return false;
            }
        }
        return true;
    }

    fn score(&self, v: u8) -> u64 {
        let mut s: u64 = 0;
        for i in 0..4 {
            for j in 0..4 {
                if self.marked[i][j] {
                    s += self.board[i][j] as u64;
                }
            }
        }
        return s * v as u64;
    }
}

fn main() {
    println!("Hello, world!");
}
