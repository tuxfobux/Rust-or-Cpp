#include<iostream>
#include<algorithm>
using namespace std;

int seed = 7; // 1-9
int rand_arr[9] = {9,5,6,1,7,4,8,3,2};
int grid[9][9];

void print_grid(){
	for (int row = 0; row < 9; ++row){
		for (int col = 0; col < 9; ++col)
			cout << grid[row][col] << " ";
		cout << '\n';
	}
	cout << '\n';
}

int random_number(){
	seed = rand_arr[seed-1];
	return seed;
}

void remove_elements(int n){
	while(--n){
		grid[random_number()-1][random_number()-1] = 0;
		grid[(random_number()+3)%9][(random_number()-2)%9] = 0; // due to custom random numbers

	}
}

bool is_safe(int row, int col, int num){
	// rows
	for (int x = 0; x < 9; ++x)
		if (grid[row][x] == num)
			return false;

	// columns
	for (int x = 0; x < 9; ++x)
		if (grid[x][col] == num)
			return false;

	// 3*3 matrix
	int start_row = row - row % 3;
	int start_col = col - col % 3;
	for (int r = 0; r < 3; ++r)
		for (int c = 0; c < 3; ++c)
			if (grid[r + start_row][c + start_col] == num)
				return false;

	return true;
}

void fill_diagonals(){
	for(int r = 0; r<9; ++r){
		for(int c = 0; c<3; ++c){
			grid[r][r/3*3+c] = random_number();
		}
	}
}

int next_square(int i, int j){
	int next = 0;
	while(1){
		j++; next++;
		if(j==9){ j = 0; i++; };
		if(i==9 || i/3 != j/3) break;
	}
	return next;
}

bool generate_sudoku(int i, int j){
	if(j>8){ j = j-9; i++; };
	if(i==9) return true;

	for (int digit = 1; digit < 10; ++digit){
		if (is_safe(i, j, digit)){
			grid[i][j] = digit;
			if(generate_sudoku(i, j + next_square(i, j)))
				return true;
		}
	}
	grid[i][j] = 0;
	return false;
}


int main(){
	for(int repeat = 0; repeat<100000; ++repeat){
		for(int init_seed = 1; init_seed<10; ++init_seed){
			seed = init_seed;
			fill(*grid, *grid + 81, 0);
			fill_diagonals();
			generate_sudoku(0,3);
			remove_elements(25);
			//print_grid();
		}
	}
	return 0;
}
