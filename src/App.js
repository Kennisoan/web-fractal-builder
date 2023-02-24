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
	const [points, setPoints] = useState([{ id: 0, x: 0, y: 0 }]);
	const [prevPoints, setPrevPoints] = useState([{ id: 0, x: 0, y: 0 }]);
	const [iterations, setIterations] = useState(3);

	useEffect(() => {
		init().then(() => {
			let nextPoints = fractal(inputPoints, iterations);
			setPrevPoints(points);
			setPoints(nextPoints);
			console.log(nextPoints);
		});
	}, [inputPoints, iterations]);

	return (
		<div className="App">
			<h1>Hello fractal world!</h1>
			<Line
				points={points}
				prevPoints={prevPoints}
				svgWidth={500}
				svgHeight={500}
			/>

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

				<div class="control-buttons">
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
					</button>{" "}
					<button
						onClick={() => {
							const nextInputPoints = [...inputPoints];
							nextInputPoints.splice(-1, 1);
							console.log(nextInputPoints);
							setInputPoints(nextInputPoints);
						}}
						disabled={inputPoints.length <= 3}
					>
						Remove point
					</button>
				</div>
			</div>
		</div>
	);
}

export default App;
