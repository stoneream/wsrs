use wasm_bindgen::prelude::*;
use web_sys::WebSocket;

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(start)]
pub fn main_js() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub struct Client {
    api_endpoint: String,
    web_socket: WebSocket,
}

#[wasm_bindgen]
impl Client {
    #[wasm_bindgen(constructor)]
    pub fn new(api_endpoint: String) -> Result<Client, JsValue> {
        let ws = WebSocket::new(&api_endpoint)?;

        let onopen_callback = Closure::<dyn FnMut()>::new(move || {
            console_log!("WebSocket connection opened");
        });
        ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
        onopen_callback.forget();

        let client = Client {
            api_endpoint,
            web_socket: ws,
        };
        Ok(client)
    }

    fn clone_for_js(&self) -> Result<Client, JsValue> {
        Ok(Client {
            api_endpoint: self.api_endpoint.clone(),
            web_socket: self.web_socket.clone(),
        })
    }
}
