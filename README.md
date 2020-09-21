# Sudoku Solver

This is a simple brute force sudoku solver.

This is mainly a learning exercise, to try and improve my Rust skills.

An example puzzle is included in this repo, but for extra tests I wrote a scraper which is also stored in this repository. To use it:

```bash
 $ cd scaper 
 $ npm install
 $ node main.js
```

This will download three files: `easy_sudoku`, `medium_sudoku`, and `hard_sudoku` to the `scraper` directory.

Then run against the solver with:

```bash
 $ cat scraper/<sudoku_file> | cargo run
```

This takes a sudoku as input from stdin in the form of 81 consecutive digits with no spaces. 0 represents an unknown value in the grid.

## Future Plans

 - Upgrade Scraper to run in non-headless mode and fill out the sudoku.

