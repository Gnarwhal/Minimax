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

use std::io::{self, stdin, Write};
use rand::Rng;

const MINIMAX_TITLE: &str = r"
     ____  _ _  _ _ ____  ___ __  __
    |    \| | \| | |    \|   \\ \/ /
    | | | | |    | | | | | |\ \>  <
    |_|_|_|_|_|\_|_|_|_|_|_| \_\/\_\";

enum Action {
	Example,
	Help,
	Begin,
	Set(String),
	Quit,
	Invalid(String),
}

fn main() {
	println!("{}\n\n", MINIMAX_TITLE);

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

	let mut singleplayer = true;
	let mut depth: u32   = 4;

	loop {
		print!("Please enter a command: ");
		if let Err(err) = io::stdout().flush() {
			panic!("{}", err);
		}

		let action = get_action();
		match action {
			Action::Example      => walkthrough_example(),
			Action::Help         => print_help(),
			Action::Begin        => println!("You would like to begin"),
			Action::Set(string)  => match get_values(&string) {
				Ok(("singleplayer", value)) => {
					match parse_bool(&value) {
						Ok(boolean) => { singleplayer = boolean; },
						Err(err)    => println!("{}", err),
					};
				},
				Ok(("depth", value)) => {
					depth = match value.parse::<u32>() {
						Ok(num) => {
							if num > 0 {
								num
							}
							else {
								println!("Zero is pretty great! Alas it does not make sense as a depth value!");
								continue;
							}
						},
						Err(_) => {
							println!("A depth of \"value\" does not make sense to me! I was expecting a positive number!");
							continue;
						},
					};
				},
				Ok((key, _)) => println!("\"{}\" is not a key I recognize! Perhaps try something else!", key),
				Err(_)     => println!("Unfortunately I'm not sure what you are trying to set with the command \"{}\"!", string),
			},
			Action::Quit         => break,
			Action::Invalid(err) => println!("Beep! Bop! Boop! Cannot compute \"{}\"! Haha!", err),
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

example > Runs you through an example game if still feel a little unclear on the rules of the game
help    > Well I think you already know about this command :)
begin   > Begins the epic journey through the binary tree
set {{key}} = {{value}} > Sets some paramaters for the game. Currently the available paramaters are:
	singleplayer - a boolean value which switches between playing against me and playing against your mates
	depth        - an integer value which sets the depth of the tree
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
		return Err(())
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
		_       => Err("I do apologize, but I was expecting a boolean type!"),
	}
}