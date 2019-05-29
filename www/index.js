'use-strict';

import {computeBoardGeometry} from './geometry.js';
import {IMAGES} from './images.js';

let canvas = document.getElementById("board-canvas");
const ctx = canvas.getContext("2d");
ctx.drawBounds = b => ctx.fillRect(b.minx, b.miny, b.width, b.height);
ctx.drawImageInBounds = (im, b) => ctx.drawImage(im, b.minx, b.miny, b.width, b.height);
let windowWidth = () => window.innerWidth;
let windowHeight = () => window.innerHeight;

function clearCanvas(canvas) {
    canvas.getContext("2d").clearRect(0, 0, canvas.width, canvas.height);
}

function isLightIndex(index) {
	let [row, col] = [Math.floor(index / 8), index % 8];
	return (row + col) % 2 === 0; 
}



let renderOp = () => {
    clearCanvas(canvas)
    let w = windowWidth();
    let h = windowHeight();
    canvas.width = w;
    canvas.height = h;
    ctx.fillStyle = 'black';
    //ctx.fillRect(0, 0, w, h);

    let [lightFill, darkFill] = ["#e5c9ae", "#442100"]
    let geometry = computeBoardGeometry(w, h, 64, "b");
    ctx.fillStyle = darkFill;
    ctx.drawBounds(geometry.back);
    ctx.fillStyle = lightFill;
    for (var i = 0; i < 64; i++) {
    	if (isLightIndex(i)) {
    		ctx.drawBounds(geometry.squares[i]);
    	}
    }
    if (IMAGES.loaded) {
    	ctx.drawImageInBounds(IMAGES.images[0], geometry.squares[0]);
    }

    requestAnimationFrame(renderOp);
}
requestAnimationFrame(renderOp);
