use permutohedron::LexicalPermutation;
use std::collections::BTreeSet;

static mut SEED: i32 = 7;
static mut GRID: [[i32; 9]; 9] = [[0i32; 9]; 9];
static RAND_ARR: [i32; 9] = [9, 5, 6, 1, 7, 4, 8, 3, 2];
static mut PERMUTATIONS: [[i32; 9]; 362880] = [[0i32; 9]; 362880];

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

fn generate_rows() {
    let mut nums: [i32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut n = 0;
    loop {
        unsafe {
            PERMUTATIONS[n] = nums;
        }
        n = n + 1;
        if !nums.next_permutation() {
            break;
        }
    }
}

fn is_safe(curr_row: i32) -> bool {
    // columns
    for col in 0..9 {
        let mut uniq = BTreeSet::new();
        let mut count: usize = 0;
        for x in 0..9 {
            unsafe {
                if GRID[x][col] != 0 {
                    uniq.insert(GRID[x][col]);
                    count = count + 1;
                }
            }
            if uniq.len() != count {
                return false;
            }
        }
    }

    // 3*3 matrix
    let matrix_rows: usize = (curr_row / 3) as usize;
    let mut box_num: usize = 0;
    while box_num < 3 {
        let mut uniq: BTreeSet<i32> = BTreeSet::new();
        let mut count: usize = 0;
        for row in 0..3 {
            for col in 0..3 {
                unsafe {
                    if GRID[row + 3 * matrix_rows][col + 3 * box_num] != 0 {
                        uniq.insert(GRID[row + 3 * matrix_rows][col + 3 * box_num]);
                        count = count + 1;
                    }
                }
            }
        }
        if uniq.len() != count {
            return false;
        }
        box_num = box_num + 1;
    }
    true
}

fn generate_sudoku() {
    let seed: [i32; 9] = [
        201421, 184861, 215341, 147357, 399111, 125680, 45181, 71359, 215561,
    ]; // adjustable
    for curr_row in 0..9 {
        let mut n: i32 = 0;
        loop {
            let perm = (seed[curr_row] + n) % 362880;
            unsafe {
                GRID[curr_row] = PERMUTATIONS[perm as usize];
            }
            n = (n + 1) % 362880;
            if is_safe(curr_row as i32) {
                break;
            }
        }
    }
}

fn main() {
    for _repeat in 0..100 {
        generate_rows();
        generate_sudoku();
        remove_elements(25);
        //print_grid();
    }
}
