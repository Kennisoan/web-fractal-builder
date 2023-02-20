import React from "react";

function Line({ data, svgWidth, svgHeight }) {
	// Break if array is empty
	if (data.length == 0) {
		return <svg></svg>;
	}

	// Find the minimum and maximum x and y values in the data list
	const minX = Math.min(...data.map((point) => point.x));
	const maxX = Math.max(...data.map((point) => point.x));
	const minY = Math.min(...data.map((point) => point.y));
	const maxY = Math.max(...data.map((point) => point.y));

	// Calculate the scaling factor to fit the data inside the SVG
	const xRange = maxX - minX;
	const yRange = maxY - minY;
	const scaleX = xRange !== 0 ? svgWidth / xRange : 1;
	const scaleY = yRange !== 0 ? svgHeight / yRange : 1;
	const scale = Math.min(scaleX, scaleY);

	// Shift the scaled points so that the minimum x and y values are aligned with the edges of the SVG
	const offsetX = (svgWidth - scale * xRange) / 2 - minX * scale;
	const offsetY = (svgHeight - scale * yRange) / 2 - minY * scale;

	// Initialize the path data with the first point in the list, scaled and shifted by the factor
	let path = `M ${data[0].x * scale + offsetX} ${data[0].y * scale + offsetY}`;

	// Loop through the rest of the points and add a line segment for each one
	for (let i = 1; i < data.length; i++) {
		path += ` L ${data[i].x * scale + offsetX} ${data[i].y * scale + offsetY}`;
	}

	return (
		<div className="line-container">
			<svg width={svgWidth} height={svgHeight}>
				<path
					d={path}
					strokeWidth="3"
					stroke="#FF0040"
					strokeLinecap="round"
					strokeLinejoin="round"
					fill="none"
				/>
			</svg>
			<span className="minX">{minX}</span>
			<span className="maxX">{maxX}</span>
			<span className="maxY">{maxY}</span>
			<span className="minY">{minY}</span>
		</div>
	);
}

export default Line;
