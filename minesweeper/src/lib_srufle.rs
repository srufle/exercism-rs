pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let height = minefield.len();
    let width = if height > 0 { minefield[0].len() } else { 0 };
    if height == 0 {
        return vec![];
    } else if width == 0 {
        return vec![String::new()];
    }

    let mut ans = minefield
        .iter()
        .map(|row| row.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let mut row = 0;
    let mut col = 0;
    while row < ans.len() {
        while col < ans[row].len() {
            let curr_cell = ans[row][col];
            if curr_cell == '*' {
                col += 1;
                continue;
            }
            let mut score = 0;
            let directions = [
                Direction::North,
                Direction::NorthEast,
                Direction::East,
                Direction::SouthEast,
                Direction::South,
                Direction::SouthWest,
                Direction::West,
                Direction::NorthWest,
            ];
            for direction in directions {
                if let Some(c) = get_cell(direction, &ans, row, col, width, height) {
                    if c == '*' {
                        score += 1;
                    }
                }
            }

            if score > 0 {
                ans[row][col] = char::from_digit(score, 10).unwrap();
            }

            col += 1;
        }
        row += 1;
        col = 0;
    }

    let final_ans = ans
        .iter()
        .map(|row| row.iter().collect())
        .collect::<Vec<String>>();
    final_ans
}

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
    NorthEast,
    SouthEast,
    NorthWest,
    SouthWest,
}
fn get_cell(
    direction: Direction,
    list: &Vec<Vec<char>>,
    row: usize,
    col: usize,
    width: usize,
    height: usize,
) -> Option<char> {
    let row = row as isize;
    let col = col as isize;
    let width = width as isize;
    let height = height as isize;
    let (row, col) = match direction {
        Direction::North => {
            let row = row - 1;

            (row, col)
        }
        Direction::NorthEast => {
            let row = row - 1;
            let col = col + 1;
            (row, col)
        }
        Direction::East => {
            let col = col + 1;
            (row, col)
        }
        Direction::SouthEast => {
            let row = row + 1;
            let col = col + 1;
            (row, col)
        }
        Direction::South => {
            let row = row + 1;
            (row, col)
        }
        Direction::SouthWest => {
            let row = row + 1;
            let col = col - 1;
            (row, col)
        }
        Direction::West => {
            let col = col - 1;
            (row, col)
        }
        Direction::NorthWest => {
            let row = row - 1;
            let col = col - 1;
            (row, col)
        }
    };
    if row >= 0 && row < height && col >= 0 && col < width {
        return Some(list[row as usize][col as usize]);
    } else {
        None
    }
}
