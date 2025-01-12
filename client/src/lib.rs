use wasm_bindgen::prelude::*;
use web_sys::{MessageEvent, WebSocket};
use shared_types::payload::join_room::join_room_request_data::JoinRoomRequestData;
use shared_types::payload::raw_request::{Operation, RawRequest};
use shared_types::payload::send_message::send_message_request_data::SendMessageRequestData;

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
        let client = Client {
            api_endpoint,
            web_socket: ws,
        };

        {
            let ws_for_onopen = client.web_socket.clone();
            let cloned_client = client.clone_for_js()?;
            let onopen_callback = Closure::<dyn FnMut()>::new(move || {
                cloned_client.on_open();
            });
            ws_for_onopen.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
            onopen_callback.forget();
        }

        {
            let ws_for_onclose = client.web_socket.clone();
            let cloned_client = client.clone_for_js()?;
            let onclose_callback = Closure::<dyn FnMut()>::new(move || {
                cloned_client.on_close();
            });
            ws_for_onclose.set_onclose(Some(onclose_callback.as_ref().unchecked_ref()));
            onclose_callback.forget();
        }

        {
            let ws_for_onmessage = client.web_socket.clone();
            let cloned_client = client.clone_for_js()?;
            let onmessage_callback = Closure::<dyn FnMut(MessageEvent)>::new(move |event| {
                cloned_client.on_message(event);
            });
            ws_for_onmessage.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
            onmessage_callback.forget();
        }

        Ok(client)
    }

    fn on_open(&self) {
        console_log!("WebSocket connection opened");
    }

    fn on_message(&self, event: MessageEvent) {
        if event.data().is_string() {
            let message = event.data().as_string().unwrap();
            console_log!("WebSocket message received: {}", message);
        }
    }

    fn on_close(&self) {
        console_log!("WebSocket connection closed");
    }

    #[wasm_bindgen]
    pub fn create_room(&self) {
        let request = RawRequest::new(Operation::CreateRoom, None);
        let json = request.to_json().unwrap();
        self.web_socket.send_with_str(&json).unwrap();
    }

    #[wasm_bindgen]
    pub fn join_room(&self, room_id: String) {
        let data = JoinRoomRequestData::new(room_id.clone());
        let request = RawRequest::new(Operation::JoinRoom, Some(data.to_value().unwrap()));
        let json = request.to_json().unwrap();
        self.web_socket.send_with_str(&json).unwrap();
    }

    #[wasm_bindgen]
    pub fn leave_room(&self) {
        let request = RawRequest::new(Operation::LeaveRoom, None);
        let json = request.to_json().unwrap();
        self.web_socket.send_with_str(&json).unwrap();
    }

    #[wasm_bindgen]
    pub fn send_message(&self, message: String) {
        let data = SendMessageRequestData::new(message.clone());
        let request = RawRequest::new(Operation::SendMessage, Some(data.to_value().unwrap()));
        let json = request.to_json().unwrap();
        self.web_socket.send_with_str(&json).unwrap();
    }

    fn clone_for_js(&self) -> Result<Client, JsValue> {
        Ok(Client {
            api_endpoint: self.api_endpoint.clone(),
            web_socket: self.web_socket.clone(),
        })
    }
}
