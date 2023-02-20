import "./App.css";
import { useState, useEffect } from "react";
import Line from "./components/Line";
import init, { fractal } from "wasm-lib";

function App() {
	const [inputPoints, setInputPoints] = useState([
		{ id: 0, x: 0, y: 0 },
		{ id: 1, x: 1, y: 2 },
		{ id: 2, x: 3, y: 4 },
	]);
	const [points, setPoints] = useState([]);

	useEffect(() => {
		init().then(() => {
			let nextPoints = fractal(inputPoints);
			setPoints(nextPoints);
			console.log(nextPoints);
		});
	}, [inputPoints]);

	return (
		<div className="App">
			<h1>Hello fractal world!</h1>
			<Line data={points} svgWidth={500} svgHeight={500} />
			{inputPoints.map((point) => {
				return (
					<div className="point-input-container" key={point.id}>
						<b>{point.id} —</b>
						<span>x:</span>
						<input
							type="number"
							value={point.x}
							onChange={(event) => {
								const nextInputPoints = [...inputPoints];
								nextInputPoints[point.id] = {
									...inputPoints[point.id],
									x: parseFloat(event.target.value),
								};
								setInputPoints(nextInputPoints);
							}}
						/>
						<span> y:</span>
						<input
							type="number"
							value={point.y}
							onChange={(event) => {
								const nextInputPoints = [...inputPoints];
								nextInputPoints[point.id] = {
									...inputPoints[point.id],
									y: parseFloat(event.target.value),
								};
								setInputPoints(nextInputPoints);
							}}
						/>
					</div>
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
	);
}

export default App;
