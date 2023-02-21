import "./App.css";
import { useState, useEffect } from "react";
import Line from "./components/Line";
import { IterInput, PointInput } from "./components/Inputs";
import init, { fractal } from "wasm-lib";

function App() {
	const [inputPoints, setInputPoints] = useState([
		{ id: 0, x: 0, y: 0 },
		{ id: 1, x: 1, y: 0 },
		{ id: 2, x: 2, y: 1 },
		{ id: 3, x: 3, y: 0 },
		{ id: 4, x: 4, y: 0 },
	]);
	const [iterations, setIterations] = useState(3);
	const [points, setPoints] = useState([]);

	useEffect(() => {
		init().then(() => {
			let nextPoints = fractal(inputPoints, iterations);
			setPoints(nextPoints);
			console.log(nextPoints);
		});
	}, [inputPoints, iterations]);

	return (
		<div className="App">
			<h1>Hello fractal world!</h1>
			<Line data={points} svgWidth={500} svgHeight={500} />

			<div className="controls">
				<IterInput iterations={iterations} setIterations={setIterations} />
				{inputPoints.map((point) => {
					return (
						<PointInput
							key={point.id}
							point={point}
							inputPoints={inputPoints}
							setInputPoints={setInputPoints}
						/>
					);
				})}
				<button
					onClick={() => {
						const nextInputPoints = [
							...inputPoints,
							{ id: inputPoints.length, x: 0, y: 0 },
						];
						setInputPoints(nextInputPoints);
					}}
				>
					Add point
				</button>
			</div>
		</div>
	);
}

export default App;
