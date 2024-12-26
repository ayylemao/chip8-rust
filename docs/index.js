import __wbg_init, { Chip8 } from './chip8_rust.js';

const CHIP8_KEYMAP = {
    'Numpad0': 0x0, 'Numpad1': 0x1, 'Numpad2': 0x2, 'Numpad3': 0x3,
    'Numpad4': 0x4, 'Numpad5': 0x5, 'Numpad6': 0x6, 'Numpad7': 0x7,
    'Numpad8': 0x8, 'Numpad9': 0x9,
    'KeyA': 0xA, 'KeyB': 0xB, 'KeyC': 0xC, 'KeyD': 0xD, 'KeyE': 0xE, 'KeyF': 0xF
};

const keyStates = new Array(16).fill(false);

function setupKeyListeners() {
    window.addEventListener('keydown', (event) => {
        const chip8Key = CHIP8_KEYMAP[event.code];
        if (chip8Key !== undefined) {
            keyStates[chip8Key] = 1;
        }
    });

    window.addEventListener('keyup', (event) => {
        const chip8Key = CHIP8_KEYMAP[event.code];
        if (chip8Key !== undefined) {
            keyStates[chip8Key] = 0;
        }
    });
}

async function run() {
    await __wbg_init(); 
    console.log("WASM module loaded successfully!");

    const canvas = document.getElementById('chip8-canvas');
    const context = canvas.getContext('2d');
    console.log("Canvas created");

    const chip8 = new Chip8();
    chip8.init();
    setupKeyListeners();

    const scale = 10;
    let lastLogicTime = performance.now();
    const LOGIC_INTERVAL = 1.85;
    
    let stepsCount = 0;
    let lastFrequencyTime = performance.now();
    
    const updateRateDisplay = document.getElementById("update-rate");
    
    function mainLoop() {
        const now = performance.now();
    
        while (now - lastLogicTime >= LOGIC_INTERVAL) {
            chip8.update_keyboard(keyStates);
            chip8.step();
            lastLogicTime += LOGIC_INTERVAL;
            stepsCount++;
        }
    
        if (now - lastFrequencyTime >= 1000) {
            const frequencyHz = stepsCount;
            updateRateDisplay.textContent = `CPU Frequency: ${frequencyHz} Hz`;
            stepsCount = 0;
            lastFrequencyTime = now;
        }
    
        chip8.render(context, scale);
    
        requestAnimationFrame(mainLoop);
    };
    mainLoop();
}

run();
