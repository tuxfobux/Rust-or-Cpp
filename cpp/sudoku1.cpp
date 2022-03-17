#include<iostream>
#include<set>
#include<algorithm>
using namespace std;

int seed = 7; // 1-9
int rand_arr[9] = {9,5,6,1,7,4,8,3,2};
int grid[9][9];
int permutations[362880][9];

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

void generate_rows(){
	int nums[9] = {1, 2, 3, 4, 5, 6, 7, 8, 9};
	int n = 0;
	do {
		copy(nums, nums + 9, permutations[n]);
		++n;
	} while (next_permutation(nums, nums + 9));
}

bool is_safe(int curr_row){
	// columns
	for(int col = 0; col < 9; ++col){
		set<int> uniq;
		int count = 0;
		for (int x = 0; x < 9; ++x)
			if (grid[x][col] != 0){
				uniq.insert(grid[x][col]);
				++count;
			}
		if(uniq.size() != count) return false;
	}

	// 3*3 matrixes
	int matrix_rows = curr_row / 3;
	int box_num = 0;
	while(box_num < 3){
		set<int> uniq;
		int count = 0;
		for (int row = 0; row < 3; ++row){
			for (int col = 0; col < 3; ++col){
				if (grid[row + 3 * matrix_rows][col + 3 * box_num] != 0){
					uniq.insert(grid[row + 3 * matrix_rows][col + 3 * box_num]);
					count++;
				}
			}
		}
		if(uniq.size() != count) return false;
		++box_num;
	}
	return true;
}

void generate_sudoku(){
	int seed[9] = {201421, 184861, 215341, 147357, 399111, 125680, 45181, 71359, 215561}; // adjustable 
	for(int curr_row = 0; curr_row < 9; ++curr_row){
		int n = 0;
		do {
			int perm = (seed[curr_row] + n) % 362880;
			copy(permutations[perm], permutations[perm] + 9, grid[curr_row]);
			n = (n + 1) % 362880;
		} while (!is_safe(curr_row));
	}
}

int main(){
	for(int repeat = 0; repeat<100; ++repeat){
		generate_rows();
		generate_sudoku();
		remove_elements(25);
		//print_grid();
	}
	return 0;
}
