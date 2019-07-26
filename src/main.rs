/*******************************************************************************
 *
 * Copyright (c) 2019 Gnarly Narwhal
 *
 * -----------------------------------------------------------------------------
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files(the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 *******************************************************************************/

use std::{thread, time};
use std::io::{self, stdin, Write};
use rand::Rng;

const MINIMAX_TITLE: &str = r"
******************************************
*     ____  _ _  _ _ ____  ___ __  __    *
*    |    \| | \| | |    \|   \\ \/ /    *
*    | | | | |    | | | | | |\ \>  <     *
*    |_|_|_|_|_|\_|_|_|_|_|_| \_\/\_\    *
*                                        *
******************************************";

enum Action {
	Example,
	Help,
	Begin,
	Set(String),
	Quit,
	Invalid(String),
}

struct Parameters {
	singleplayer: bool,
	depth: u32,
	parity: bool,
}

fn main() {
	println!("{}\n", MINIMAX_TITLE);

	println!("Welcome to Minimax, a simple game of perfect knowledge!

The rules are simple. A perfect binary tree will be randomly generated and each leaf will be assigned a random value.
The depth of the tree is completely up to you (but please don't make it too big because then I, the computer, have to
work extra hard and it won't fit on screen and it will just be a big ole messeroni). Once generated it is time to
begin the game. Starting at the root, players take turns choosing a direction to traverse the tree. Once the bottom
of the tree is reached, the value of that leaf is the result of the game. One player is working to minimize that value.
The other player is working to maximize that value. I think this is best demonstrated through an example, but maybe
you don't need it. You seem of the intelligent type. In fact you've probably already worked out the optimal strategy.
But on the off chance that you would like an example why don't I do this. I'll just kinda casually mention in passing
that you can enter \"example\" to have me walk you through an example game. Remember there is no shame in asking for help.

Oh yeah! One last thing. If you aren't sure what to do help is available 24/7. All you need to do
is enter the \"help\" command and I'll be there to assist you. I say assist you like I'm doing you
a favor. Really I just explain the limited ways in which you can interact with me given my simplistic
intelligence. I won't always understand you, but I try my best.

So! Without further ado let's get this show on the road!
");

	let mut params = Parameters {
		singleplayer: true,
		depth: 4,
		parity: true,
	};

	loop {
		print!("Please enter a command: ");
		if let Err(err) = io::stdout().flush() {
			panic!("{}", err);
		}

		let action = get_action();
		match action {
			Action::Example      => walkthrough_example(),
			Action::Help         => print_help(),
			Action::Begin        => run_game(&params),
			Action::Set(string)  => match get_values(&string) {
				Ok(("singleplayer", value)) => {
					match parse_bool(&value) {
						Ok(boolean) => { params.singleplayer = boolean; },
						Err(err)    => println!("{}", err),
					};
				},
				Ok(("depth", value)) => {
					params.depth = match value.parse::<u32>() {
						Ok(num) => {
							if num > 0 {
								num
							}
							else {
								println!("Zero is pretty great! Alas it does not work as a depth value in this case!");
								continue;
							}
						},
						Err(_) => {
							println!("That's quite the depth you got there! I'm not sure how to handle it!");
							continue;
						},
					};
				},
				Ok(("min", value)) => {
					params.parity = match value {
						"player_one" => false,
						"player_two" => true,
						_            => {
							println!("Hmm. I am not familiar with player \"value\"!");
							continue;
						}
					};
				},
				Ok(("max", value)) => {
					params.parity = match value {
						"player_one" => true,
						"player_two" => false,
						_            => {
							println!("Hmm. I am not familiar with player \"value\"!");
							continue;
						}
					};
				},
				Ok((key, _)) => println!("\"{}\" is not a key I am familiar with!", key),
				Err(_)     => println!("The syntax in your set command \"{}\" appears to be off!", string),
			},
			Action::Quit         => break,
			Action::Invalid(err) => println!("Beep! Bop! Boop! Cannot execute command \"{}\"! Haha!", err),
		}
	}
}

fn get_action() -> Action {
	let mut input = String::new();
	stdin()
		.read_line(&mut input)
		.expect("Oh dear! It appears that input was invalid!");

	let input = input.trim();

	match input {
		"example" => Action::Example,
		"help"    => Action::Help,
		"begin"   => Action::Begin,
		"quit"    => Action::Quit,
		&_ if starts_with(&input, "set ") => Action::Set(input["set ".len()..].to_string()),
		&_        => Action::Invalid(input.to_string()),
	}
}

fn starts_with(string: &str, begin: &str) -> bool {
	if string.len() < begin.len() { return false; }
	for chars in begin.chars().zip(string.chars()) {
		let (a, b) = chars;
		if a != b { return false; }
	}
	true
}

fn walkthrough_example() {
	println!("Alright an example it is. For starters lets \"generate\" a binary tree (the tree for the example is pre-generated, but pretend
this is a real game and I'm generating it dynamically). We'll use a small depth 4 tree for our example.

0:         +-------B-------+
           |               |
1:     +---B---+       +---B---+
       |       |       |       |
2:   +-B-+   +-B-+   +-B-+   +-B-+
     |   |   |   |   |   |   |   |
3:   4   7   1   3   7   1   2   9

This is our binary tree. It's got the letter B representing the branch nodes and all the leaf nodes at the bottom have a numerical value.
You can also see it's got 4 rows (depth = 4) labeled 0-3 on the lefthand side. I do so enjoy some nice 0 indexing :)");
	pause();

	println!("Anyway moving on. Lets introduce some players. Let's say player 1 is Argyle and player 2 is Sol. In this particular game let's also
say that Argyle is trying to maximize the final number and Sol is trying to minimize it. We begin our journey at the top of the tree
in layer 0 where the caret is pointing to the B.

0:>        +-------B-------+
           |       ^       |
1:     +---B---+       +---B---+
       |       |       |       |
2:   +-B-+   +-B-+   +-B-+   +-B-+
     |   |   |   |   |   |   |   |
3:   4   7   1   3   7   1   2   9");
	pause();

	println!("Argyle is now faced with a decision. Do they want to move left down the tree or right down the tree? For whatever reason Argyle decides
to move left down the tree. After that our game would look like this. The entire right side is now inaccesible and we are now looking at layer 1.

0:         +-<-<-<-B       x
           V
1:>    +---B---+       x   B   x
       |   ^   |
2:   +-B-+   +-B-+   x B x   x B x
     |   |   |   |
3:   4   7   1   3   x   x   x   x");
	pause();

	println!("Now it is Sol's turn. Sol is faced with the same decision as Argyle. Sol decides that they have a good feeling about left as well.
So Sol moves left. Now, Argyle seeing that it's between a 4 and a 7 decides that the best option is to go right. And with this move the
game is complete. The bottom of the tree has been reached and the result of the game is 7. How you decide who is the winner is beyond me,
but you are free to decide that for yourself if you so choose. After all moves have been taken this is what the game board would look like.

0:         +-<-<-<-B       x
           V
1:>    +-<-B   x       x   B   x
       V
2:   x B>+   x B x   x B x   x B x
         V
3:   4  [7]  1   3   x   x   x   x
         ^
Final value: 7


And thus concludes our little example game. Hopefully it has demonstrated clearly how one plays the game of Minimax.
I feel I should note that our friends Argyle and Sol were not playing optimally. I didn't want to spoil the strategy
for anybody ;) Anyway enough talk. Lets get on with the game.

");
}

fn pause() {
	println!("\n\nPress enter to continue. . .");
	stdin().read_line(&mut String::new()).unwrap();
	println!();
}

fn print_help() {
	println!("Currently I have a number of commands at my disposal. I'll go ahead and list them for you real quick.

example > Walks you through an example game if you still feel a little unclear on the rules of the game
help    > Well I think you already know about this command :)
begin   > Begins the epic journey through the binary tree
set {{key}} = {{value}} > Example: \"set depth = 6\" | Sets some paramaters for the game. Currently the available paramaters are:
	singleplayer - A boolean value which which sets whether you play against me or play against your mates
	depth        - An integer value which sets the depth of the tree
	min          - One of either \"player_one\" OR \"player_two\". Sets that player's goal to minimize the value
	max          - One of either \"player_one\" OR \"player_two\". Sets that player's goal to maximize the value
quit > I should think you would know what this one does as well");
}

fn get_values(string: &str) -> Result<(&str, &str), ()> {
	let mut split = 0;
	for (i, byte) in string.bytes().enumerate() {
		if byte == b'=' {
			split = i;
			break;
		}
	}
	if split == string.len() - 1 {
		return Err(());
	}
	let key   = string[..split].trim();
	let value = string[(split + 1)..].trim();
	if key.len() == 0 || value.len() == 0 {
		Err(())
	}
	else {
		Ok((key, value))
	}
}

fn parse_bool(value: &str) -> Result<bool, &'static str> {
	match value {
		"true"  => Ok(true),
		"false" => Ok(false),
		_       => Err("I do apologize, but I was expecting a boolean value!"),
	}
}

enum Direction {
	Left,
	Right,
}

fn run_game(params: &Parameters) {
	let mut active_layer:  u32 = 0;
	let mut active_branch: u32 = 0;
	let mut active_player      = false;
	let tree = generate_tree(params.depth, params.parity);

	println!("Let the game begin!

Player 1 you are trying to {player_one_goal} the value. Player 2 you are trying to {player_two_goal} the value.",
	player_one_goal = if params.parity { "maximize" } else { "minimize" },
	player_two_goal = if params.parity { "minimize" } else { "maximize" });

	loop {
		print_tree(&tree, active_layer, active_branch);

		if active_layer as usize == tree.len() - 1 {
			println!("And that's the game! Your final result is: {}\n", tree[active_layer as usize][active_branch as usize]);
			break;
		}
		else {
			print!("Player {} you're up! Which direction would you like to go, left or right?\nDirection: ", 
				if active_player { 2 }
				else             { 1 }
			);
			if let Err(err) = io::stdout().flush() {
				panic!("{}", err);
			}

			let dir =
				if !params.singleplayer || !active_player {
					user_input()
				}
				else {
					computer_input(active_layer, active_branch, &tree, params.parity)
				};

			match dir {
				Direction::Left => {
					active_branch = active_branch * 2;
				},
				Direction::Right => {
					active_branch = active_branch * 2 + 1;
				},
			}
			active_layer += 1;
			active_player = !active_player;
		}
	}
}

fn generate_tree(depth: u32, parity: bool) -> Vec<Vec<i32>> {
	const TREE_RANGE_MIN: i32 = 100;
	const TREE_RANGE_MAX: i32 = 1000;
	
	let mut tree: Vec<Vec<i32>> = Vec::new();
	
	let mut leaves: Vec<i32> = Vec::new();
	let mut leaf_count = pow(2, depth - 1);
	while leaf_count > 0 {
		leaves.push(rand::thread_rng().gen_range(TREE_RANGE_MIN, TREE_RANGE_MAX));
		leaf_count -= 1;
	}
	tree.push(leaves);
	
	expand_tree(&mut tree, depth % 2 == parity as u32);

	tree
}

fn pow(mut base: u32, mut power: u32) -> u32 {
	let mut result = 1;
	while power > 0 {
		if power & 1 != 0 { result = result * base; }
		base *= base;
		power >>= 1;
	}
	result
}

fn expand_tree(tree: &mut Vec<Vec<i32>>, parity: bool) {
	if tree[0].len() == 1 { return; }
	let mut write_layer: Vec<i32> = Vec::new();
	for i in 0..tree[0].len() / 2 {
		write_layer.push(
			if (tree[0][i * 2] < tree[0][i * 2 + 1]) == parity { tree[0][i * 2] }
			else { tree[0][i * 2 + 1] }
		);
	}
	tree.insert(0, write_layer);
	expand_tree(tree, !parity);
}

fn user_input() -> Direction {
	let mut input = String::new();
	stdin()
		.read_line(&mut input)
		.expect("Oh dear! It appears that input was invalid!");
	match input.trim().as_ref() {
		"left"  => Direction::Left,
		"right" => Direction::Right,
		_       => {
			print!("Please choose either left or right.\nDirection: ");
			if let Err(err) = io::stdout().flush() {
				panic!("{}", err);
			};
			user_input()
		},
	}
}

fn computer_input(active_layer: u32, active_branch: u32, tree: &Vec<Vec<i32>>, parity: bool) -> Direction {
	const BASE_DURATION: u32 = 150;
	const VARIATION:     u32 = 50;
	let left_value  = tree[(active_layer + 1) as usize][(active_branch * 2    ) as usize];
	let right_value = tree[(active_layer + 1) as usize][(active_branch * 2 + 1) as usize];
	let direction =
		if left_value < right_value && parity {
			Direction::Left
		}
		else {
			Direction::Right
		};

	let sleep_duration = time::Duration::from_millis(500);
	thread::sleep(sleep_duration);

	for c in match direction {
			Direction::Left  => "left",
			Direction::Right => "right",
		}.chars() {
		print!("{}", c);
		if let Err(err) = io::stdout().flush() {
			panic!("{}", err);
		};
		let sleep_duration = time::Duration::from_millis((BASE_DURATION + rand::thread_rng().gen_range(0, VARIATION)) as u64);
		thread::sleep(sleep_duration);
	}

	println!("");
	direction
}

fn print_tree(tree: &Vec<Vec<i32>>, active_layer: u32, active_branch: u32) {
	println!("");
	let mut layer_number = 0;
	if tree.len() > 1 {
		let mut schematics: Vec<(u32, u32)> = vec![(1, 5)];
		for _i in 0..(tree.len() - 2) {
			let (offset, length) = schematics[0];
			schematics.insert(0, (offset + length / 2 + 1, length * 2 + 1));
		}
		let mut branch_count = 1;
		for (i, scheme) in schematics.iter().enumerate() {
			let (offset, length) = scheme;
			let mut output = String::new();
			let mut vertical = String::new();
			for _j in 0..*offset {
				output.push(' ');
				vertical.push(' ');
			}
			for j in 0..branch_count {
				output.push('+');
				vertical.push('|');
				for k in 0..*length {
					if k == length / 2 {
						output.push('B');
						if i as u32 == active_layer && j == active_branch {
							vertical.push('^');
						}
						else {
							vertical.push(' ');
						}
					}
					else {
						output.push('-');
						vertical.push(' ');
					}
				}
				output.push('+');
				vertical.push('|');
				if j < branch_count - 1 {
					for _k in 0..*length {
						output.push(' ');
						vertical.push(' ');
					}
				}
			}
			println!("{}:{}  {}\n     {}", i, if layer_number == active_layer { ">" } else { " " }, output, vertical);
			branch_count *= 2;
			layer_number += 1;
		}
	}
	print!("{}:{}  ", layer_number, if layer_number == active_layer { ">" } else { " " });
	for leaf in &tree[layer_number as usize] {
		print!("{}   ", leaf);
	}
	if let Err(err) = io::stdout().flush() {
		panic!("{}", err);
	}
	println!("\n");
}