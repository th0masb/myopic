'use-strict';

export function computeSquareLocations(containerWidth, containerHeight, sizeCap, orientation) {
	if ([containerWidth, containerHeight, sizeCap, orientation].some(x => x === undefined)) {
		throw "Look at your call args"
	}
	let squareSize = Math.min(containerWidth / 8, containerHeight / 8, sizeCap);
	let boardSize = 8 * squareSize;
	let [originx, originy] = [(containerWidth - boardSize) / 2, (containerHeight - boardSize) / 2];
	let dest = [];
	for (var i = 0; i < 64; i++) {
		let [row, col] = [Math.floor(i / 8), i % 8];
		dest.push(bounds(originx + col * squareSize, originy + row * squareSize, squareSize, squareSize));
	}
	if (orientation === "w") {
		dest.reverse();
	}
	return dest;
}

function bounds(minx, miny, width, height) {
	return {minx: minx, miny: miny, width: width, height: height};
}

