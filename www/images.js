'use strict'

export const IMAGES = {
	loaded: false,
	images: [
		createImg("./resources/WP64.png", 0),
		createImg("./resources/WN64.png", 1),
		createImg("./resources/WB64.png", 2),
		createImg("./resources/WR64.png", 3),
		createImg("./resources/WQ64.png", 4),
		createImg("./resources/WK64.png", 5),

		createImg("./resources/BP64.png", 6),
		createImg("./resources/BN64.png", 7),
		createImg("./resources/BB64.png", 8),
		createImg("./resources/BR64.png", 9),
		createImg("./resources/BQ64.png", 10),
		createImg("./resources/BK64.png", 11),
	]
}

const LOADED = new Array(12).fill(false);

function createImg(path, index) {
	let img = new Image();
	img.src = path;
	img.addEventListener("load", () => updateLoaded(index));
	return img
}

function updateLoaded(index) {
	LOADED[index] = true;
	IMAGES.loaded = LOADED.every(x => x);
}