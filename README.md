# arena_keeper_quick
A continuation of [arena_keeper](https://github.com/lenscas/arena_keeper) but using quicksilver instead of Yew.

Arena keeper is a game where you will take care of people from various species and let them fight for money.
This will allow you to get more people and buy more stuff for them.

## Install
1. Install [rustup and cargo](https://www.rust-lang.org/tools/install)
2. install [cargo-web](https://github.com/koute/cargo-web)
3. install yarn and nodejs
4. add some pictures (.png) to /species/human/images and /species/merfolk/images
5. run `yarn` followed by `yarn gen all`
## Run
After you ran the above commands, simply run `cargo run` to run the project in debug mode. You can also use `cargo web start` 
to start a http server that will host a wasm build of the project. This allows you play the game in your browser.

You can also use `cargo run --release` and `cargo web start --release` to build the game in release mode.

## Generating code
This project contain some commands to automatically generate code.

### Species
Command : `yarn gen species`

This generates various functions and enums based upon the files and folders inside /species. It also copies the images to the static folder so they can be accesed. 

### Assets
Command : `yarn gen assets`

This generates a function that will load every asset that exists in the static folder, along with a structure to hold them.
This allows the game to only start playing after every asset is loaded as loading is asynchronous.

As of now, it only bundles pictures (.png) and fonts (.ttf) but more are planned in the future once I need them.

### All
Command : `yarn gen all`

This simply runs all other gen commands in the correct order for you. 
