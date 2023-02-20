import "./App.css";
import { useState, useEffect } from "react";
import init, { fractal } from "wasm-lib";

function App() {

	init().then(() => {

		const dictionaries = [
			{"id": 1, "x": 0.0, "y": 0.0},
			{"id": 2, "x": 1.0, "y": 0.0},
			{"id": 3, "x": 2.0, "y": 3.0},
			{"id": 4, "x": 3.0, "y": 0.0},
			{"id": 5, "x": 4.0, "y": 0.0}
		];
		const result = fractal(dictionaries, 1);
		console.log(result);

	});

	return (
		<div className="App">
			<h1>Hello fractal world!</h1>
		</div>
	);
}

export default App;
