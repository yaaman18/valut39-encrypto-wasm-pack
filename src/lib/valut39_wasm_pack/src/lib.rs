use wasm_bindgen::prelude::*;

mod encrypto;

#[wasm_bindgen]
pub async fn handle_data(
   input_seed: String,
   password: String,
) -> Result<String, JsValue> {
   encrypto::handle_data(input_seed, password)
       .await
       .map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen]
pub async fn minimalize_seeds(
   input_seed_phrase: String,
) -> Result<String, JsValue> {
   encrypto::minimalize_seeds(encrypto::MinimalizeSeedsArgs {
       input_seed_phrase,
   })
   .await
   .map_err(|e| JsValue::from_str(&e.to_string()))
}