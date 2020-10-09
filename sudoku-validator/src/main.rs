struct Sudoku {
    data: Vec<Vec<u32>>,
}

fn sqrt(n: usize) -> usize {
    (n as f64).sqrt() as usize
}

use std::collections::HashSet;

impl Sudoku {
    fn is_valid(&self) -> bool {
        println!("{:?}", self.data);
        let size = self.data.len();

        let n = sqrt(size);

        let mut rows: Vec<HashSet<u32>> = vec![HashSet::new(); size];
        let mut columns: Vec<HashSet<u32>> = vec![HashSet::new(); size];
        let mut sub: Vec<Vec<HashSet<u32>>> = vec![vec![HashSet::new(); n]; n];

        for (row_num, row) in self.data.iter().enumerate() {
            if row.len() != size {
                return false;
            }
            for (col_num, number) in row.iter().enumerate() {
                let max = size as u32;
                if *number > max || *number < 1 {
                    return false;
                }
                rows[row_num].insert(*number);
                columns[col_num].insert(*number);
                sub[row_num / n][col_num / n].insert(*number);
            }
        }

        for hash in rows {
            if hash.len() != size {
                return false;
            }
        }

        for hash in columns {
            if hash.len() != size {
                return false;
            }
        }

        for entry in sub {
            for hash in entry {
                if hash.len() != size {
                    return false;
                }
            }
        }

        return true;
    }
}

fn main() {
    let good_sudoku_1 = Sudoku {
        data: vec![
            vec![7, 8, 4, 1, 5, 9, 3, 2, 6],
            vec![5, 3, 9, 6, 7, 2, 8, 4, 1],
            vec![6, 1, 2, 4, 3, 8, 7, 5, 9],
            vec![9, 2, 8, 7, 1, 5, 4, 6, 3],
            vec![3, 5, 7, 8, 4, 6, 1, 9, 2],
            vec![4, 6, 1, 9, 2, 3, 5, 8, 7],
            vec![8, 7, 6, 3, 9, 4, 2, 1, 5],
            vec![2, 4, 3, 5, 6, 1, 9, 7, 8],
            vec![1, 9, 5, 2, 8, 7, 6, 3, 4],
        ],
    };

    println!("is valid: {}", good_sudoku_1.is_valid());
}

#[test]
fn good_sudoku() {
    let good_sudoku_1 = Sudoku {
        data: vec![
            vec![7, 8, 4, 1, 5, 9, 3, 2, 6],
            vec![5, 3, 9, 6, 7, 2, 8, 4, 1],
            vec![6, 1, 2, 4, 3, 8, 7, 5, 9],
            vec![9, 2, 8, 7, 1, 5, 4, 6, 3],
            vec![3, 5, 7, 8, 4, 6, 1, 9, 2],
            vec![4, 6, 1, 9, 2, 3, 5, 8, 7],
            vec![8, 7, 6, 3, 9, 4, 2, 1, 5],
            vec![2, 4, 3, 5, 6, 1, 9, 7, 8],
            vec![1, 9, 5, 2, 8, 7, 6, 3, 4],
        ],
    };

    let good_sudoku_2 = Sudoku {
        data: vec![
            vec![1, 4, 2, 3],
            vec![3, 2, 4, 1],
            vec![4, 1, 3, 2],
            vec![2, 3, 1, 4],
        ],
    };
    assert!(good_sudoku_1.is_valid());
    assert!(good_sudoku_2.is_valid());
}

#[test]
fn bad_sudoku() {
    let bad_sudoku_1 = Sudoku {
        data: vec![
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        ],
    };

    let bad_sudoku_2 = Sudoku {
        data: vec![
            vec![1, 2, 3, 4, 5],
            vec![1, 2, 3, 4],
            vec![1, 2, 3, 4],
            vec![1],
        ],
    };

    assert!(!bad_sudoku_1.is_valid());
    assert!(!bad_sudoku_2.is_valid());
}
