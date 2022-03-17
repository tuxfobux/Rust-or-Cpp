static mut SEED: i32 = 7;
static mut GRID: [[i32; 9]; 9] = [[0i32; 9]; 9];
static RAND_ARR: [i32; 9] = [9, 5, 6, 1, 7, 4, 8, 3, 2];

fn print_grid() {
    for row in 0..9 {
        for col in 0..9 {
            unsafe {
                print!("{} ", GRID[row][col]);
            }
        }
        println!();
    }
    println!();
}

fn random_number() -> i32 {
    unsafe {
        SEED = RAND_ARR[SEED as usize - 1];
        SEED
    }
}

fn remove_elements(mut n: i32) {
    while n != 0 {
        unsafe {
            GRID[random_number() as usize - 1][random_number() as usize - 1] = 0;
            GRID[(random_number() + 3) as usize % 9][(random_number() - 2) as usize % 9] = 0;
            // due to custom random numbers
        }
        n = n - 1;
    }
}

fn is_safe(row: usize, col: usize, num: i32) -> bool {
    // rows
    for x in 0..9 {
        unsafe {
            if GRID[row][x] == num {
                return false;
            }
        }
    }

    // columns
    for x in 0..9 {
        unsafe {
            if GRID[x][col] == num {
                return false;
            }
        }
    }

    // 3*3 matrix
    let start_row = row - row % 3;
    let start_col = col - col % 3;
    for r in 0..3 {
        for c in 0..3 {
            unsafe {
                if GRID[r + start_row][c + start_col] == num {
                    return false;
                }
            }
        }
    }
    true
}

fn fill_diagonals() {
    for r in 0..9 {
        for c in 0..3 {
            unsafe {
                GRID[r][r / 3 * 3 + c] = random_number();
            }
        }
    }
}

fn next_square(mut i: i32, mut j: i32) -> i32 {
    let mut next = 0;
    loop {
        j = j + 1;
        next = next + 1;
        if j == 9 {
            j = 0;
            i = i + 1;
        }
        if i == 9 || i / 3 != j / 3 {
            break;
        }
    }
    next
}

fn generate_sudoku(mut i: i32, mut j: i32) -> bool {
    if j > 8 {
        j = j - 9;
        i = i + 1;
    }
    if i == 9 {
        return true;
    }

    for digit in 1..10 {
        if is_safe(i as usize, j as usize, digit) {
            unsafe {
                GRID[i as usize][j as usize] = digit;
            }
            if generate_sudoku(i, j + next_square(i, j)) {
                return true;
            }
        }
    }
    unsafe {
        GRID[i as usize][j as usize] = 0;
    }
    false
}

fn main() {
    for _repeat in 0..100000 {
        for init_seed in 1..10 {
            unsafe {
                SEED = init_seed;
                GRID = [[0i32; 9]; 9];
            }
            fill_diagonals();
            generate_sudoku(0, 3);
            remove_elements(25);
            //print_grid();
        }
    }
}
