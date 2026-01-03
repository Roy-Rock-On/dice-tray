# Welcome to Dice Tray

Most dice rollers don't allow dice they roll to persist after they deliver their results. I want to fix that. 
The CLI creates a dice tray that stores persistant dice structs, each with their own internal RNG seed.

Version 0.1.0 has been rewritten to use the popular clap crate to streamline thje cli. Dice are now saved and reloaded each time they are used. 
Commands should be explained by running "dicetray --help"

I'd like to expand this with more dice types and criteria for modifying dice automatically. It's been a lot of fun, and I'm excited to keep learning more about rust. 
 