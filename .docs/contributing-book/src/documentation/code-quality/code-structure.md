# Code Structure

In order to structure your code there is only 1 rule that you must remember and that is that everything must always be ordered alphabetically - as long as there isn't a dependency error or a better name for a default field of an enum. The reason is that it provides a structure that almost everyone will be familiar with. This means that nobody has to learn how you have decided to structure your code and thus where to look for a variable, function, import etc. This makes it really easy to navigate your entire project and in turn navigate all projects without having to memorize the intricacies of each project.

To be explicit

- All dependencies must be declared in alphabetical order just below the declaration of the program type e.g. below `contract;`
- All imports must be in alphabetical order and any modules within those imports must be in alphabetical order
- All functions, structs, enums, fields inside structs and enums, parameters of a function, parameters in documentation etc. must be in alphabetical order in all files (this does not apply to variables inside a function)

Example

```rust
contract;

dep data_structures;
dep interface;
dep utils;

use data_structures::{Game, Player, Winner};
use interface::{ConnectFour, DrawEvent, MoveEvent, WinnerEvent};
use std::identity::Identity;
use utils::validate_move;

storage {
   /// The total number of created games
   games_played: u64 = 0,

   /// The number of times player 2 has won against player 1
   player_two_wins: u64 = 0,

   /// The number of times player 1 has won against player 2
   player_one_wins: u64 = 0,
   
   // ...
}

impl ConnectFour for Contract {

   fn create_game(player_two: Player, player_one: Player) -> Game {
      // Perform a check on each player address to see if they are blacklisted

      // owl
   }

   fn move(column: u64, game: Game) -> Game {
      // Perform a check to see if the game has ended
      // Perform a check to see if the position is valid
      
      // owl
   }

   // rest of owl

}
```

The only other aspect to remember is to use the formatter before commiting your work.

- `cargo fmt` to format your `Rust` files (SDK tests)
- `forc fmt` to format your `Sway` files
