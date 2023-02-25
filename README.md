#### 2048-in-rust
======:q
==================
Implemented:
------------
Game Struct
Board struct
Square struct

-------------
Missing:
-------------
Implement the rule system for the game.
Movement!!
Lost state
Key::events
Interface

-------------
##Movement!!
-------------
[11][12][13][14]
[21][22][23][24]
[31][32][33][34]
[41][42][43][44]

[11][12][13][14]


First thing is to move all the squares 
then add the number

Conditions:
Always check conditions after a move

This one after adding a square:
if(sum_possible){
do sum
}

if (!sum_possible && board_full){
game_over
}




* Check if the board is not full 
* Add Square
* check if a sum is possible 
* If is not then is GAME OVER

Where should i put this??
the movement in the board strut obv
but the sums?
maybe the sums in the square struct since already sets the values for the sqares
for now i just input the self parameter for this fuctions


