const SQUARE_SIZE = 80;
const WIDTH = 640;
const HEIGHT = 740;
const HEADER_HEIGHT = 100;

const GREEN = "#090";
const WHITE = "#fff";
const BLACK = "#000";
const GRAY = "#b4b4b4";
const DEEP_GRAY = "#464646";

const DEFAULT_BOARD =  [
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, -1, 1, 0, 0, 0,
    0, 0, 0, 1, -1, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
]

const coord = coordinate()

export function render(canvas, othellot) {

    canvas.width = WIDTH
    canvas.height = HEIGHT

    let ctx = canvas.getContext('2d');
    let click_sound = document.getElementById("click")

    drawBoard(ctx);
    drawDiscs(ctx, DEFAULT_BOARD);
    showDisplay(ctx, [2,2], "black")

    let click_handler = (e) => {
        if (e.screenY < 100) {
            return
        } 

        let index = offset_to_index(e.offsetX, e.offsetY)
        let diff = othellot.move_by_user(index)

        if (diff === undefined) {
            return
        }

        drawDiscs(ctx, diff)
        click_sound.play()
        
        let turn = othellot.which_turn()
        let score = othellot.get_score()
        showDisplay(ctx, score, turn)

        if (turn == "black") {
            return
        } else if (turn == "gameover") {
            play_gameover_sound(score)
            return
        }

        setTimeout(() => {
            for (let i = 0; i < 64; i++) {
                let diff = othellot.move_by_ai()

                drawDiscs(ctx, diff)
                click_sound.play()

                let turn = othellot.which_turn()
                let score = othellot.get_score()
                showDisplay(ctx, score, turn)

                if (turn === "black") {
                    return
                } else if (turn === "white") {
                    continue
                } else if (turn === "gameover") {
                    play_gameover_sound(score)
                    return
                }
            }
        }, 500)
   }
    canvas.addEventListener('click', click_handler);
}

function showDisplay(ctx, score, turn) {
    let display
    if (turn === "gameover") {
        let result = score[0] > score[1] ? "<WIN>" : score[0] < score[1] ? "<LOSE>" : "<DRAW>"
        display = result + ` BLACK: ${score[0]} / WHITE: ${score[1]}` 
    } else {
        display = `BLACK: ${score[0]} / WHITE: ${score[1]}` 
    }

    ctx.clearRect(0.0, 0.0, WIDTH, HEADER_HEIGHT);
    ctx.fillStyle = GREEN
    ctx.font = "20px sans-serif"
    ctx.textAlign = "center"
    ctx.fillText(display, WIDTH / 2, 40.0)
}

function play_gameover_sound(score) {
    const win_sound= document.getElementById("win")
    const lose_sound = document.getElementById("lose")
    score[0] > score[1] ? win_sound.play() : lose_sound.play()
}

function drawBoard(ctx) {
    ctx.clearRect(0, 0, WIDTH, HEIGHT);
    ctx.fillStyle = BLACK
    ctx.fillStyle = GREEN
    ctx.fillRect(0, HEADER_HEIGHT, WIDTH, WIDTH)

    for (var x = 0; x < 8; x++) {
        for (var y = 0; y < 8; y++) {
            ctx.strokeStyle = WHITE;
            ctx.beginPath();
            ctx.fillRect(x * SQUARE_SIZE, y * SQUARE_SIZE + HEADER_HEIGHT, SQUARE_SIZE, SQUARE_SIZE);
            ctx.strokeRect(x * SQUARE_SIZE, y * SQUARE_SIZE + HEADER_HEIGHT, SQUARE_SIZE, SQUARE_SIZE);
        }
    }
}

function drawDiscs(ctx, diff) {
    diff.forEach((v, i)=> {
        if (v === 1) {
            drawDisc(ctx, coord[i]["x"], coord[i]["y"], "black")
        } else if  (v === -1) {
            drawDisc(ctx, coord[i]["x"], coord[i]["y"], "white")
        }
    })
}

function drawDisc (ctx, x, y, color) {
    let grad = ctx.createLinearGradient(x, y, x + SQUARE_SIZE, y + SQUARE_SIZE);
    if (color === "black") {
        grad.addColorStop(0, BLACK);
        grad.addColorStop(0.4, DEEP_GRAY);
        grad.addColorStop(1, DEEP_GRAY);
    } else if (color === "white") {
        grad.addColorStop(0, GRAY);
        grad.addColorStop(0.4, WHITE);
        grad.addColorStop(1, WHITE);
    }
    ctx.fillStyle = grad;
    ctx.beginPath();
    ctx.arc(x + SQUARE_SIZE / 2.0, y + SQUARE_SIZE / 2.0, SQUARE_SIZE / 2.2, 0.0, 2.0 * Math.PI);
    ctx.fill()
}

function coordinate() {
    let coord = new Array()
    for (let x = 0; x < 8; x++) {
        for (let y = 0; y < 8; y++) {
            coord.push({ x: y * SQUARE_SIZE, y : x * SQUARE_SIZE + HEADER_HEIGHT })
        }
    }
    return coord;
}

function offset_to_index(x, y) {
    return 63 - (Math.floor(x / SQUARE_SIZE) + Math.floor((y - 100) / SQUARE_SIZE) * 8)
}
