import init, { wasm_render, wasm_calc_physics_step, wasm_handle_mouse_down, wasm_handle_mouse_up } from "../pkg/charge_hockey.js"
let canvas = document.getElementById("display");
let ctx = canvas.getContext("2d");

let mousePosition = { x: 0, y: 0 };

canvas.addEventListener("mousemove", (event) => {
    const rect = canvas.getBoundingClientRect();
    mousePosition.x = event.clientX - rect.left;
    mousePosition.y = event.clientY - rect.top;
});


async function render() {
    await init();

    await wasm_render(ctx, mousePosition.x, mousePosition.y, false);

    requestAnimationFrame(render);
}

render();

let endTime = performance.now();

async function calc_physics_step() {
    await init();

    let deltaTime = performance.now() - endTime
    endTime = performance.now();

    await wasm_calc_physics_step(deltaTime);
}

setInterval(calc_physics_step, 10);

async function handle_mouse_down() {
    await init();

    await wasm_handle_mouse_down(mousePosition.x, mousePosition.y);
}

async function handle_mouse_up() {
    await init();

    await wasm_handle_mouse_up(mousePosition.x, mousePosition.y);
}

canvas.addEventListener("mousedown", async (event) => {
    handle_mouse_down();
});

canvas.addEventListener("mouseup", async (event) => {
    handle_mouse_up();
});
