let canvas = document.getElementById("board-canvas");
const ctx = canvas.getContext("2d");
let windowWidth = () => window.innerWidth;
let windowHeight = () => window.innerHeight;

function clearCanvas(canvas) {
    canvas.getContext("2d").clearRect(0, 0, canvas.width, canvas.height);
}


let renderOp = () => {
    clearCanvas(canvas)
    let w = windowWidth();
    let h = windowHeight();
    canvas.width = w;
    canvas.height = h;
    ctx.fillStyle = 'black';
    ctx.fillRect(0, 0, w, h);
    requestAnimationFrame(renderOp);
}

requestAnimationFrame(renderOp);
