# euchre-rs
Implementation of Euchre in Rust.

## Speed of Simulation
Below is a table of the time it takes to run a number of games between 4 players who each make their decisions randomly.
Note: Make sure to compile the binary with the `--release` flag.


|    Games   | Time to play (sec) |
|  --------  |  ----------------  |
|  10,000    |        0.131       |
|  100,000   |        1.422       |
|  500,000   |        6.247       |
|  1,000,000 |        14.019      |


## As a first milestone:
- [X] I want this implementation of Euchre to be a CLI game that you can play against.

### Features:
- I want the logs of the game to be dumped to a file called output.txt, if not specified.

## In the future:
I want to create a CLI interface for running configurable games of Euchre.

### Features:

 - Simulating a game of euchre against 4 random players
 - Be able to seed players hands, either partially or fully
    - "I want player 1 to have all hearts. I want player 2 to have the king of diamonds, two tens, and the rest random cards, players 3 and 4 should have all random cards. I want to face up card to be the Jack of hearts. I want one of the buried cards to be the 9 of clubs."
- Be able to feed a list of moves to start the simulations from a certain point.
