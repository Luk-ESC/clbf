unsafe extern "C" {
	safe fn putchar(ch: i32) -> i32;
	safe fn getchar() -> i32;
}

fn main() {
	let mut grid = [0u8; 30000];
	let mut ptr = 15000;

	grid[ptr] += 4;
	grid[ptr + 1] += 5 * grid[ptr];
	grid[ptr] = 0;
	grid[ptr] += 5 * grid[ptr + 1];
	grid[ptr + 1] = 0;
	grid[ptr + 1] = 1;
	grid[ptr] += 1;

	while grid[ptr] != 0 {
		grid[ptr + 2] += grid[ptr + 1];
		grid[ptr + 3] += grid[ptr + 1];
		grid[ptr + 1] = 0;
		grid[ptr + 1] = 2;
		grid[ptr + 1] += grid[ptr + 3];
		grid[ptr + 3] = 0;
		grid[ptr + 6] = 0;
		grid[ptr + 6] = 2;
		grid[ptr + 7] = 0;
		grid[ptr + 7] = 1;
		grid[ptr + 10] += 1;
		ptr += 10;

		while grid[ptr] != 0 {
			grid[ptr] = 0;
			grid[ptr] = 6;
			ptr += 3;
		}

		ptr -= 3;

		while grid[ptr] != 0 {
			grid[ptr - 1] += 8 * grid[ptr];
			grid[ptr - 2] += 2 * grid[ptr];
			grid[ptr] = 0;
			grid[ptr] = 1;
			putchar(grid[ptr - 1] as i32);
			grid[ptr - 1] -= 4 * grid[ptr - 2];
			grid[ptr - 2] = 0;
			ptr -= 3;
		}

		ptr -= 2;

		while grid[ptr] != 0 {
			ptr += 5;

			while grid[ptr] != 0 {
				grid[ptr + 3] = 0;
				grid[ptr + 3] = 9;
				grid[ptr + 3] -= grid[ptr + 2];
				grid[ptr + 2] = 0;
				grid[ptr + 2] = 9;
				ptr += 3;

				while grid[ptr] != 0 {
					grid[ptr] -= 1;
					grid[ptr - 1] -= grid[ptr];
					grid[ptr] = 0;
					grid[ptr] = 1;

					while grid[ptr] != 0 {
						ptr -= 3;
					}

				}

				grid[ptr] += grid[ptr - 1];
				grid[ptr - 1] = 0;
			}

			ptr -= 2;
			grid[ptr] -= 1;
		}

		ptr -= 2;
		grid[ptr] -= 1;
	}


	while grid[ptr] != 0 {
		putchar(grid[ptr] as i32);
		putchar(grid[ptr] as i32);
		putchar(grid[ptr] as i32);
	}

}
