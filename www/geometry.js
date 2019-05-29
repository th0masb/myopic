'use-strict';

export function computeBoardGeometry(containerWidth, containerHeight, sizeCap, orientation) {
	if ([...arguments].some(x => x === undefined)) {
		throw "Look at your call args"
	}
	let squareSize = Math.min(containerWidth / 9, containerHeight / 9, sizeCap);
	let boardSize = 9 * squareSize;
	let [originx, originy] = [(containerWidth - boardSize ) / 2, (containerHeight - boardSize) / 2];
	let dest = [];
	const squareBounds = (minx, miny) => bounds(minx, miny, squareSize, squareSize);
	for (var i = 0; i < 64; i++) {
		let [row, col, sz] = [Math.floor(i / 8), i % 8];
		dest.push(squareBounds(originx + (col + 0.5) * squareSize, originy + (row + 0.5) * squareSize));
	}
	if (orientation === "w") {
		dest.reverse();
	}

	return {squares: dest, back: bounds(originx, originy, boardSize, boardSize)};
}

function bounds(minx, miny, width, height) {
	return {minx: minx, miny: miny, width: width, height: height};
}

