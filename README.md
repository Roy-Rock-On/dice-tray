# Welcome to Dice Tray

Most dice rollers don't allow dice they roll to persist after they deliver thier results. I want to fix that. 
The CLI creates a dice tray that stores persistant dice structs, each with their own internal RNG seed.

# Features
- **Table Display** => Your dice tray displays in a table that shows all your dice and relevent information about them. Thank to the fantastic crate [cli_table](https://docs.rs/cli-table/latest/cli_table/index.html).

- **Dice Identity** => using the $ in a command will allow you to refer to dice by identities. Allowing you to apply behaviour to all dice that share an identity. Syntax for dice identities sould look like this "$DiceID". Commands are case insensitive, but dice identities are case sensitive and can only contain alphanumeric characters. Dice IDs can also be chained using commas (e.g "$DiceName1,DiceName2,DiceName3").

- **Dice Index** => using the @ in a command will allow you to refer to dice by a specific index. Allowing you to apply behaviour seclectively in your tray. For example "@2" would target the dice at index of 2 in the tray. Like identiy, index can be chained using commas (e.g "@1,3,5" would target dice at indexs one three and five.) The dice tray indexes from 0.

# Commands
- **-a | -add** => adds dice to the tray based on the following dice expressions. (e.g "-a d6 2d4" would add a six-sided dice and two four-sided dice to the tray.) Dice can be given identities by using a $ followed by the dice name (e.g "-a $FIREBALL 8d6" would add eight six-sided dice to the tray with the identity FIREBALL.) The add flag is the default behaviour if no command flags are provided (e.g. typing "2d10" alone would add 2 ten sided dice to the tray.) 

- **-r | -roll** => Rolls dice at the provided target. If no targets are provided, rolls all dice in the tray.

- **-rb | -rerollbest** => Re-rolls the dice at the provided target and updates the result if it is higher than the current result. If no targets are provided, re-rolls all dice in the tray.

- **-rw | -rerollworst** => Re-rolls the dice at the provided target and updates the result if it is lower than the current result. If no targets are provided, re-rolls all dice in the tray.

- **-e | -explode** => Re-rolls the dice at the provided target and adds the new result to the previous result. If no targets are provided, explodes all dice in the tray.

- **-d | -drop** => Drops the dice at the provided target, removing them from the tray. If no targets are provides, clears the tray of all dice.

- **-h | -help** => Displays a help message in the CLI. It will remind you of the commands, but it is a poor replacement for this fantastic readme.

- **-e | -exit** => Ends the program and returns you to your terminal. 

The other crates featured in this project are [rand](https://docs.rs/rand/latest/rand/) and [regex](https://docs.rs/regex/latest/regex/).

This is my first Rust project, and I'm still learning. Feedback from more experienced rustaceans is appreciated. More to come soon. 
