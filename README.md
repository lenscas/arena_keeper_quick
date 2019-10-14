[![dependency status](https://deps.rs/repo/github/lenscas/arena_keeper_quick/status.svg)](https://deps.rs/repo/github/lenscas/arena_keeper_quick)
# arena_keeper_quick
A continuation of [arena_keeper](https://github.com/lenscas/arena_keeper) but using quicksilver instead of Yew.

Arena keeper is a game where you will take care of people from various species and let them fight for money.
This will allow you to get more people and buy more stuff for them.

An online build is available [here](https://lenscas.github.io/arena_keeper_quick/) though this may not always be up to date. Alternatively, you can build the game yourself using the steps below.

## Setup
To set the project up all you need is [rustup and cargo](https://www.rust-lang.org/tools/install), along with any dependencies from [quicksilver](https://github.com/ryanisaacg/quicksilver/).

In order to make WASM builds [cargo-web](https://github.com/koute/cargo-web) is highly recomended.

## Run

After you ran the above commands, simply run `cargo run` to run the project in debug mode. If you installed cargo-web You can also use `cargo web start` to start a http server that will host a wasm build of the project. This allows you play the game in your browser.

You can also use `cargo run --release` and `cargo web start --release` to build the game in release mode.

## Develop mode

The game also features a "Develop" mode. You can compile it into develop mode using `cargo run --bin develop`.

develop builds allow you to enter dev mode by pressing `ctl + p`. This will pause the game and give you extra ways to interact with the window.

By clicking on the screen its possible to draw a square and get its exact coordinates. This is usefull when creating parts of a GUI as it allows you to quickly draw where the elements should go and get the coordinates that corresponds to the given screen location.

Assuming you have access to the stdin of the program (Ran it from the terminal for example) you are also able to give it extra commands. Right now only `read` is supported.

`read` takes an index and prints out the position and size of the drawn square corresponding to index.

### Note

Develop mode makes use of a seperate thread and thus does ***not*** work when compiling to WASM.

## Modules
Most content will be added through dynamically loaded json files, which are packed inside `.zip` files that are placed inside the `static` folder.

There is also a json file called `mods.json`. This one tells the game which zip files to load. The zip files can contain a `tiles` folder and/or a `species` folder. The tiles folder is used to describe tiles and the species folder is used to describe species.

There are no further requirments in the layout of the .zip contents. There are however requirements on the layout of the json files. The layout for both of them can be viewed in [structs.rs](src/modules/structs.rs) as `SpeciesConf` for species and `Tile` for tiles.

## Credit
Sprites made by: [0x72](https://0x72.itch.io/) and [disfey](https://disfey.itch.io/)