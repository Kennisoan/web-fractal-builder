import "./App.css";
import { useState, useEffect } from "react";
import init, { add } from "wasm-lib";

function App() {
	const [number, setNumber] = useState(0);
	useEffect(() => {
		init().then(() => {
			setNumber(add(2, 2));
		});
	}, []);

	return (
		<div className="App">
			<h1>Hello fractal world!</h1>
			<p>The number is {number}</p>
		</div>
	);
}

export default App;
