import "core-js/stable";
import "regenerator-runtime/runtime";

import { render } from "./render.js";

async function run() {
    let module = await import('../pkg/index.js');
    let othellot = module.make_othello();
    let canvas = document.getElementById("canv")
    render(canvas, othellot)
}

run()
