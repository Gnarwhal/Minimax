# Minimax
A game of perfect knowledge

Perfect knowledge means all information is available to all the players. For example in chess, both
players can see all pieces and where they are. In a game like poker, there is hidden information. You
can't see what's in the deck and you can't see what is in other players hands. Minimax is nice because
it has perfect knowledge without being too computationally expensive, so it works well as a demonstration.

## How to play
Game play is very simple in Minimax. To start you need to draw yourself a perfect binary tree.
Then assign each leaf in the tree a random number (or "random" if you wann rig the game). Starting
from the top of the tree, 2 players now take turns navigating down the tree. When you reach the bottom of
the tree, the value at that leaf is the result of the game. The goals for each player are polar opposite.
One player wishes to maximize the final value while the other wishes to minimize it. I'm not sure there is
a winner or a loser in this game, but if you wanna develop your own method for determining a winner and
and a loser then be my guest.

## How the program works
The program is just a command line version of minimax. Once you run the program you'll be prompted to enter
a command. The command list is as follows:
#### example
Runs you through an example game if still feel a little unclear on the rules of the game
#### help
Brings up a help menu with a list of commands
#### begin
Begins the game
#### set { key } = { value }
Example: `set depth = 6`  
Sets some paramaters for the game. Currently the available paramaters are:
- singleplayer - A boolean value which which sets whether you play against the computer or another person
- depth        - An integer value which sets the depth of the tree
- min          - One of either \"player_one\" OR \"player_two\". Sets that player's goal to minimize the value
- max          - One of either \"player_one\" OR \"player_two\". Sets that player's goal to maximize the value
#### quit
Exits the program

### Playing the game
Once you enter the begin command, a tree will be generated and the game will begin. Players take turns
entering either `left` or `right` and when the end is reached the value will be announced. You can see
where the you currently are in the tree by looking for the caret `^` underneath the branch point. You
can also look for the right angle bracket `>` next to the line numbers on the left to see which row
you are currently in.
