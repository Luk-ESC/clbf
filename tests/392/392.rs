unsafe extern "C" {
	safe fn putchar(ch: i32) -> i32;
	safe fn getchar() -> i32;
}

fn main() {
	let mut grid = [0u8; 30000];
	let mut ptr = 15000;

	grid[ptr] -= 1;
	grid[ptr + 1] += 2;
	grid[ptr + 2] += 3;
	grid[ptr + 3] += 1;
	grid[ptr + 4] += 1;
	grid[ptr + 5] += 3;
	grid[ptr + 25] += 1;
	grid[ptr + 26] += 1;
	grid[ptr + 27] += 2;
	grid[ptr + 28] += 3;
	grid[ptr + 29] += 2;
	grid[ptr + 31] += 3;
	grid[ptr + 32] += 1;
	grid[ptr + 65] += 1;
	grid[ptr + 66] += 1;
	grid[ptr + 68] += 3;
	grid[ptr + 70] += 3;
	grid[ptr + 75] += 3;
	grid[ptr + 76] += 1;
	grid[ptr + 85] += 2;
	grid[ptr + 86] += 3;
	grid[ptr + 87] += 3;
	grid[ptr + 88] += 1;
	grid[ptr + 90] += 3;
	grid[ptr + 93] += 3;
	grid[ptr + 94] += 1;
	grid[ptr + 95] += 2;
	grid[ptr + 96] += 3;
	grid[ptr + 99] += 1;
	grid[ptr + 100] += 1;
	grid[ptr + 101] += 2;
	grid[ptr + 102] += 3;
	grid[ptr + 103] += 1;
	grid[ptr + 104] += 1;
	grid[ptr + 106] += 3;
	grid[ptr + 113] += 1;
	grid[ptr + 114] += 1;
	grid[ptr + 117] += 1;
	grid[ptr + 118] += 1;
	grid[ptr + 119] += 2;
	grid[ptr + 120] += 3;
	grid[ptr + 121] += 3;
	grid[ptr + 122] += 1;
	grid[ptr + 124] += 3;
	grid[ptr + 127] += 3;
	grid[ptr + 128] += 1;
	grid[ptr + 129] += 2;
	grid[ptr + 130] += 3;
	grid[ptr + 131] += 2;
	grid[ptr + 133] += 1;
	grid[ptr + 134] += 1;
	grid[ptr + 135] += 2;
	grid[ptr + 136] += 3;
	grid[ptr + 137] += 1;
	grid[ptr + 138] += 1;
	grid[ptr + 140] += 3;
	grid[ptr + 145] += 3;
	grid[ptr + 146] += 1;
	grid[ptr + 151] += 2;
	grid[ptr + 152] += 3;
	grid[ptr + 153] += 3;
	grid[ptr + 154] += 1;
	grid[ptr + 156] += 3;
	grid[ptr + 159] += 3;
	grid[ptr + 160] += 1;
	grid[ptr + 161] += 3;
	grid[ptr + 162] += 1;
	grid[ptr + 164] += 3;
	grid[ptr + 166] += 3;
	grid[ptr + 168] += 2;
	ptr += 168;

	while grid[ptr] != 0 {

		while grid[ptr] != 0 {
			ptr += 2;
			grid[ptr] += 1;

			while grid[ptr] != 0 {
				ptr += 1;
			}

			grid[ptr] = 2;
			ptr += 1;
			grid[ptr] += 2;

			while grid[ptr] != 0 {
				ptr -= 1;
			}

			ptr -= 1;
			grid[ptr] -= 1;
		}

		ptr += 1;
		grid[ptr] += 1;

		while grid[ptr] != 0 {
			ptr += 1;
		}

		grid[ptr - 1] += 1;
		grid[ptr - 2] += 3;
		ptr -= 2;

		while grid[ptr] != 0 {
			ptr -= 1;
		}

		ptr -= 1;
		grid[ptr] += 1;
	}

	ptr += 1;
	grid[ptr] += 1;

	while grid[ptr] != 0 {
		ptr += 1;
	}

	grid[ptr] = 4;
	ptr += 1;
	grid[ptr] += 2;

	while grid[ptr] != 0 {
		grid[ptr - 1] += 16 * grid[ptr];
		grid[ptr] = 0;
		grid[ptr - 1] += 9;
		putchar(grid[ptr - 1] as i32);
		ptr -= 2;
	}


	while grid[ptr] != 0 {
		grid[ptr] -= 1;
		putchar(grid[ptr] as i32);
		putchar(grid[ptr] as i32);
		putchar(grid[ptr] as i32);
	}

}
