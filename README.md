# rust-tetris
playing around with SDL2 graphics in rust lang, prototype a tetris game in rust with no libs except SDL opengl graphics


## gameplay
This is what the game looks like 

screenshot_1.png           |  screenshot_2.png
:-------------------------:|:-------------------------:
![Game Screenshot 1](/screenshot_1.png?raw=true "Game Screenshot 1") |  ![Game Screenshot 2](/screenshot_2.png?raw=true "Game Screenshot 2")


## controls
 Key | Action
 :-------------------------:|:-------------------------:
 Left or Right Arrow | Move Horizontal
 Down Arrow | Move Piece Down
 Upper Arrow | Rotate Piece
 Space | Drop piece
 
 
## installation
download the repo and use 
`cargo build --release` in root directory of the repo to build the executable

this game uses SDL2 for graphics so you might have to install SDL2 for your respective OS.
