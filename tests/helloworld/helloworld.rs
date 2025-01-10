unsafe extern "C" {
	safe fn putchar(ch: i32) -> i32;
	safe fn getchar() -> i32;
}

fn main() {
	let mut grid = [0u8; 30000];
	let mut ptr = 15000;

	ptr += 1;
	grid[ptr] += 8;
	grid[ptr - 1] += 9 * grid[ptr];
	grid[ptr] = 0;
	ptr -= 1;
	putchar(grid[ptr] as i32);
	ptr += 1;
	grid[ptr] += 4;
	grid[ptr - 1] += 7 * grid[ptr];
	grid[ptr] = 0;
	ptr -= 1;
	grid[ptr] += 1;
	putchar(grid[ptr] as i32);
	grid[ptr] += 7;
	putchar(grid[ptr] as i32);
	putchar(grid[ptr] as i32);
	grid[ptr] += 3;
	putchar(grid[ptr] as i32);
	ptr += 2;
	grid[ptr] += 6;
	grid[ptr - 1] += 7 * grid[ptr];
	grid[ptr] = 0;
	ptr -= 1;
	grid[ptr] += 2;
	putchar(grid[ptr] as i32);
	grid[ptr] -= 12;
	putchar(grid[ptr] as i32);
	ptr += 1;
	grid[ptr] += 6;
	grid[ptr - 1] += 9 * grid[ptr];
	grid[ptr] = 0;
	ptr -= 1;
	grid[ptr] += 1;
	putchar(grid[ptr] as i32);
	ptr -= 1;
	putchar(grid[ptr] as i32);
	grid[ptr] += 3;
	putchar(grid[ptr] as i32);
	grid[ptr] -= 6;
	putchar(grid[ptr] as i32);
	grid[ptr] -= 8;
	putchar(grid[ptr] as i32);
	ptr += 3;
	grid[ptr] += 4;
	grid[ptr - 1] += 8 * grid[ptr];
	grid[ptr] = 0;
	ptr -= 1;
	grid[ptr] += 1;
	putchar(grid[ptr] as i32);
}
