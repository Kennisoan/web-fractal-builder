import React from "react";

export function IterInput({ iterations, setIterations }) {
	return (
		<div className="iter-input-container">
			<label htmlFor="iterations-slider">Iterations ({iterations}):</label>
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
					const value = event.target.value == "" ? 0 : event.target.value;
					const nextInputPoints = [...inputPoints];
					nextInputPoints[point.id] = {
						...inputPoints[point.id],
						x: Math.max(0, parseFloat(value)),
					};
					setInputPoints(nextInputPoints);
				}}
			/>
			<span> y:</span>
			<input
				type="number"
				value={point.y}
				onChange={(event) => {
					const value = event.target.value == "" ? 0 : event.target.value;
					const nextInputPoints = [...inputPoints];
					nextInputPoints[point.id] = {
						...inputPoints[point.id],
						y: Math.max(0, parseFloat(value)),
					};
					setInputPoints(nextInputPoints);
				}}
			/>
		</div>
	);
}
