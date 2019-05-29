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

function generateStartState() {
	let dest = new Array(63).fill(-1);
	let setLocs = obj => {
		for (const [k, v] of Object.entries(obj)) {
			v.forEach(function(loc) {
				dest[loc] = k;
			});
		}
	}
	// White pawns
	setLocs({0: [8, 9, 10, 11, 12, 13, 14, 15]})
	// White non-pawns
	setLocs({1: [1, 6], 2: [2, 5], 3: [0, 7], 4: [4], 5: [3]});
	// Black pawns
	setLocs({6: [48, 49, 50, 51, 52, 53, 54, 55]});
	// Black non-pawns
	setLocs({7: [57, 62], 8: [58, 61], 9: [56, 63], 10: [60], 11: [59]});
	return dest;
}

function renderBoard(ctx, geometry) {
	let [lightFill, darkFill] = ["#e5c9ae", "#7f3b00"];// "#442100"];
	ctx.fillStyle = darkFill;
    ctx.drawBounds(geometry.back);
    ctx.fillStyle = lightFill;
    for (var i = 0; i < 64; i++) {
    	if (isLightIndex(i)) {
    		ctx.drawBounds(geometry.squares[i]);
    	}
    }
}

function renderState(ctx, geometry, state) {
	if (IMAGES.loaded) {
		for (var i = 0; i < 64; i++) {
			let pieceIndex = state[i];
			if (pieceIndex > -1) {
				ctx.drawImageInBounds(IMAGES.images[pieceIndex], geometry.squares[i]);
			}
		}
	}
}


let renderOp = () => {
    clearCanvas(canvas)
    let w = windowWidth();
    let h = windowHeight();
    canvas.width = w;
    canvas.height = h;
    ctx.fillStyle = 'black';
    //ctx.fillRect(0, 0, w, h);

    
    let geometry = computeBoardGeometry(w, h, 64, "w");
    let state = generateStartState();
    renderBoard(ctx, geometry);
    renderState(ctx, geometry, state);

    requestAnimationFrame(renderOp);
}
requestAnimationFrame(renderOp);
