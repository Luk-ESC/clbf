unsafe extern "C" {
	safe fn putchar(ch: i32) -> i32;
	safe fn getchar() -> i32;
}

fn main() {
	let mut grid = [0u8; 30000];
	let mut ptr = 15000;

	grid[ptr] += 13;
	grid[ptr + 1] += 2 * grid[ptr];
	grid[ptr + 4] += 5 * grid[ptr];
	grid[ptr + 5] += 2 * grid[ptr];
	grid[ptr + 6] += grid[ptr];
	grid[ptr] = 0;
	grid[ptr + 5] += 6;
	grid[ptr + 6] -= 3;
	grid[ptr + 16] += 15;
	ptr += 16;

	while grid[ptr] != 0 {

		while grid[ptr] != 0 {
			ptr += 9;
		}

		grid[ptr] += 1;

		while grid[ptr] != 0 {
			ptr -= 9;
		}

		ptr += 9;
		grid[ptr] -= 1;
	}

	grid[ptr] += 1;

	while grid[ptr] != 0 {
		grid[ptr + 8] = 0;
		ptr += 9;
	}

	ptr -= 9;

	while grid[ptr] != 0 {
		ptr -= 9;
	}

	grid[ptr + 8] = 1;
	grid[ptr + 1] += 5;
	ptr += 1;

	while grid[ptr] != 0 {
		grid[ptr] -= 1;
		grid[ptr + 9] += grid[ptr];
		grid[ptr] = 0;
		ptr += 9;
	}

	grid[ptr + 7] += 1;
	grid[ptr + 34] += 1;
	ptr += 17;

	while grid[ptr] != 0 {
		ptr -= 9;
	}

	ptr += 3;
	grid[ptr] = 1;

	while grid[ptr] != 0 {
		ptr += 6;

		while grid[ptr] != 0 {
			grid[ptr + 7] = 0;
			ptr += 9;
		}

		ptr -= 9;

		while grid[ptr] != 0 {
			ptr -= 9;
		}

		grid[ptr + 7] = 1;
		grid[ptr + 1] += 4;
		ptr += 1;

		while grid[ptr] != 0 {
			grid[ptr] -= 1;
			grid[ptr + 9] += grid[ptr];
			grid[ptr] = 0;
			ptr += 9;
		}

		grid[ptr + 6] += 1;
		grid[ptr] += 7;

		while grid[ptr] != 0 {
			grid[ptr] -= 1;
			grid[ptr + 9] += grid[ptr];
			grid[ptr] = 0;
			ptr += 9;
		}

		grid[ptr + 6] += 1;
		ptr -= 10;

		while grid[ptr] != 0 {
			ptr -= 9;
		}

		ptr += 3;

		while grid[ptr] != 0 {
			grid[ptr] = 0;
			ptr += 6;

			while grid[ptr] != 0 {
				grid[ptr + 1] += grid[ptr + 7];
				grid[ptr + 7] = 0;
				grid[ptr + 7] += grid[ptr + 1];
				grid[ptr + 5] += grid[ptr + 1];
				grid[ptr + 2] += grid[ptr + 1];
				grid[ptr + 1] = 0;
				ptr += 9;
			}

			ptr -= 9;

			while grid[ptr] != 0 {
				ptr -= 9;
			}

			ptr += 9;

			while grid[ptr] != 0 {
				grid[ptr + 1] += grid[ptr + 8];
				grid[ptr + 8] = 0;
				grid[ptr + 8] += grid[ptr + 1];
				grid[ptr + 6] += grid[ptr + 1];
				grid[ptr + 3] += grid[ptr + 1];
				grid[ptr + 1] = 0;
				ptr += 9;
			}

			ptr -= 9;

			while grid[ptr] != 0 {
				ptr -= 9;
			}

			grid[ptr] += grid[ptr + 7];
			grid[ptr + 7] = 0;
			grid[ptr + 7] += grid[ptr];
			grid[ptr + 5] += grid[ptr];
			grid[ptr] = 0;
			grid[ptr + 9] += 15;
			ptr += 9;

			while grid[ptr] != 0 {

				while grid[ptr] != 0 {
					ptr += 9;
				}

				grid[ptr] += 1;
				grid[ptr + 1] = 0;
				grid[ptr + 2] = 0;
				grid[ptr + 3] = 0;
				grid[ptr + 4] = 0;
				grid[ptr + 5] = 0;
				grid[ptr + 6] = 0;
				grid[ptr + 7] = 0;
				grid[ptr + 8] = 0;
				grid[ptr + 9] = 0;

				while grid[ptr] != 0 {
					ptr -= 9;
				}

				ptr += 9;
				grid[ptr] -= 1;
			}

			grid[ptr] += 1;

			while grid[ptr] != 0 {
				grid[ptr + 1] += 1;
				ptr += 9;
			}

			ptr -= 9;

			while grid[ptr] != 0 {
				ptr -= 9;
			}

			ptr += 9;

			while grid[ptr] != 0 {
				grid[ptr + 1] -= 1;
				grid[ptr + 1] += grid[ptr + 5];
				grid[ptr + 5] = 0;
				ptr += 1;

				while grid[ptr] != 0 {
					grid[ptr] -= 1;
					grid[ptr + 4] += 1;
					ptr -= 1;

					while grid[ptr] != 0 {
						grid[ptr] -= 1;
						grid[ptr] += grid[ptr + 2];
						grid[ptr + 2] = 0;
						grid[ptr + 2] += grid[ptr];
						grid[ptr + 4] += grid[ptr];
						grid[ptr] = 0;
						grid[ptr] += 1;
						ptr += 9;
					}

					ptr -= 8;

					while grid[ptr] != 0 {
						ptr -= 9;
					}

				}

				ptr += 9;

				while grid[ptr] != 0 {
					ptr += 9;
				}

				ptr -= 9;

				while grid[ptr] != 0 {
					grid[ptr + 10] += grid[ptr + 1];
					grid[ptr + 1] = 0;
					ptr -= 9;
				}

				grid[ptr + 10] += grid[ptr + 1];
				grid[ptr + 1] = 0;
				grid[ptr] += 1;
				ptr += 8;
			}

			ptr -= 9;

			while grid[ptr] != 0 {
				grid[ptr + 1] = 0;
				grid[ptr] -= 1;
				ptr += 4;

				while grid[ptr] != 0 {
					grid[ptr] -= 1;
					grid[ptr - 4] += 1;
					grid[ptr - 4] -= grid[ptr - 3];
					grid[ptr - 9] += grid[ptr - 3];
					grid[ptr - 3] = 0;
					grid[ptr - 3] += grid[ptr - 4];
					grid[ptr - 4] = 0;
				}

				grid[ptr] += grid[ptr - 3];
				grid[ptr - 3] = 0;
				grid[ptr - 4] += 1;
				ptr -= 13;
			}

			ptr += 9;

			while grid[ptr] != 0 {
				grid[ptr + 1] += 1;
				ptr += 9;
			}

			ptr -= 9;

			while grid[ptr] != 0 {
				ptr -= 9;
			}

			ptr += 9;

			while grid[ptr] != 0 {
				grid[ptr + 1] -= 1;
				grid[ptr + 1] += grid[ptr + 6];
				grid[ptr + 6] = 0;
				ptr += 1;

				while grid[ptr] != 0 {
					grid[ptr] -= 1;
					grid[ptr + 5] += 1;
					ptr -= 1;

					while grid[ptr] != 0 {
						grid[ptr] -= 1;
						grid[ptr] += grid[ptr + 3];
						grid[ptr + 3] = 0;
						grid[ptr + 3] += grid[ptr];
						grid[ptr + 4] += grid[ptr];
						grid[ptr] = 0;
						grid[ptr] += 1;
						ptr += 9;
					}

					ptr -= 8;

					while grid[ptr] != 0 {
						ptr -= 9;
					}

				}

				ptr += 9;

				while grid[ptr] != 0 {
					ptr += 9;
				}

				ptr -= 9;

				while grid[ptr] != 0 {
					grid[ptr + 11] += grid[ptr + 2];
					grid[ptr + 2] = 0;
					ptr -= 9;
				}

				grid[ptr + 11] += grid[ptr + 2];
				grid[ptr + 2] = 0;
				grid[ptr] += 1;
				ptr += 8;
			}

			ptr -= 9;

			while grid[ptr] != 0 {
				grid[ptr + 1] = 0;
				grid[ptr] -= 1;
				ptr += 4;

				while grid[ptr] != 0 {
					grid[ptr] -= 1;
					grid[ptr - 4] += 1;
					grid[ptr - 4] -= grid[ptr - 3];
					grid[ptr - 9] += grid[ptr - 3];
					grid[ptr - 3] = 0;
					grid[ptr - 3] += grid[ptr - 4];
					grid[ptr - 4] = 0;
				}

				grid[ptr] += grid[ptr - 3];
				grid[ptr - 3] = 0;
				grid[ptr - 4] += 1;
				ptr -= 13;
			}

			ptr += 9;

			while grid[ptr] != 0 {
				grid[ptr - 32] += grid[ptr + 4];
				grid[ptr + 4] = 0;
				ptr += 9;
			}

			ptr -= 9;

			while grid[ptr] != 0 {
				ptr -= 9;
			}

			ptr += 9;
			grid[ptr] += 15;

			while grid[ptr] != 0 {

				while grid[ptr] != 0 {
					ptr += 9;
				}

				grid[ptr - 9] -= 1;
				ptr -= 18;

				while grid[ptr] != 0 {
					ptr -= 9;
				}

				ptr += 9;
				grid[ptr] -= 1;
			}

			grid[ptr] += 1;
			grid[ptr + 21] += 1;
			ptr += 18;

			while grid[ptr] != 0 {
				ptr -= 9;
			}

			ptr += 9;

			while grid[ptr] != 0 {
				grid[ptr] -= grid[ptr + 3];
				grid[ptr + 3] = 0;
				grid[ptr + 3] += 1;

				while grid[ptr] != 0 {
					grid[ptr] -= 1;
					grid[ptr + 3] -= 1;
					grid[ptr] += grid[ptr + 4];
					grid[ptr + 4] = 0;

					while grid[ptr] != 0 {
						grid[ptr] -= 1;
						grid[ptr + 4] += 1;
						ptr -= 9;

						while grid[ptr] != 0 {
							ptr -= 9;
						}

						grid[ptr + 4] = 1;
						ptr += 9;

						while grid[ptr] != 0 {
							ptr += 9;
						}

						grid[ptr + 1] += 1;
					}

				}

				grid[ptr] += 1;
				grid[ptr] -= grid[ptr + 4];
				grid[ptr + 4] = 0;
				grid[ptr + 4] += 1;

				while grid[ptr] != 0 {
					grid[ptr] -= 1;
					grid[ptr + 4] -= 1;
					grid[ptr] += grid[ptr + 3];
					grid[ptr + 3] = 0;

					while grid[ptr] != 0 {
						grid[ptr] -= 1;
						grid[ptr + 3] += 1;
						ptr -= 9;

						while grid[ptr] != 0 {
							ptr -= 9;
						}

						grid[ptr + 3] = 1;
						ptr += 9;

						while grid[ptr] != 0 {
							ptr += 9;
						}

						grid[ptr + 1] = 1;
					}

				}

				grid[ptr] += 1;
				ptr += 1;

				while grid[ptr] != 0 {
					grid[ptr] -= 1;
					ptr -= 1;

					while grid[ptr] != 0 {
						ptr += 9;
					}

					ptr -= 8;
				}

				ptr += 8;
			}

			ptr -= 9;

			while grid[ptr] != 0 {
				ptr -= 9;
			}

			grid[ptr - 6] += grid[ptr - 7];
			grid[ptr - 3] -= grid[ptr - 7];
			grid[ptr - 7] = 0;
			grid[ptr + 2] += 26;
			grid[ptr] += grid[ptr + 4];
			grid[ptr + 4] = 0;

			while grid[ptr] != 0 {
				grid[ptr] -= 1;
				grid[ptr + 4] += 1;
				grid[ptr + 2] = 0;
			}

			ptr += 2;

			while grid[ptr] != 0 {
				grid[ptr - 7] += 1;
				ptr -= 8;

				while grid[ptr] != 0 {
					grid[ptr] -= 1;
					grid[ptr - 1] += 1;
					grid[ptr + 3] += 1;
					grid[ptr + 1] = 0;
					ptr += 1;
				}

				ptr += 1;

				while grid[ptr] != 0 {
					grid[ptr] -= 1;
					grid[ptr - 1] += grid[ptr - 2];
					grid[ptr + 2] -= grid[ptr - 2];
					grid[ptr - 2] = 0;
					ptr += 1;
				}

				ptr += 13;

				while grid[ptr] != 0 {
					grid[ptr + 2] = 0;
					grid[ptr + 3] = 0;
					grid[ptr + 4] = 0;
					ptr += 9;
				}

				ptr -= 9;

				while grid[ptr] != 0 {
					ptr -= 9;
				}

				grid[ptr + 3] = 0;
				ptr += 9;

				while grid[ptr] != 0 {
					grid[ptr + 1] += grid[ptr + 5];
					grid[ptr + 5] = 0;
					grid[ptr + 5] += grid[ptr + 1];
					grid[ptr + 2] += grid[ptr + 1];
					grid[ptr + 1] = 0;
					ptr += 9;
				}

				ptr -= 9;

				while grid[ptr] != 0 {
					ptr -= 9;
				}

				ptr += 9;

				while grid[ptr] != 0 {
					grid[ptr - 7] += grid[ptr + 2];
					grid[ptr + 2] = 0;
					ptr += 9;
				}

				ptr -= 9;

				while grid[ptr] != 0 {
					ptr -= 9;
				}

				ptr += 9;
				grid[ptr] += 15;

				while grid[ptr] != 0 {

					while grid[ptr] != 0 {
						ptr += 9;
					}

					grid[ptr] += 1;
					grid[ptr + 1] = 0;
					grid[ptr + 2] = 0;
					grid[ptr + 3] = 0;
					grid[ptr + 4] = 0;
					grid[ptr + 5] = 0;
					grid[ptr + 6] = 0;
					grid[ptr + 7] = 0;
					grid[ptr + 8] = 0;
					grid[ptr + 9] = 0;

					while grid[ptr] != 0 {
						ptr -= 9;
					}

					ptr += 9;
					grid[ptr] -= 1;
				}

				grid[ptr] += 1;

				while grid[ptr] != 0 {
					grid[ptr + 1] += 1;
					ptr += 9;
				}

				ptr -= 9;

				while grid[ptr] != 0 {
					ptr -= 9;
				}

				ptr += 9;

				while grid[ptr] != 0 {
					grid[ptr + 1] -= 1;
					grid[ptr + 1] += grid[ptr + 6];
					grid[ptr + 6] = 0;
					ptr += 1;

					while grid[ptr] != 0 {
						grid[ptr] -= 1;
						grid[ptr + 5] += 1;
						ptr -= 1;

						while grid[ptr] != 0 {
							grid[ptr] -= 1;
							grid[ptr] += grid[ptr + 2];
							grid[ptr + 2] = 0;
							grid[ptr + 2] += grid[ptr];
							grid[ptr + 3] += grid[ptr];
							grid[ptr] = 0;
							grid[ptr] += 1;
							ptr += 9;
						}

						ptr -= 8;

						while grid[ptr] != 0 {
							ptr -= 9;
						}

					}

					ptr += 9;

					while grid[ptr] != 0 {
						ptr += 9;
					}

					ptr -= 9;

					while grid[ptr] != 0 {
						grid[ptr + 10] += grid[ptr + 1];
						grid[ptr + 1] = 0;
						ptr -= 9;
					}

					grid[ptr + 10] += grid[ptr + 1];
					grid[ptr + 1] = 0;
					grid[ptr] += 1;
					ptr += 8;
				}

				ptr -= 9;

				while grid[ptr] != 0 {
					grid[ptr + 1] = 0;
					grid[ptr] -= 1;
					ptr += 3;

					while grid[ptr] != 0 {
						grid[ptr] -= 1;
						grid[ptr - 3] += 1;
						grid[ptr - 3] -= grid[ptr - 2];
						grid[ptr - 9] += grid[ptr - 2];
						grid[ptr - 2] = 0;
						grid[ptr - 2] += grid[ptr - 3];
						grid[ptr - 3] = 0;
					}

					grid[ptr] += grid[ptr - 2];
					grid[ptr - 2] = 0;
					grid[ptr - 3] += 1;
					ptr -= 12;
				}

				ptr += 9;

				while grid[ptr] != 0 {
					grid[ptr + 1] += grid[ptr + 6];
					grid[ptr + 6] = 0;
					grid[ptr + 6] += grid[ptr + 1];
					grid[ptr + 2] += grid[ptr + 1];
					grid[ptr + 1] = 0;
					ptr += 9;
				}

				ptr -= 9;

				while grid[ptr] != 0 {
					ptr -= 9;
				}

				ptr += 9;

				while grid[ptr] != 0 {
					grid[ptr + 1] += 1;
					ptr += 9;
				}

				ptr -= 9;

				while grid[ptr] != 0 {
					ptr -= 9;
				}

				ptr += 9;

				while grid[ptr] != 0 {
					grid[ptr + 1] -= 1;
					grid[ptr + 1] += grid[ptr + 6];
					grid[ptr + 6] = 0;
					ptr += 1;

					while grid[ptr] != 0 {
						grid[ptr] -= 1;
						grid[ptr + 5] += 1;
						ptr -= 1;

						while grid[ptr] != 0 {
							grid[ptr] -= 1;
							grid[ptr] += grid[ptr + 2];
							grid[ptr + 2] = 0;
							grid[ptr + 2] += grid[ptr];
							grid[ptr + 4] += grid[ptr];
							grid[ptr] = 0;
							grid[ptr] += 1;
							ptr += 9;
						}

						ptr -= 8;

						while grid[ptr] != 0 {
							ptr -= 9;
						}

					}

					ptr += 9;

					while grid[ptr] != 0 {
						ptr += 9;
					}

					ptr -= 9;

					while grid[ptr] != 0 {
						grid[ptr + 10] += grid[ptr + 1];
						grid[ptr + 1] = 0;
						ptr -= 9;
					}

					grid[ptr + 10] += grid[ptr + 1];
					grid[ptr + 1] = 0;
					grid[ptr] += 1;
					ptr += 8;
				}

				ptr -= 9;

				while grid[ptr] != 0 {
					grid[ptr + 1] = 0;
					grid[ptr] -= 1;
					ptr += 4;

					while grid[ptr] != 0 {
						grid[ptr] -= 1;
						grid[ptr - 4] += 1;
						grid[ptr - 4] -= grid[ptr - 3];
						grid[ptr - 9] += grid[ptr - 3];
						grid[ptr - 3] = 0;
						grid[ptr - 3] += grid[ptr - 4];
						grid[ptr - 4] = 0;
					}

					grid[ptr] += grid[ptr - 3];
					grid[ptr - 3] = 0;
					grid[ptr - 4] += 1;
					ptr -= 13;
				}

				ptr += 9;

				while grid[ptr] != 0 {
					grid[ptr - 32] += grid[ptr + 4];
					grid[ptr + 4] = 0;
					ptr += 9;
				}

				ptr -= 9;

				while grid[ptr] != 0 {
					ptr -= 9;
				}

				ptr += 9;

				while grid[ptr] != 0 {
					grid[ptr - 33] += grid[ptr + 3];
					grid[ptr + 3] = 0;
					ptr += 9;
				}

				ptr -= 9;

				while grid[ptr] != 0 {
					ptr -= 9;
				}

				ptr += 9;
				grid[ptr] += 15;

				while grid[ptr] != 0 {

					while grid[ptr] != 0 {
						ptr += 9;
					}

					grid[ptr - 9] -= 1;
					ptr -= 18;

					while grid[ptr] != 0 {
						ptr -= 9;
					}

					ptr += 9;
					grid[ptr] -= 1;
				}

				grid[ptr] += 1;

				while grid[ptr] != 0 {
					grid[ptr + 1] += grid[ptr + 8];
					grid[ptr + 8] = 0;
					grid[ptr + 8] += grid[ptr + 1];
					grid[ptr + 2] += grid[ptr + 1];
					grid[ptr + 1] = 0;
					ptr += 9;
				}

				ptr -= 9;

				while grid[ptr] != 0 {
					ptr -= 9;
				}

				ptr += 9;

				while grid[ptr] != 0 {
					grid[ptr + 6] = 0;
					ptr += 9;
				}

				ptr -= 9;

				while grid[ptr] != 0 {
					ptr -= 9;
				}

				grid[ptr + 4] += 1;
				grid[ptr + 4] -= grid[ptr + 5];
				grid[ptr] += grid[ptr + 5];
				grid[ptr + 5] = 0;
				ptr += 6;

				while grid[ptr] != 0 {
					grid[ptr] -= 1;
					grid[ptr - 1] += grid[ptr - 6];
					grid[ptr - 2] += 2 * grid[ptr];
					grid[ptr - 6] = 0;
					grid[ptr - 6] += grid[ptr - 1];
					grid[ptr - 1] = 0;
					grid[ptr - 2] -= 1;
					grid[ptr - 1] += 1;
				}

				grid[ptr] += grid[ptr - 1];
				grid[ptr - 1] = 0;
				grid[ptr - 1] += grid[ptr - 6];
				grid[ptr - 6] = 0;
				grid[ptr] = 0;
				grid[ptr - 6] += 1;
				grid[ptr - 6] -= grid[ptr - 2];
				grid[ptr - 2] = 0;
				grid[ptr - 2] += 1;
				ptr -= 6;

				while grid[ptr] != 0 {
					grid[ptr] -= 1;
					grid[ptr + 4] -= 1;
					ptr += 9;

					while grid[ptr] != 0 {
						grid[ptr] -= grid[ptr + 2];
						grid[ptr + 2] = 0;
						grid[ptr + 2] += 1;

						while grid[ptr] != 0 {
							grid[ptr] -= 1;
							grid[ptr + 2] -= 1;
							grid[ptr] += grid[ptr + 3];
							grid[ptr + 3] = 0;

							while grid[ptr] != 0 {
								grid[ptr] -= 1;
								grid[ptr + 3] += 1;
								ptr -= 9;

								while grid[ptr] != 0 {
									ptr -= 9;
								}

								grid[ptr + 3] = 1;
								ptr += 9;

								while grid[ptr] != 0 {
									ptr += 9;
								}

								grid[ptr + 1] += 1;
							}

						}

						grid[ptr] += 1;
						grid[ptr] -= grid[ptr + 3];
						grid[ptr + 3] = 0;
						grid[ptr + 3] += 1;

						while grid[ptr] != 0 {
							grid[ptr] -= 1;
							grid[ptr + 3] -= 1;
							grid[ptr] += grid[ptr + 2];
							grid[ptr + 2] = 0;

							while grid[ptr] != 0 {
								grid[ptr] -= 1;
								grid[ptr + 2] += 1;
								ptr -= 9;

								while grid[ptr] != 0 {
									ptr -= 9;
								}

								grid[ptr + 4] = 1;
								ptr += 9;

								while grid[ptr] != 0 {
									ptr += 9;
								}

								grid[ptr + 1] = 1;
							}

						}

						grid[ptr] += 1;
						ptr += 1;

						while grid[ptr] != 0 {
							grid[ptr] -= 1;
							ptr -= 1;

							while grid[ptr] != 0 {
								ptr += 9;
							}

							ptr -= 8;
						}

						ptr += 8;
					}

					ptr -= 9;

					while grid[ptr] != 0 {
						ptr -= 9;
					}

					grid[ptr] += grid[ptr + 4];
					grid[ptr + 4] = 0;

					while grid[ptr] != 0 {
						grid[ptr] -= 1;
						grid[ptr + 4] += 1;
						ptr += 9;

						while grid[ptr] != 0 {
							grid[ptr + 1] += 1;
							grid[ptr + 1] -= grid[ptr + 3];
							grid[ptr + 3] = 0;
							grid[ptr + 3] += grid[ptr + 1];
							grid[ptr + 1] = 0;
							ptr += 9;
						}

						grid[ptr - 8] += 1;
						ptr -= 9;

						while grid[ptr] != 0 {
							ptr += 1;

							while grid[ptr] != 0 {
								grid[ptr] -= 1;
								grid[ptr + 5] += 1;
								ptr += 1;

								while grid[ptr] != 0 {
									grid[ptr] -= 1;
									grid[ptr + 4] -= 1;
									grid[ptr - 10] += 1;
									grid[ptr + 4] += grid[ptr + 1];
									grid[ptr + 1] = 0;
								}

								grid[ptr + 4] -= grid[ptr + 1];
								grid[ptr - 10] += grid[ptr + 1];
								grid[ptr + 1] = 0;
								ptr -= 1;
							}

							ptr += 1;

							while grid[ptr] != 0 {
								grid[ptr] -= 1;
								grid[ptr + 4] += 1;
								grid[ptr + 4] -= grid[ptr + 1];
								grid[ptr - 10] += grid[ptr + 1];
								grid[ptr + 1] = 0;
							}

							grid[ptr + 4] += grid[ptr + 1];
							grid[ptr + 1] = 0;
							ptr -= 11;
						}

						grid[ptr + 4] = 0;
					}

					grid[ptr] += grid[ptr + 3];
					grid[ptr + 3] = 0;

					while grid[ptr] != 0 {
						grid[ptr] -= 1;
						grid[ptr + 3] += 1;
						ptr += 9;

						while grid[ptr] != 0 {
							grid[ptr + 1] += 1;
							grid[ptr + 1] -= grid[ptr + 2];
							grid[ptr + 2] = 0;
							grid[ptr + 2] += grid[ptr + 1];
							grid[ptr + 1] = 0;
							ptr += 9;
						}

						grid[ptr - 8] += 1;
						ptr -= 9;

						while grid[ptr] != 0 {
							ptr += 1;

							while grid[ptr] != 0 {
								grid[ptr] -= 1;
								grid[ptr + 5] += 1;
								ptr += 2;

								while grid[ptr] != 0 {
									grid[ptr] -= 1;
									grid[ptr + 3] -= 1;
									grid[ptr - 11] += 1;
									grid[ptr + 3] += grid[ptr - 1];
									grid[ptr - 1] = 0;
								}

								grid[ptr + 3] -= grid[ptr - 1];
								grid[ptr - 11] += grid[ptr - 1];
								grid[ptr - 1] = 0;
								ptr -= 2;
							}

							ptr += 2;

							while grid[ptr] != 0 {
								grid[ptr] -= 1;
								grid[ptr + 3] += 1;
								grid[ptr + 3] -= grid[ptr - 1];
								grid[ptr - 11] += grid[ptr - 1];
								grid[ptr - 1] = 0;
							}

							grid[ptr + 3] += grid[ptr - 1];
							grid[ptr - 1] = 0;
							ptr -= 12;
						}

						grid[ptr + 6] += 1;
					}

				}

				grid[ptr] += grid[ptr + 4];
				grid[ptr + 4] = 0;

				while grid[ptr] != 0 {
					grid[ptr] -= 1;
					grid[ptr + 4] += 1;
					ptr += 9;

					while grid[ptr] != 0 {
						ptr += 9;
					}

					ptr -= 9;

					while grid[ptr] != 0 {
						ptr += 1;

						while grid[ptr] != 0 {
							grid[ptr] -= 1;
							grid[ptr + 5] += 1;
							ptr += 1;

							while grid[ptr] != 0 {
								grid[ptr] -= 1;
								grid[ptr + 4] -= 1;
								grid[ptr - 10] += 1;
								grid[ptr + 4] += grid[ptr + 1];
								grid[ptr + 1] = 0;
							}

							grid[ptr + 4] -= grid[ptr + 1];
							grid[ptr - 10] += grid[ptr + 1];
							grid[ptr + 1] = 0;
							ptr -= 1;
						}

						ptr += 1;

						while grid[ptr] != 0 {
							grid[ptr] -= 1;
							grid[ptr + 4] += 1;
							grid[ptr + 4] -= grid[ptr + 1];
							grid[ptr - 10] += grid[ptr + 1];
							grid[ptr + 1] = 0;
						}

						grid[ptr + 4] += grid[ptr + 1];
						grid[ptr + 1] = 0;
						ptr -= 11;
					}

				}

				grid[ptr + 1] = 0;
				grid[ptr + 3] = 0;
				grid[ptr + 4] = 0;
				ptr += 9;

				while grid[ptr] != 0 {
					grid[ptr + 2] = 0;
					grid[ptr + 3] = 0;
					ptr += 9;
				}

				ptr -= 9;

				while grid[ptr] != 0 {
					ptr -= 9;
				}

				ptr += 9;

				while grid[ptr] != 0 {
					grid[ptr + 1] += grid[ptr + 5];
					grid[ptr + 5] = 0;
					grid[ptr + 5] += grid[ptr + 1];
					grid[ptr + 2] += grid[ptr + 1];
					grid[ptr + 1] = 0;
					ptr += 9;
				}

				ptr -= 9;

				while grid[ptr] != 0 {
					ptr -= 9;
				}

				ptr += 9;
				grid[ptr] += 15;

				while grid[ptr] != 0 {

					while grid[ptr] != 0 {
						ptr += 9;
					}

					grid[ptr] += 1;
					grid[ptr + 1] = 0;
					grid[ptr + 2] = 0;
					grid[ptr + 3] = 0;
					grid[ptr + 4] = 0;
					grid[ptr + 5] = 0;
					grid[ptr + 6] = 0;
					grid[ptr + 7] = 0;
					grid[ptr + 8] = 0;
					grid[ptr + 9] = 0;

					while grid[ptr] != 0 {
						ptr -= 9;
					}

					ptr += 9;
					grid[ptr] -= 1;
				}

				grid[ptr] += 1;

				while grid[ptr] != 0 {
					grid[ptr + 1] += 1;
					ptr += 9;
				}

				ptr -= 9;

				while grid[ptr] != 0 {
					ptr -= 9;
				}

				ptr += 9;

				while grid[ptr] != 0 {
					grid[ptr + 1] -= 1;
					grid[ptr + 1] += grid[ptr + 5];
					grid[ptr + 5] = 0;
					ptr += 1;

					while grid[ptr] != 0 {
						grid[ptr] -= 1;
						grid[ptr + 4] += 1;
						ptr -= 1;

						while grid[ptr] != 0 {
							grid[ptr] -= 1;
							grid[ptr] += grid[ptr + 2];
							grid[ptr + 2] = 0;
							grid[ptr + 2] += grid[ptr];
							grid[ptr + 3] += grid[ptr];
							grid[ptr] = 0;
							grid[ptr] += 1;
							ptr += 9;
						}

						ptr -= 8;

						while grid[ptr] != 0 {
							ptr -= 9;
						}

					}

					ptr += 9;

					while grid[ptr] != 0 {
						ptr += 9;
					}

					ptr -= 9;

					while grid[ptr] != 0 {
						grid[ptr + 10] += grid[ptr + 1];
						grid[ptr + 1] = 0;
						ptr -= 9;
					}

					grid[ptr + 10] += grid[ptr + 1];
					grid[ptr + 1] = 0;
					grid[ptr] += 1;
					ptr += 8;
				}

				ptr -= 9;

				while grid[ptr] != 0 {
					grid[ptr + 1] = 0;
					grid[ptr] -= 1;
					ptr += 3;

					while grid[ptr] != 0 {
						grid[ptr] -= 1;
						grid[ptr - 3] += 1;
						grid[ptr - 3] -= grid[ptr - 2];
						grid[ptr - 9] += grid[ptr - 2];
						grid[ptr - 2] = 0;
						grid[ptr - 2] += grid[ptr - 3];
						grid[ptr - 3] = 0;
					}

					grid[ptr] += grid[ptr - 2];
					grid[ptr - 2] = 0;
					grid[ptr - 3] += 1;
					ptr -= 12;
				}

				ptr += 9;

				while grid[ptr] != 0 {
					grid[ptr - 33] += grid[ptr + 3];
					grid[ptr + 3] = 0;
					ptr += 9;
				}

				ptr -= 9;

				while grid[ptr] != 0 {
					ptr -= 9;
				}

				grid[ptr + 5] = 0;
				grid[ptr + 9] += 15;
				ptr += 9;

				while grid[ptr] != 0 {

					while grid[ptr] != 0 {
						ptr += 9;
					}

					grid[ptr - 9] -= 1;
					ptr -= 18;

					while grid[ptr] != 0 {
						ptr -= 9;
					}

					ptr += 9;
					grid[ptr] -= 1;
				}

				grid[ptr] += 1;

				while grid[ptr] != 0 {
					grid[ptr] -= grid[ptr + 3];
					grid[ptr + 3] = 0;
					grid[ptr + 3] += 1;

					while grid[ptr] != 0 {
						grid[ptr] -= 1;
						grid[ptr + 3] -= 1;
						grid[ptr] += grid[ptr + 4];
						grid[ptr + 4] = 0;

						while grid[ptr] != 0 {
							grid[ptr] -= 1;
							grid[ptr + 4] += 1;
							ptr -= 9;

							while grid[ptr] != 0 {
								ptr -= 9;
							}

							grid[ptr + 4] = 1;
							ptr += 9;

							while grid[ptr] != 0 {
								ptr += 9;
							}

							grid[ptr + 1] += 1;
						}

					}

					grid[ptr] += 1;
					grid[ptr] -= grid[ptr + 4];
					grid[ptr + 4] = 0;
					grid[ptr + 4] += 1;

					while grid[ptr] != 0 {
						grid[ptr] -= 1;
						grid[ptr + 4] -= 1;
						grid[ptr] += grid[ptr + 3];
						grid[ptr + 3] = 0;

						while grid[ptr] != 0 {
							grid[ptr] -= 1;
							grid[ptr + 3] += 1;
							ptr -= 9;

							while grid[ptr] != 0 {
								ptr -= 9;
							}

							grid[ptr + 3] = 1;
							ptr += 9;

							while grid[ptr] != 0 {
								ptr += 9;
							}

							grid[ptr + 1] = 1;
						}

					}

					grid[ptr] += 1;
					ptr += 1;

					while grid[ptr] != 0 {
						grid[ptr] -= 1;
						ptr -= 1;

						while grid[ptr] != 0 {
							ptr += 9;
						}

						ptr -= 8;
					}

					ptr += 8;
				}

				ptr -= 9;

				while grid[ptr] != 0 {
					ptr -= 9;
				}

				grid[ptr] += grid[ptr + 3];
				grid[ptr + 3] = 0;

				while grid[ptr] != 0 {
					grid[ptr] -= 1;
					grid[ptr + 3] += 1;
					ptr += 9;

					while grid[ptr] != 0 {
						grid[ptr + 1] += 1;
						grid[ptr + 1] -= grid[ptr + 4];
						grid[ptr + 4] = 0;
						grid[ptr + 4] += grid[ptr + 1];
						grid[ptr + 1] = 0;
						ptr += 9;
					}

					grid[ptr - 8] += 1;
					ptr -= 9;

					while grid[ptr] != 0 {
						ptr += 1;

						while grid[ptr] != 0 {
							grid[ptr] -= 1;
							grid[ptr + 1] += 1;
							ptr += 2;

							while grid[ptr] != 0 {
								grid[ptr] -= 1;
								grid[ptr - 1] -= 1;
								grid[ptr - 11] += 1;
								grid[ptr - 1] += grid[ptr + 1];
								grid[ptr + 1] = 0;
							}

							grid[ptr - 1] -= grid[ptr + 1];
							grid[ptr - 11] += grid[ptr + 1];
							grid[ptr + 1] = 0;
							ptr -= 2;
						}

						ptr += 2;

						while grid[ptr] != 0 {
							grid[ptr] -= 1;
							grid[ptr - 1] += 1;
							grid[ptr - 1] -= grid[ptr + 1];
							grid[ptr - 11] += grid[ptr + 1];
							grid[ptr + 1] = 0;
						}

						grid[ptr - 1] += grid[ptr + 1];
						grid[ptr + 1] = 0;
						ptr -= 12;
					}

				}

				grid[ptr] += grid[ptr + 4];
				grid[ptr + 4] = 0;

				while grid[ptr] != 0 {
					grid[ptr] -= 1;
					grid[ptr + 4] += 1;
					ptr += 9;

					while grid[ptr] != 0 {
						grid[ptr + 1] += 1;
						grid[ptr + 1] -= grid[ptr + 3];
						grid[ptr + 3] = 0;
						grid[ptr + 3] += grid[ptr + 1];
						grid[ptr + 1] = 0;
						ptr += 9;
					}

					grid[ptr - 8] += 1;
					ptr -= 9;

					while grid[ptr] != 0 {
						ptr += 1;

						while grid[ptr] != 0 {
							grid[ptr] -= 1;
							grid[ptr + 1] += 1;
							ptr += 3;

							while grid[ptr] != 0 {
								grid[ptr] -= 1;
								grid[ptr - 2] -= 1;
								grid[ptr - 12] += 1;
								grid[ptr - 2] += grid[ptr - 1];
								grid[ptr - 1] = 0;
							}

							grid[ptr - 2] -= grid[ptr - 1];
							grid[ptr - 12] += grid[ptr - 1];
							grid[ptr - 1] = 0;
							ptr -= 3;
						}

						ptr += 3;

						while grid[ptr] != 0 {
							grid[ptr] -= 1;
							grid[ptr - 2] += 1;
							grid[ptr - 2] -= grid[ptr - 1];
							grid[ptr - 12] += grid[ptr - 1];
							grid[ptr - 1] = 0;
						}

						grid[ptr - 2] += grid[ptr - 1];
						grid[ptr - 1] = 0;
						ptr -= 13;
					}

					grid[ptr + 5] += 1;
				}

				ptr += 9;

				while grid[ptr] != 0 {
					grid[ptr + 3] = 0;
					grid[ptr + 4] = 0;
					grid[ptr + 5] = 0;
					ptr += 9;
				}

				ptr -= 9;

				while grid[ptr] != 0 {
					ptr -= 9;
				}

				grid[ptr + 3] = 0;
				grid[ptr + 4] = 0;
				ptr += 9;

				while grid[ptr] != 0 {
					grid[ptr + 1] += grid[ptr + 7];
					grid[ptr + 7] = 0;
					grid[ptr + 7] += grid[ptr + 1];
					grid[ptr + 3] += grid[ptr + 1];
					grid[ptr + 1] = 0;
					ptr += 9;
				}

				ptr -= 9;

				while grid[ptr] != 0 {
					ptr -= 9;
				}

				grid[ptr + 4] += 1;
				grid[ptr + 4] -= grid[ptr + 5];
				grid[ptr] += grid[ptr + 5];
				grid[ptr + 5] = 0;
				ptr += 7;

				while grid[ptr] != 0 {
					grid[ptr] -= 1;
					grid[ptr - 2] += grid[ptr - 7];
					grid[ptr - 3] += 2 * grid[ptr];
					grid[ptr - 7] = 0;
					grid[ptr - 7] += grid[ptr - 2];
					grid[ptr - 2] = 0;
					grid[ptr - 3] -= 1;
					grid[ptr - 2] += 1;
				}

				grid[ptr] += grid[ptr - 2];
				grid[ptr - 2] = 0;
				grid[ptr - 2] += grid[ptr - 7];
				grid[ptr - 7] = 0;
				grid[ptr - 7] += 1;
				grid[ptr - 7] -= grid[ptr - 3];
				grid[ptr - 3] = 0;
				grid[ptr - 3] += 1;
				ptr -= 7;

				while grid[ptr] != 0 {
					grid[ptr] -= 1;
					grid[ptr + 4] -= 1;
					ptr += 9;

					while grid[ptr] != 0 {
						grid[ptr] -= grid[ptr + 3];
						grid[ptr + 3] = 0;
						grid[ptr + 3] += 1;

						while grid[ptr] != 0 {
							grid[ptr] -= 1;
							grid[ptr + 3] -= 1;
							grid[ptr] += grid[ptr + 2];
							grid[ptr + 2] = 0;

							while grid[ptr] != 0 {
								grid[ptr] -= 1;
								grid[ptr + 2] += 1;
								ptr -= 9;

								while grid[ptr] != 0 {
									ptr -= 9;
								}

								grid[ptr + 4] = 1;
								ptr += 9;

								while grid[ptr] != 0 {
									ptr += 9;
								}

								grid[ptr + 1] += 1;
							}

						}

						grid[ptr] += 1;
						grid[ptr] -= grid[ptr + 2];
						grid[ptr + 2] = 0;
						grid[ptr + 2] += 1;

						while grid[ptr] != 0 {
							grid[ptr] -= 1;
							grid[ptr + 2] -= 1;
							grid[ptr] += grid[ptr + 3];
							grid[ptr + 3] = 0;

							while grid[ptr] != 0 {
								grid[ptr] -= 1;
								grid[ptr + 3] += 1;
								ptr -= 9;

								while grid[ptr] != 0 {
									ptr -= 9;
								}

								grid[ptr + 3] = 1;
								ptr += 9;

								while grid[ptr] != 0 {
									ptr += 9;
								}

								grid[ptr + 1] = 1;
							}

						}

						grid[ptr] += 1;
						ptr += 1;

						while grid[ptr] != 0 {
							grid[ptr] -= 1;
							ptr -= 1;

							while grid[ptr] != 0 {
								ptr += 9;
							}

							ptr -= 8;
						}

						ptr += 8;
					}

					ptr -= 9;

					while grid[ptr] != 0 {
						ptr -= 9;
					}

					grid[ptr] += grid[ptr + 3];
					grid[ptr + 3] = 0;

					while grid[ptr] != 0 {
						grid[ptr] -= 1;
						grid[ptr + 3] += 1;
						ptr += 9;

						while grid[ptr] != 0 {
							grid[ptr + 1] += 1;
							grid[ptr + 1] -= grid[ptr + 2];
							grid[ptr + 2] = 0;
							grid[ptr + 2] += grid[ptr + 1];
							grid[ptr + 1] = 0;
							ptr += 9;
						}

						grid[ptr - 8] += 1;
						ptr -= 9;

						while grid[ptr] != 0 {
							ptr += 1;

							while grid[ptr] != 0 {
								grid[ptr] -= 1;
								grid[ptr + 4] += 1;
								ptr += 2;

								while grid[ptr] != 0 {
									grid[ptr] -= 1;
									grid[ptr + 2] -= 1;
									grid[ptr - 11] += 1;
									grid[ptr + 2] += grid[ptr - 1];
									grid[ptr - 1] = 0;
								}

								grid[ptr + 2] -= grid[ptr - 1];
								grid[ptr - 11] += grid[ptr - 1];
								grid[ptr - 1] = 0;
								ptr -= 2;
							}

							ptr += 2;

							while grid[ptr] != 0 {
								grid[ptr] -= 1;
								grid[ptr + 2] += 1;
								grid[ptr + 2] -= grid[ptr - 1];
								grid[ptr - 11] += grid[ptr - 1];
								grid[ptr - 1] = 0;
							}

							grid[ptr + 2] += grid[ptr - 1];
							grid[ptr - 1] = 0;
							ptr -= 12;
						}

						grid[ptr + 5] = 0;
						grid[ptr] += grid[ptr + 7];
						grid[ptr + 7] = 0;
						grid[ptr + 7] += grid[ptr];
						grid[ptr + 5] += grid[ptr];
						grid[ptr] = 0;
					}

					grid[ptr] += grid[ptr + 4];
					grid[ptr + 4] = 0;

					while grid[ptr] != 0 {
						grid[ptr] -= 1;
						grid[ptr + 4] += 1;
						ptr += 9;

						while grid[ptr] != 0 {
							grid[ptr + 1] += 1;
							grid[ptr + 1] -= grid[ptr + 3];
							grid[ptr + 3] = 0;
							grid[ptr + 3] += grid[ptr + 1];
							grid[ptr + 1] = 0;
							ptr += 9;
						}

						grid[ptr - 8] += 1;
						ptr -= 9;

						while grid[ptr] != 0 {
							ptr += 1;

							while grid[ptr] != 0 {
								grid[ptr] -= 1;
								grid[ptr + 4] += 1;
								ptr += 1;

								while grid[ptr] != 0 {
									grid[ptr] -= 1;
									grid[ptr + 3] -= 1;
									grid[ptr - 10] += 1;
									grid[ptr + 3] += grid[ptr + 1];
									grid[ptr + 1] = 0;
								}

								grid[ptr + 3] -= grid[ptr + 1];
								grid[ptr - 10] += grid[ptr + 1];
								grid[ptr + 1] = 0;
								ptr -= 1;
							}

							ptr += 1;

							while grid[ptr] != 0 {
								grid[ptr] -= 1;
								grid[ptr + 3] += 1;
								grid[ptr + 3] -= grid[ptr + 1];
								grid[ptr - 10] += grid[ptr + 1];
								grid[ptr + 1] = 0;
							}

							grid[ptr + 3] += grid[ptr + 1];
							grid[ptr + 1] = 0;
							ptr -= 11;
						}

					}

					grid[ptr + 4] = 0;
				}

				grid[ptr] += grid[ptr + 4];
				grid[ptr + 4] = 0;

				while grid[ptr] != 0 {
					grid[ptr] -= 1;
					grid[ptr + 4] += 1;
					grid[ptr + 5] = 0;
					grid[ptr] += grid[ptr + 7];
					grid[ptr + 7] = 0;
					grid[ptr + 7] += grid[ptr];
					grid[ptr + 5] += grid[ptr];
					grid[ptr] = 0;
					ptr += 9;

					while grid[ptr] != 0 {
						ptr += 9;
					}

					ptr -= 9;

					while grid[ptr] != 0 {
						ptr += 1;

						while grid[ptr] != 0 {
							grid[ptr] -= 1;
							grid[ptr + 4] += 1;
							ptr += 1;

							while grid[ptr] != 0 {
								grid[ptr] -= 1;
								grid[ptr + 3] -= 1;
								grid[ptr - 10] += 1;
								grid[ptr + 3] += grid[ptr + 1];
								grid[ptr + 1] = 0;
							}

							grid[ptr + 3] -= grid[ptr + 1];
							grid[ptr - 10] += grid[ptr + 1];
							grid[ptr + 1] = 0;
							ptr -= 1;
						}

						ptr += 1;

						while grid[ptr] != 0 {
							grid[ptr] -= 1;
							grid[ptr + 3] += 1;
							grid[ptr + 3] -= grid[ptr + 1];
							grid[ptr - 10] += grid[ptr + 1];
							grid[ptr + 1] = 0;
						}

						grid[ptr + 3] += grid[ptr + 1];
						grid[ptr + 1] = 0;
						ptr -= 11;
					}

				}

				ptr += 9;

				while grid[ptr] != 0 {
					grid[ptr + 2] = 0;
					grid[ptr + 3] = 0;
					ptr += 9;
				}

				ptr -= 9;

				while grid[ptr] != 0 {
					ptr -= 9;
				}

				grid[ptr + 3] = 0;
				grid[ptr + 4] = 0;
				ptr += 9;

				while grid[ptr] != 0 {
					grid[ptr + 1] += grid[ptr + 5];
					grid[ptr + 5] = 0;
					grid[ptr + 5] += grid[ptr + 1];
					grid[ptr + 2] += grid[ptr + 1];
					grid[ptr + 1] = 0;
					ptr += 9;
				}

				ptr -= 9;

				while grid[ptr] != 0 {
					ptr -= 9;
				}

				ptr += 9;

				while grid[ptr] != 0 {
					grid[ptr + 1] += grid[ptr + 6];
					grid[ptr + 6] = 0;
					grid[ptr + 6] += grid[ptr + 1];
					grid[ptr + 3] += grid[ptr + 1];
					grid[ptr + 1] = 0;
					ptr += 9;
				}

				ptr -= 9;

				while grid[ptr] != 0 {
					ptr -= 9;
				}

				ptr += 9;
				grid[ptr] += 15;

				while grid[ptr] != 0 {

					while grid[ptr] != 0 {
						ptr += 9;
					}

					grid[ptr] += 1;
					grid[ptr + 1] = 0;
					grid[ptr + 2] = 0;
					grid[ptr + 3] = 0;
					grid[ptr + 4] = 0;
					grid[ptr + 5] = 0;
					grid[ptr + 6] = 0;
					grid[ptr + 7] = 0;
					grid[ptr + 8] = 0;
					grid[ptr + 9] = 0;

					while grid[ptr] != 0 {
						ptr -= 9;
					}

					ptr += 9;
					grid[ptr] -= 1;
				}

				grid[ptr] += 1;

				while grid[ptr] != 0 {
					grid[ptr + 1] += 1;
					ptr += 9;
				}

				ptr -= 9;

				while grid[ptr] != 0 {
					ptr -= 9;
				}

				ptr += 9;

				while grid[ptr] != 0 {
					grid[ptr + 1] -= 1;
					grid[ptr + 1] += grid[ptr + 5];
					grid[ptr + 5] = 0;
					ptr += 1;

					while grid[ptr] != 0 {
						grid[ptr] -= 1;
						grid[ptr + 4] += 1;
						ptr -= 1;

						while grid[ptr] != 0 {
							grid[ptr] -= 1;
							grid[ptr] += grid[ptr + 2];
							grid[ptr + 2] = 0;
							grid[ptr + 2] += grid[ptr];
							grid[ptr + 4] += grid[ptr];
							grid[ptr] = 0;
							grid[ptr] += 1;
							ptr += 9;
						}

						ptr -= 8;

						while grid[ptr] != 0 {
							ptr -= 9;
						}

					}

					ptr += 9;

					while grid[ptr] != 0 {
						ptr += 9;
					}

					ptr -= 9;

					while grid[ptr] != 0 {
						grid[ptr + 10] += grid[ptr + 1];
						grid[ptr + 1] = 0;
						ptr -= 9;
					}

					grid[ptr + 10] += grid[ptr + 1];
					grid[ptr + 1] = 0;
					grid[ptr] += 1;
					ptr += 8;
				}

				ptr -= 9;

				while grid[ptr] != 0 {
					grid[ptr + 1] = 0;
					grid[ptr] -= 1;
					ptr += 4;

					while grid[ptr] != 0 {
						grid[ptr] -= 1;
						grid[ptr - 4] += 1;
						grid[ptr - 4] -= grid[ptr - 3];
						grid[ptr - 9] += grid[ptr - 3];
						grid[ptr - 3] = 0;
						grid[ptr - 3] += grid[ptr - 4];
						grid[ptr - 4] = 0;
					}

					grid[ptr] += grid[ptr - 3];
					grid[ptr - 3] = 0;
					grid[ptr - 4] += 1;
					ptr -= 13;
				}

				ptr += 9;

				while grid[ptr] != 0 {
					grid[ptr + 1] += 1;
					ptr += 9;
				}

				ptr -= 9;

				while grid[ptr] != 0 {
					ptr -= 9;
				}

				ptr += 9;

				while grid[ptr] != 0 {
					grid[ptr + 1] -= 1;
					grid[ptr + 1] += grid[ptr + 6];
					grid[ptr + 6] = 0;
					ptr += 1;

					while grid[ptr] != 0 {
						grid[ptr] -= 1;
						grid[ptr + 5] += 1;
						ptr -= 1;

						while grid[ptr] != 0 {
							grid[ptr] -= 1;
							grid[ptr] += grid[ptr + 3];
							grid[ptr + 3] = 0;
							grid[ptr + 3] += grid[ptr];
							grid[ptr + 4] += grid[ptr];
							grid[ptr] = 0;
							grid[ptr] += 1;
							ptr += 9;
						}

						ptr -= 8;

						while grid[ptr] != 0 {
							ptr -= 9;
						}

					}

					ptr += 9;

					while grid[ptr] != 0 {
						ptr += 9;
					}

					ptr -= 9;

					while grid[ptr] != 0 {
						grid[ptr + 11] += grid[ptr + 2];
						grid[ptr + 2] = 0;
						ptr -= 9;
					}

					grid[ptr + 11] += grid[ptr + 2];
					grid[ptr + 2] = 0;
					grid[ptr] += 1;
					ptr += 8;
				}

				ptr -= 9;

				while grid[ptr] != 0 {
					grid[ptr + 1] = 0;
					grid[ptr] -= 1;
					ptr += 4;

					while grid[ptr] != 0 {
						grid[ptr] -= 1;
						grid[ptr - 4] += 1;
						grid[ptr - 4] -= grid[ptr - 3];
						grid[ptr - 9] += grid[ptr - 3];
						grid[ptr - 3] = 0;
						grid[ptr - 3] += grid[ptr - 4];
						grid[ptr - 4] = 0;
					}

					grid[ptr] += grid[ptr - 3];
					grid[ptr - 3] = 0;
					grid[ptr - 4] += 1;
					ptr -= 13;
				}

				ptr += 9;

				while grid[ptr] != 0 {
					grid[ptr - 32] += grid[ptr + 4];
					grid[ptr + 4] = 0;
					ptr += 9;
				}

				ptr -= 9;

				while grid[ptr] != 0 {
					ptr -= 9;
				}

				ptr += 9;
				grid[ptr] += 15;

				while grid[ptr] != 0 {

					while grid[ptr] != 0 {
						ptr += 9;
					}

					grid[ptr - 9] -= 1;
					ptr -= 18;

					while grid[ptr] != 0 {
						ptr -= 9;
					}

					ptr += 9;
					grid[ptr] -= 1;
				}

				grid[ptr] += 1;
				grid[ptr + 21] += 1;
				ptr += 18;

				while grid[ptr] != 0 {
					ptr -= 9;
				}

				ptr += 9;

				while grid[ptr] != 0 {
					grid[ptr] -= grid[ptr + 3];
					grid[ptr + 3] = 0;
					grid[ptr + 3] += 1;

					while grid[ptr] != 0 {
						grid[ptr] -= 1;
						grid[ptr + 3] -= 1;
						grid[ptr] += grid[ptr + 4];
						grid[ptr + 4] = 0;

						while grid[ptr] != 0 {
							grid[ptr] -= 1;
							grid[ptr + 4] += 1;
							ptr -= 9;

							while grid[ptr] != 0 {
								ptr -= 9;
							}

							grid[ptr + 4] = 1;
							ptr += 9;

							while grid[ptr] != 0 {
								ptr += 9;
							}

							grid[ptr + 1] += 1;
						}

					}

					grid[ptr] += 1;
					grid[ptr] -= grid[ptr + 4];
					grid[ptr + 4] = 0;
					grid[ptr + 4] += 1;

					while grid[ptr] != 0 {
						grid[ptr] -= 1;
						grid[ptr + 4] -= 1;
						grid[ptr] += grid[ptr + 3];
						grid[ptr + 3] = 0;

						while grid[ptr] != 0 {
							grid[ptr] -= 1;
							grid[ptr + 3] += 1;
							ptr -= 9;

							while grid[ptr] != 0 {
								ptr -= 9;
							}

							grid[ptr + 3] = 1;
							ptr += 9;

							while grid[ptr] != 0 {
								ptr += 9;
							}

							grid[ptr + 1] = 1;
						}

					}

					grid[ptr] += 1;
					ptr += 1;

					while grid[ptr] != 0 {
						grid[ptr] -= 1;
						ptr -= 1;

						while grid[ptr] != 0 {
							ptr += 9;
						}

						ptr -= 8;
					}

					ptr += 8;
				}

				ptr -= 9;

				while grid[ptr] != 0 {
					ptr -= 9;
				}

				grid[ptr + 2] -= 1;
				grid[ptr] += grid[ptr + 4];
				grid[ptr + 4] = 0;

				while grid[ptr] != 0 {
					grid[ptr] -= 1;
					grid[ptr + 4] += 1;
					grid[ptr + 2] = 0;
				}

				ptr += 2;
			}

			grid[ptr - 2] += 1;
			grid[ptr - 2] -= grid[ptr + 2];
			grid[ptr + 2] = 0;
			grid[ptr + 2] += 1;
			ptr -= 2;

			while grid[ptr] != 0 {
				grid[ptr] -= 1;
				grid[ptr + 4] -= 1;
				ptr -= 2;
				putchar(grid[ptr] as i32);
				ptr += 2;
			}

			ptr += 4;

			while grid[ptr] != 0 {
				grid[ptr] -= 1;
				ptr -= 7;
				putchar(grid[ptr] as i32);
				ptr += 7;
			}

			grid[ptr - 3] = 0;
			grid[ptr - 2] = 0;
			grid[ptr - 1] = 0;
			grid[ptr] = 0;
			grid[ptr + 1] = 0;
			grid[ptr + 2] = 0;
			ptr += 5;

			while grid[ptr] != 0 {
				grid[ptr + 1] = 0;
				grid[ptr + 2] = 0;
				grid[ptr + 3] = 0;
				grid[ptr + 4] = 0;
				grid[ptr + 5] = 0;
				grid[ptr + 6] = 0;
				ptr += 9;
			}

			ptr -= 9;

			while grid[ptr] != 0 {
				ptr -= 9;
			}

			ptr += 9;

			while grid[ptr] != 0 {
				grid[ptr + 5] = 0;
				ptr += 9;
			}

			ptr -= 9;

			while grid[ptr] != 0 {
				ptr -= 9;
			}

			ptr += 1;
			grid[ptr] += 11;

			while grid[ptr] != 0 {
				grid[ptr] -= 1;
				grid[ptr + 9] += grid[ptr];
				grid[ptr] = 0;
				ptr += 9;
			}

			grid[ptr + 4] += 1;
			grid[ptr + 13] += 1;
			ptr -= 1;

			while grid[ptr] != 0 {
				ptr -= 9;
			}

			grid[ptr] += grid[ptr + 7];
			grid[ptr + 7] = 0;

			while grid[ptr] != 0 {
				grid[ptr] -= 1;
				grid[ptr + 7] = 0;
				ptr += 9;

				while grid[ptr] != 0 {
					ptr += 9;
				}

				ptr -= 9;

				while grid[ptr] != 0 {
					grid[ptr + 1] += grid[ptr + 7];
					grid[ptr + 7] = 0;
					ptr += 1;

					while grid[ptr] != 0 {
						grid[ptr] -= 1;
						grid[ptr + 6] += 1;
						ptr -= 1;

						while grid[ptr] != 0 {
							ptr -= 9;
						}

						grid[ptr + 7] = 1;
						ptr += 10;
					}

					ptr -= 10;
				}

			}

			grid[ptr] += grid[ptr + 7];
			grid[ptr + 7] = 0;

			while grid[ptr] != 0 {
				grid[ptr] -= 1;
				grid[ptr + 7] += 1;
				ptr += 9;

				while grid[ptr] != 0 {
					grid[ptr + 1] += 1;
					grid[ptr + 1] -= grid[ptr + 5];
					grid[ptr + 5] = 0;
					grid[ptr + 5] += grid[ptr + 1];
					grid[ptr + 1] = 0;
					ptr += 9;
				}

				grid[ptr - 2] += 1;
				ptr -= 9;

				while grid[ptr] != 0 {
					grid[ptr + 7] += grid[ptr + 5];
					grid[ptr + 5] = 0;
					ptr -= 9;
				}

				ptr += 9;

				while grid[ptr] != 0 {
					ptr += 9;
				}

				ptr -= 9;

				while grid[ptr] != 0 {
					grid[ptr + 1] = 0;
					grid[ptr] -= 1;
					ptr += 7;

					while grid[ptr] != 0 {
						grid[ptr] -= 1;
						grid[ptr - 7] += 1;
						grid[ptr - 7] -= grid[ptr - 6];
						grid[ptr - 9] += grid[ptr - 6];
						grid[ptr - 6] = 0;
						grid[ptr - 6] += grid[ptr - 7];
						grid[ptr - 7] = 0;
					}

					grid[ptr] += grid[ptr - 6];
					grid[ptr - 6] = 0;
					grid[ptr - 7] += 1;
					ptr -= 16;
				}

				grid[ptr + 7] -= 1;
				grid[ptr + 3] = 1;
			}

			grid[ptr] += 1;
			grid[ptr] -= grid[ptr + 7];
			grid[ptr + 7] = 0;
			grid[ptr + 7] += 1;

			while grid[ptr] != 0 {
				grid[ptr] -= 1;
				grid[ptr + 7] -= 1;
				ptr += 9;

				while grid[ptr] != 0 {
					grid[ptr + 7] += grid[ptr + 5];
					grid[ptr + 5] = 0;
					ptr += 9;
				}

				ptr -= 9;

				while grid[ptr] != 0 {
					grid[ptr + 1] = 0;
					grid[ptr] -= 1;
					ptr += 7;

					while grid[ptr] != 0 {
						grid[ptr] -= 1;
						grid[ptr - 7] += 1;
						grid[ptr - 7] -= grid[ptr - 6];
						grid[ptr - 9] += grid[ptr - 6];
						grid[ptr - 6] = 0;
						grid[ptr - 6] += grid[ptr - 7];
						grid[ptr - 7] = 0;
					}

					grid[ptr] += grid[ptr - 6];
					grid[ptr - 6] = 0;
					grid[ptr - 7] += 1;
					ptr -= 16;
				}

				ptr += 1;
				grid[ptr] += 5;

				while grid[ptr] != 0 {
					grid[ptr] -= 1;
					grid[ptr + 9] += grid[ptr];
					grid[ptr] = 0;
					ptr += 9;
				}

				grid[ptr + 4] += 1;
				ptr -= 1;

				while grid[ptr] != 0 {
					ptr -= 9;
				}

				ptr += 9;

				while grid[ptr] != 0 {
					grid[ptr] -= grid[ptr + 5];
					grid[ptr + 5] = 0;
					grid[ptr + 5] += 1;

					while grid[ptr] != 0 {
						grid[ptr] -= 1;
						grid[ptr + 5] -= 1;
						grid[ptr] += grid[ptr + 7];
						grid[ptr + 7] = 0;

						while grid[ptr] != 0 {
							grid[ptr] -= 1;
							grid[ptr + 7] += 1;
							ptr -= 9;

							while grid[ptr] != 0 {
								ptr -= 9;
							}

							grid[ptr + 4] = 1;
							ptr += 9;

							while grid[ptr] != 0 {
								ptr += 9;
							}

							grid[ptr + 1] += 1;
						}

					}

					grid[ptr] += 1;
					grid[ptr] -= grid[ptr + 7];
					grid[ptr + 7] = 0;
					grid[ptr + 7] += 1;

					while grid[ptr] != 0 {
						grid[ptr] -= 1;
						grid[ptr + 7] -= 1;
						grid[ptr] += grid[ptr + 5];
						grid[ptr + 5] = 0;

						while grid[ptr] != 0 {
							grid[ptr] -= 1;
							grid[ptr + 5] += 1;
							ptr -= 9;

							while grid[ptr] != 0 {
								ptr -= 9;
							}

							grid[ptr + 3] = 1;
							ptr += 9;

							while grid[ptr] != 0 {
								ptr += 9;
							}

							grid[ptr + 1] = 1;
						}

					}

					grid[ptr] += 1;
					ptr += 1;

					while grid[ptr] != 0 {
						grid[ptr] -= 1;
						ptr -= 1;

						while grid[ptr] != 0 {
							ptr += 9;
						}

						ptr -= 8;
					}

					ptr += 8;
				}

				ptr -= 9;

				while grid[ptr] != 0 {
					ptr -= 9;
				}

				grid[ptr + 4] = 0;
				grid[ptr + 1] += 5;
				ptr += 1;

				while grid[ptr] != 0 {
					grid[ptr] -= 1;
					grid[ptr + 9] += grid[ptr];
					grid[ptr] = 0;
					ptr += 9;
				}

				grid[ptr + 4] -= 1;
				ptr -= 1;

				while grid[ptr] != 0 {
					ptr -= 9;
				}

			}

			ptr += 3;
		}

		ptr -= 4;
		putchar(grid[ptr] as i32);
		ptr += 10;

		while grid[ptr] != 0 {
			grid[ptr + 6] = 0;
			ptr += 9;
		}

		ptr -= 9;

		while grid[ptr] != 0 {
			ptr -= 9;
		}

		ptr += 1;
		grid[ptr] += 10;

		while grid[ptr] != 0 {
			grid[ptr] -= 1;
			grid[ptr + 9] += grid[ptr];
			grid[ptr] = 0;
			ptr += 9;
		}

		grid[ptr + 5] += 1;
		grid[ptr + 14] += 1;
		ptr -= 1;

		while grid[ptr] != 0 {
			ptr -= 9;
		}

		grid[ptr] += grid[ptr + 8];
		grid[ptr + 8] = 0;

		while grid[ptr] != 0 {
			grid[ptr] -= 1;
			grid[ptr + 8] = 0;
			ptr += 9;

			while grid[ptr] != 0 {
				ptr += 9;
			}

			ptr -= 9;

			while grid[ptr] != 0 {
				grid[ptr + 1] += grid[ptr + 8];
				grid[ptr + 8] = 0;
				ptr += 1;

				while grid[ptr] != 0 {
					grid[ptr] -= 1;
					grid[ptr + 7] += 1;
					ptr -= 1;

					while grid[ptr] != 0 {
						ptr -= 9;
					}

					grid[ptr + 8] = 1;
					ptr += 10;
				}

				ptr -= 10;
			}

		}

		grid[ptr] += grid[ptr + 8];
		grid[ptr + 8] = 0;

		while grid[ptr] != 0 {
			grid[ptr] -= 1;
			grid[ptr + 8] += 1;
			ptr += 9;

			while grid[ptr] != 0 {
				grid[ptr + 1] += 1;
				grid[ptr + 1] -= grid[ptr + 6];
				grid[ptr + 6] = 0;
				grid[ptr + 6] += grid[ptr + 1];
				grid[ptr + 1] = 0;
				ptr += 9;
			}

			grid[ptr - 1] += 1;
			ptr -= 9;

			while grid[ptr] != 0 {
				grid[ptr + 8] += grid[ptr + 6];
				grid[ptr + 6] = 0;
				ptr -= 9;
			}

			ptr += 9;

			while grid[ptr] != 0 {
				ptr += 9;
			}

			ptr -= 9;

			while grid[ptr] != 0 {
				grid[ptr + 1] = 0;
				grid[ptr] -= 1;
				ptr += 8;

				while grid[ptr] != 0 {
					grid[ptr] -= 1;
					grid[ptr - 8] += 1;
					grid[ptr - 8] -= grid[ptr - 7];
					grid[ptr - 9] += grid[ptr - 7];
					grid[ptr - 7] = 0;
					grid[ptr - 7] += grid[ptr - 8];
					grid[ptr - 8] = 0;
				}

				grid[ptr] += grid[ptr - 7];
				grid[ptr - 7] = 0;
				grid[ptr - 8] += 1;
				ptr -= 17;
			}

			grid[ptr + 8] -= 1;
			grid[ptr + 3] = 1;
		}

		grid[ptr] += 1;
		grid[ptr] -= grid[ptr + 8];
		grid[ptr + 8] = 0;
		grid[ptr + 8] += 1;

		while grid[ptr] != 0 {
			grid[ptr] -= 1;
			grid[ptr + 8] -= 1;
			ptr += 9;

			while grid[ptr] != 0 {
				grid[ptr + 8] += grid[ptr + 6];
				grid[ptr + 6] = 0;
				ptr += 9;
			}

			ptr -= 9;

			while grid[ptr] != 0 {
				grid[ptr + 1] = 0;
				grid[ptr] -= 1;
				ptr += 8;

				while grid[ptr] != 0 {
					grid[ptr] -= 1;
					grid[ptr - 8] += 1;
					grid[ptr - 8] -= grid[ptr - 7];
					grid[ptr - 9] += grid[ptr - 7];
					grid[ptr - 7] = 0;
					grid[ptr - 7] += grid[ptr - 8];
					grid[ptr - 8] = 0;
				}

				grid[ptr] += grid[ptr - 7];
				grid[ptr - 7] = 0;
				grid[ptr - 8] += 1;
				ptr -= 17;
			}

			ptr += 1;
			grid[ptr] += 5;

			while grid[ptr] != 0 {
				grid[ptr] -= 1;
				grid[ptr + 9] += grid[ptr];
				grid[ptr] = 0;
				ptr += 9;
			}

			grid[ptr + 5] += 1;
			grid[ptr + 32] += 1;
			ptr += 26;

			while grid[ptr] != 0 {
				ptr -= 9;
			}

			ptr += 9;

			while grid[ptr] != 0 {
				grid[ptr] -= grid[ptr + 6];
				grid[ptr + 6] = 0;
				grid[ptr + 6] += 1;

				while grid[ptr] != 0 {
					grid[ptr] -= 1;
					grid[ptr + 6] -= 1;
					grid[ptr] += grid[ptr + 8];
					grid[ptr + 8] = 0;

					while grid[ptr] != 0 {
						grid[ptr] -= 1;
						grid[ptr + 8] += 1;
						ptr -= 9;

						while grid[ptr] != 0 {
							ptr -= 9;
						}

						grid[ptr + 4] = 1;
						ptr += 9;

						while grid[ptr] != 0 {
							ptr += 9;
						}

						grid[ptr + 1] += 1;
					}

				}

				grid[ptr] += 1;
				grid[ptr] -= grid[ptr + 8];
				grid[ptr + 8] = 0;
				grid[ptr + 8] += 1;

				while grid[ptr] != 0 {
					grid[ptr] -= 1;
					grid[ptr + 8] -= 1;
					grid[ptr] += grid[ptr + 6];
					grid[ptr + 6] = 0;

					while grid[ptr] != 0 {
						grid[ptr] -= 1;
						grid[ptr + 6] += 1;
						ptr -= 9;

						while grid[ptr] != 0 {
							ptr -= 9;
						}

						grid[ptr + 3] = 1;
						ptr += 9;

						while grid[ptr] != 0 {
							ptr += 9;
						}

						grid[ptr + 1] = 1;
					}

				}

				grid[ptr] += 1;
				ptr += 1;

				while grid[ptr] != 0 {
					grid[ptr] -= 1;
					ptr -= 1;

					while grid[ptr] != 0 {
						ptr += 9;
					}

					ptr -= 8;
				}

				ptr += 8;
			}

			ptr -= 9;

			while grid[ptr] != 0 {
				ptr -= 9;
			}

			grid[ptr + 4] = 0;
			grid[ptr + 1] += 5;
			ptr += 1;

			while grid[ptr] != 0 {
				grid[ptr] -= 1;
				grid[ptr + 9] += grid[ptr];
				grid[ptr] = 0;
				ptr += 9;
			}

			grid[ptr + 5] -= 1;
			grid[ptr + 32] -= 1;
			ptr += 26;

			while grid[ptr] != 0 {
				ptr -= 9;
			}

		}

		ptr += 3;
	}

}
