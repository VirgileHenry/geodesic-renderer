

async fn create_renderer_to_json_values(canvas: web_sys::HtmlCanvasElement) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
    crate::renderer::Renderer::new(canvas).await
        .map(wasm_bindgen::JsValue::from)
        .map_err(wasm_bindgen::JsValue::from)
}

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn create_renderer(canvas: web_sys::HtmlCanvasElement) -> web_sys::js_sys::Promise {
    wasm_bindgen_futures::future_to_promise(create_renderer_to_json_values(canvas))
}

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn render(renderer: &crate::renderer::Renderer, world: &crate::world::World) -> Result<(), crate::renderer::RenderError> {
    renderer.render(world)
}

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn create_world() -> crate::world::World {
    crate::world::World::new()
}

