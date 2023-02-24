import { React, useState, useEffect } from "react";
import { motion } from "framer-motion";
import { interpolate } from "flubber";

function Line({ points, prevPoints, svgWidth, svgHeight }) {
	function getPath(points) {
		let minX = Number.MAX_SAFE_INTEGER;
		let maxX = Number.MIN_SAFE_INTEGER;
		let minY = Number.MAX_SAFE_INTEGER;
		let maxY = Number.MIN_SAFE_INTEGER;

		for (let point of points) {
			minX = Math.min(minX, point.x);
			maxX = Math.max(maxX, point.x);
			minY = Math.min(minY, point.y);
			maxY = Math.max(maxY, point.y);
		}

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
		let path = `M ${points[0].x * scale + offsetX} ${
			points[0].y * scale + offsetY
		}`;

		// Loop through the rest of the points and add a line segment for each one
		for (let i = 1; i < points.length; i++) {
			path += ` L ${points[i].x * scale + offsetX} ${
				points[i].y * scale + offsetY
			}`;
		}
		// path += " z";
		return path;
	}

	const [pathData, setPathData] = useState(getPath(points));
	useEffect(() => {
		const interpolator = interpolate(getPath(prevPoints), getPath(points));

		const animation = requestAnimationFrame((timestamp) => {
			const start = timestamp || new Date().getTime();
			const duration = 100; // transition duration in milliseconds
			const step = (timestamp) => {
				const progress = (timestamp - start) / duration;
				if (progress >= 1) {
					setPathData(getPath(points));
				} else {
					setPathData(interpolator(progress));
					requestAnimationFrame(step);
				}
			};
			requestAnimationFrame(step);
		});

		return () => cancelAnimationFrame(animation);
	}, [points]);

	return (
		<div className="line-container">
			<svg width={svgWidth} height={svgHeight}>
				<motion.path
					d={pathData}
					strokeWidth="2"
					stroke="gray"
					strokeLinecap="round"
					strokeLinejoin="round"
					pathLength="0.5"
					fill="transparent"
				/>
			</svg>
		</div>
	);
}

export default Line;
