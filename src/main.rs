
extern crate solution;
use solution::*;
use std::io;

fn main() {
	let mut board : Board = Board::new();
	board.initialize();
	println!("Add who is first(Me or Comp): ");
	let mut input = String::new();
	io::stdin().read_line(&mut input).expect("string");
	let who_is_first=input.trim();
 
	// Our move
	if who_is_first == "Me" {
		let mut is_valid : bool = false;
		while !is_valid {
			println!("Add new coordinates for your turn, split them with , : ");
			input.clear();
			io::stdin().read_line(&mut input).expect("string");
			let coord=input.trim();
			let coords = coord.split(',')
				.map(|x| x.parse::<usize>().unwrap())
				.collect::<Vec<usize>>();

			println!("{:?}", coords);
			is_valid = board.change( coords[0], coords[1] );
			if !is_valid {
				println!("Invalid move");
			}
		}
		board.print_matrix();
	}
	
	// Main loop
	while !(board.value() == 1 || board.value() == -1) {
		let diff = solution::Board::max(board);
		board = diff.0;
		board.print_matrix();
		if board.value() == 1 {
			println!("Computer wins!");
			break;
		}
		else if board.is_game_finished() {
			println!("No one wins!");
			break;
		}

		let mut is_valid = false;

		while !is_valid {
			println!("Add new coordinates for yoour turn, split them with , : ");
			input.clear();
			io::stdin().read_line(&mut input).expect("string");
			let coord=input.trim();
			let coords = coord.split(',')
				.map(|x| x.parse::<usize>().unwrap())
				.collect::<Vec<usize>>();
			is_valid = board.change( coords[0], coords[1] );
			if !is_valid {
				println!("Invalid move");
			}
		};

		board.print_matrix();

		if board.value() == -1 {
			println!("You win!");
			break;
		}
		else if board.is_game_finished() {
			println!("No one wins!");
			break;
		}
	}
}
