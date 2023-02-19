import "./App.css";
import { useState, useEffect } from "react";
import init, { process_dictionaries } from "wasm-lib";

function App() {

	init().then(() => {

		const dictionary = [
			{"id": 1, "x": 4.0, "y": 2.0},
			{"id": 2, "x": 4.0, "y": 3.0}
		];
		const result = process_dictionaries(dictionary);
		console.log(result);

	});

	return (
		<div className="App">
			<h1>Hello fractal world!</h1>
		</div>
	);
}

export default App;
