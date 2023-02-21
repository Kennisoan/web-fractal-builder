import React from "react";

export function IterInput({ iterations, setIterations }) {
	return (
		<div className="iter-input-container">
			<label htmlFor="iterations-slider">Iterations:</label>
			<input
				type="range"
				id="iterations-slider"
				min={1}
				max={8}
				value={iterations}
				onChange={(event) => {
					setIterations(event.target.value);
				}}
			/>
		</div>
	);
}

export function PointInput({ point, inputPoints, setInputPoints }) {
	return (
		<div className="point-input-container">
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
}
