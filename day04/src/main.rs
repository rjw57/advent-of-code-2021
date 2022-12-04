use std::{
    fs::File,
    io::{self, BufRead},
};

#[derive(Clone, PartialEq, Debug)]
struct Board {
    rows: [[i32; 5]; 5],
    marked: [[bool; 5]; 5],
}

impl Board {
    fn new() -> Board {
        return Board {
            rows: [[0; 5]; 5],
            marked: [[false; 5]; 5],
        };
    }

    fn mark(&mut self, v: i32) {
        for r in 0..5 {
            for c in 0..5 {
                if self.rows[r][c] == v {
                    self.marked[r][c] = true;
                }
            }
        }
    }

    fn is_winning(&self) -> bool {
        for r in 0..5 {
            let mut wins = true;
            for c in 0..5 {
                wins &= self.marked[r][c];
            }
            if wins {
                return true;
            }
        }
        for c in 0..5 {
            let mut wins = true;
            for r in 0..5 {
                wins &= self.marked[r][c];
            }
            if wins {
                return true;
            }
        }
        return false;
    }

    fn score(&self, winning_number: i32) -> i32 {
        let mut score: i32 = 0;
        for r in 0..5 {
            for c in 0..5 {
                if !self.marked[r][c] {
                    score += self.rows[r][c];
                }
            }
        }
        score *= winning_number;
        return score;
    }
}

fn main() {
    let file = File::open("./input.txt").expect("could not open input");
    let mut lines = io::BufReader::new(file).lines();

    let draws_line = lines.next().expect("no draw line").unwrap();
    let draws: Vec<i32> = draws_line.split(",").map(|v| v.parse().unwrap()).collect();

    let mut boards: Vec<Board> = vec![];
    loop {
        if lines.next().is_none() {
            break;
        }
        let board_lines: Vec<_> = (0..5).map(|_| lines.next().unwrap().unwrap()).collect();
        let mut board = Board::new();
        for (r, l) in board_lines.iter().enumerate() {
            for (c, v) in l
                .split(" ")
                .filter(|v| v.len() > 0)
                .map(|v| v.parse::<i32>().unwrap())
                .enumerate()
            {
                board.rows[r][c] = v;
            }
        }
        boards.push(board);
    }

    // find first winning board
    'draws: for draw in &draws {
        for board in &mut boards {
            board.mark(*draw);
            if board.is_winning() {
                println!("first winning board score: {}", board.score(*draw));
                break 'draws;
            }
        }
    }

    // find last winning board
    let mut winner_score: Option<i32> = None;
    for draw in &draws {
        for board in &mut boards {
            if !board.is_winning() {
                board.mark(*draw);
                if board.is_winning() {
                    winner_score = Some(board.score(*draw));
                }
            }
        }
    }
    println!("last winning baord score: {}", winner_score.unwrap());
}
