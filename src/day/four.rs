#[derive(Debug, PartialEq, Clone, Copy)]
enum Marking {
    Marked(u8),
    Unmarked(u8)
}

#[derive(Debug)]
struct Bingo {
    boards: Vec<Board>
}

impl Bingo {
    fn new() -> Self {
        Self {
            boards: Vec::new()
        }
    }
}

#[derive(Debug, Clone)]
struct Board {
    win: bool,
    rows: Vec<Vec<Marking>>
}

impl Board {
    fn new() -> Self {
        Self {
            win: false,
            rows: Vec::new(),
        }
    }

    fn add_row(&mut self, row: Vec<Marking>) {
        self.rows.push(row);
    }

    fn mark(&mut self, number: u8) {
        for i in 0..self.rows.len() {
            for j in 0..self.rows[i].len() {
                if let Marking::Unmarked(value) = self.rows[i][j] {
                    if value == number {
                        self.rows[i][j] = Marking::Marked(number);
                    }
                }
            }
        }
    }

    fn validate_rows(&self) -> bool {
        for r in 0..5 {
            let marks = self.rows[r].iter().filter(|e| matches!(e, Marking::Marked(_))).count();
            if marks == self.rows[r].len() {
                return true
            } 
        }
        return false
    }

    fn validate_cols(&self) -> bool {
        for c in 0..5 {
            let mut cols: Vec<Marking> = Vec::new();
            for r in 0..5 {
                cols.push(self.rows[r][c]);
            }

            let marks = cols.iter().filter(|e| matches!(e, Marking::Marked(_))).count();
            if marks == 5 {
                return true
            }
        }

        return false
    }

    fn validate(&self) -> bool {
        if self.validate_rows() || self.validate_cols() {
            return true
        } else {
            return false;
        }
    }

    fn score(&self, number: i32) -> i32 {
        let mut nums: Vec<i32> = Vec::new();

        for r in 0..5 {
            for c in 0..5 {
                if let Marking::Unmarked(value) = self.rows[r][c] {
                    nums.push(value as i32);
                }
            }
        }

        let sum: i32 = nums.iter().sum();

        return sum * number
    }
}

pub fn part_one(input: Vec<String>) -> i32 {    
    let numbers: Vec<u8> = input[0].split(",").map(|x| x.parse::<u8>().unwrap()).collect();
    let chunks = numbers.chunks(5);

    let mut bingo: Bingo = Bingo::new();
    bingo.boards.push(Board::new());
    let mut current_board_index: usize = 0;
    let mut current_board: &mut Board = &mut bingo.boards[current_board_index];

    for i in 2..input.len() {
        let row: Vec<Marking> = input[i].split(" ").filter(|x| !x.is_empty()).map(|x| Marking::Unmarked(x.parse::<u8>().unwrap())).collect();
        current_board.add_row(row);

         if input[i].is_empty() {
            current_board_index = current_board_index + 1;
            bingo.boards.push(Board::new());
            current_board = &mut bingo.boards[current_board_index as usize];
            continue;
        };
    }

    for c in chunks {
        for ci in 0..c.len() {
            for bi in 0..bingo.boards.len() {
                bingo.boards[bi].mark(c[ci]);
                if bingo.boards[bi].validate() {
                    return bingo.boards[bi].score(c[ci] as i32);
                }
            }
        }
    }

    0
}

#[test]
fn verify_part_one() {
    assert_eq!(part_one(
        vec![
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1".to_owned(), 
            "".to_owned(), 
            "22 13 17 11  0".to_owned(),
            " 8  2 23  4 24".to_owned(),
            "21  9 14 16  7".to_owned(),
            " 6 10  3 18  5".to_owned(),
            " 1 12 20 15 19".to_owned(),
            "".to_owned(),
            " 3 15  0  2 22".to_owned(),
            " 9 18 13 17  5".to_owned(),
            "19  8  7 25 23".to_owned(),
            "20 11 10 24  4".to_owned(),
            "14 21 16 12  6".to_owned(),
            "".to_owned(),
            "14 21 17 24  4".to_owned(),
            "10 16 15  9 19".to_owned(),
            "18  8 23 26 20".to_owned(),
            "22 11 13  6  5".to_owned(),
            " 2  0 12  3  7".to_owned(),
        ]
    ), 4512);
}

pub fn part_two(input: Vec<String>) -> i32 {
    let numbers: Vec<u8> = input[0].split(",").map(|x| x.parse::<u8>().unwrap()).collect();
    let chunks = numbers.chunks(5);

    let mut bingo: Bingo = Bingo::new();
    bingo.boards.push(Board::new());
    let mut current_board_index: usize = 0;
    let mut current_board: &mut Board = &mut bingo.boards[current_board_index];

    for i in 2..input.len() {
        let row: Vec<Marking> = input[i].split(" ").filter(|x| !x.is_empty()).map(|x| Marking::Unmarked(x.parse::<u8>().unwrap())).collect();
        current_board.add_row(row);

         if input[i].is_empty() {
            current_board_index = current_board_index + 1;
            bingo.boards.push(Board::new());
            current_board = &mut bingo.boards[current_board_index as usize];
            continue;
        };
    }

    for c in chunks {
        for ci in 0..c.len() {
            for bi in 0..bingo.boards.len() {
                bingo.boards[bi].mark(c[ci]);
                if bingo.boards[bi].validate() && !bingo.boards[bi].win {
                    if bingo.boards.iter().filter(|b| b.win).count() == bingo.boards.iter().count() - 1 {
                        let last: Vec<&Board> = bingo.boards.iter().filter(|&b| !b.win).collect();
                        return last[0].score(c[ci] as i32);
                    }
                    bingo.boards[bi].win = true;
                }
            }
        }
    }

    0
}


#[test]
fn verify_part_two() {
    assert_eq!(part_two(
        vec![
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1".to_owned(), 
            "".to_owned(), 
            "22 13 17 11  0".to_owned(),
            " 8  2 23  4 24".to_owned(),
            "21  9 14 16  7".to_owned(),
            " 6 10  3 18  5".to_owned(),
            " 1 12 20 15 19".to_owned(),
            "".to_owned(),
            " 3 15  0  2 22".to_owned(),
            " 9 18 13 17  5".to_owned(),
            "19  8  7 25 23".to_owned(),
            "20 11 10 24  4".to_owned(),
            "14 21 16 12  6".to_owned(),
            "".to_owned(),
            "14 21 17 24  4".to_owned(),
            "10 16 15  9 19".to_owned(),
            "18  8 23 26 20".to_owned(),
            "22 11 13  6  5".to_owned(),
            " 2  0 12  3  7".to_owned(),
        ]
    ), 1924);
}