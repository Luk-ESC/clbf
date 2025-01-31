unsafe extern "C" {
	safe fn putchar(ch: i32) -> i32;
	safe fn getchar() -> i32;
}

fn main() {
	let mut grid = [0u8; 30000];
	let mut ptr = 15000;


	while grid[ptr] != 0 {
		putchar(grid[ptr] as i32);
		grid[ptr] -= 2;
		putchar(grid[ptr] as i32);
		putchar(grid[ptr] as i32);
	}

	grid[ptr] = 8;
	grid[ptr + 1] += grid[ptr];
	grid[ptr + 2] += 4 * grid[ptr];
	grid[ptr] = 0;
	grid[ptr + 1] += 2;
	grid[ptr + 3] += 1;
	ptr += 2;

	while grid[ptr] != 0 {
		grid[ptr] -= 1;
		grid[ptr + 2] += grid[ptr];
		grid[ptr] = 0;
		grid[ptr] = 1;
		ptr += 2;
	}

	ptr += 1;
	grid[ptr] += 1;

	while grid[ptr] != 0 {
		grid[ptr] -= 1;
		ptr -= 3;

		while grid[ptr] != 0 {
			grid[ptr] -= 1;
			ptr += 1;

			while grid[ptr] != 0 {
				grid[ptr] = 0;
				grid[ptr] = 1;
				grid[ptr + 1] += 2;
				grid[ptr + 4] -= 1;
				ptr += 2;
			}

			ptr -= 1;

			while grid[ptr] != 0 {
				ptr -= 1;
			}

			grid[ptr + 2] += 6;
			grid[ptr] += 5 * grid[ptr + 2];
			grid[ptr + 2] = 0;
			grid[ptr + 2] = 1;
			grid[ptr] += 2;
			putchar(grid[ptr] as i32);
			grid[ptr] = 0;
			ptr -= 2;
		}

		putchar(grid[ptr + 1] as i32);
		grid[ptr + 2] += 1;
		ptr += 2;

		while grid[ptr] != 0 {
			ptr += 2;
		}

		ptr += 1;
		grid[ptr] += 1;
	}


	while grid[ptr] != 0 {
		putchar(grid[ptr] as i32);
	}

}
