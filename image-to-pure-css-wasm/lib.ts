import wasm from "./pkg/image_to_pure_css_wasm_bg.wasm";
import init, { image_to_pure_css } from "./pkg/image_to_pure_css_wasm.js";
await init(wasm);
export { image_to_pure_css };
