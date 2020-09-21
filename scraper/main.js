const fs = require("fs");
const puppeteer = require("puppeteer");

(async () => {
	const browser = await puppeteer.launch();
	const page = await browser.newPage();
	await page.goto("https://www.nytimes.com/puzzles/sudoku");

	const sudokus = await page.evaluate(
		`const difficulties = ['easy', 'medium', 'hard'];
		difficulties.map(difficulty => window.gameData[difficulty].puzzle_data.puzzle.join(""))`
	);
	let [easy, medium, hard] = sudokus;
	fs.writeFileSync("easy_sudoku", easy);
	fs.writeFileSync("medium_sudoku", medium);
	fs.writeFileSync("hard_sudoku", hard);
	await browser.close();
})();
