unsafe extern "C" {
	safe fn putchar(ch: i32) -> i32;
	safe fn getchar() -> i32;
}

fn main() {
	let mut grid = [0u8; 30000];
	let mut ptr = 15000;

	grid[ptr] += 8;
	grid[ptr + 1] += 8 * grid[ptr];
	grid[ptr] = 0;
	grid[ptr] += 4 * grid[ptr + 1];
	grid[ptr + 1] = 0;
	grid[ptr + 1] = 1;

	while grid[ptr] != 0 {
		grid[ptr + 1] -= 1;
		grid[ptr + 1] += 4 * grid[ptr];
		grid[ptr] = 0;
		grid[ptr] += 8 * grid[ptr + 1];
		grid[ptr + 1] = 0;
		grid[ptr + 1] += 8 * grid[ptr];
		grid[ptr] = 0;
		grid[ptr] = 1;
		ptr += 1;

		while grid[ptr] != 0 {
			grid[ptr + 1] += 10;
			grid[ptr + 2] += 5 * grid[ptr + 1];
			grid[ptr + 1] = 0;
			grid[ptr + 2] += 1;
			putchar(grid[ptr + 2] as i32);
			grid[ptr + 2] -= 1;
			putchar(grid[ptr + 2] as i32);
			grid[ptr + 2] = 0;
			grid[ptr] = 0;
			grid[ptr - 1] -= 1;
		}

		ptr -= 1;

		while grid[ptr] != 0 {
			grid[ptr + 2] += 7;
			grid[ptr + 3] += 7 * grid[ptr + 2];
			grid[ptr + 2] = 0;
			putchar(grid[ptr + 3] as i32);
			grid[ptr + 3] += 5;
			putchar(grid[ptr + 3] as i32);
			grid[ptr + 3] = 0;
			grid[ptr] -= 1;
		}

	}

	ptr += 1;

	while grid[ptr] != 0 {
		grid[ptr + 1] += 8;
		grid[ptr + 2] += 7 * grid[ptr + 1];
		grid[ptr + 1] = 0;
		putchar(grid[ptr + 2] as i32);
		grid[ptr + 2] = 0;
		grid[ptr] -= 1;
	}

	grid[ptr - 1] += 11;
	grid[ptr] += 3 * grid[ptr - 1];
	grid[ptr + 1] += 9 * grid[ptr - 1];
	grid[ptr + 2] += 9 * grid[ptr - 1];
	grid[ptr + 3] += grid[ptr - 1];
	grid[ptr - 1] = 0;
	grid[ptr] -= 1;
	putchar(grid[ptr] as i32);
	grid[ptr + 1] -= 1;
	putchar(grid[ptr + 1] as i32);
	grid[ptr + 1] += 7;
	putchar(grid[ptr + 1] as i32);
	grid[ptr + 1] += 11;
	putchar(grid[ptr + 1] as i32);
	putchar(grid[ptr] as i32);
	putchar(grid[ptr + 2] as i32);
	grid[ptr + 2] += 2;
	putchar(grid[ptr + 2] as i32);
	grid[ptr + 2] += 7;
	putchar(grid[ptr + 2] as i32);
	putchar(grid[ptr + 2] as i32);
	grid[ptr + 1] -= 1;
	putchar(grid[ptr + 1] as i32);
	grid[ptr + 3] -= 1;
	putchar(grid[ptr + 3] as i32);
	ptr += 3;

	while grid[ptr] != 0 {
		grid[ptr] = 0;
		ptr -= 1;
	}

}
