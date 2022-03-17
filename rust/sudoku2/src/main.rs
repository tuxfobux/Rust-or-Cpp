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

fn generate_sudoku(mut i: i32, mut j: i32) -> bool {
    if j == 9 {
        j = 0;
        i = i + 1;
    }
    if i == 9 {
        return true;
    }
    for _n in 0..9 {
        let rand_num = random_number();
        if is_safe(i as usize, j as usize, rand_num) {
            unsafe {
                GRID[i as usize][j as usize] = rand_num;
            }
            if generate_sudoku(i, j + 1) {
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
    for _repeat in 0..1000000 {
        for init_seed in 1..10 {
            unsafe {
                SEED = init_seed;
                GRID = [[0i32; 9]; 9];
            }
            generate_sudoku(0, 0);
            remove_elements(25);
            //print_grid();
        }
    }
}
