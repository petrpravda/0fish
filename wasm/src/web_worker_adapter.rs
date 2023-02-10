use std::rc::Rc;
use wasm_bindgen::JsValue;
use zerofish::engine::OutputAdapter;
// use wasm_bindgen::prelude::*;
use web_sys::Worker;

pub struct WebWorkerOutputAdapter {
    // string_buffer: String,
    pub(crate) worker: Rc<Worker>,
    pub(crate) stop_signalling: Rc<js_sys::Int32Array>,
}

impl WebWorkerOutputAdapter {
    pub fn new(worker: &Worker, stop_signalling: &js_sys::Int32Array) -> Self {
        Self {
            // string_buffer: String::new(),
            worker: Rc::new(worker.clone()),
            stop_signalling: Rc::new(stop_signalling.clone())
        }
    }
}

impl OutputAdapter for WebWorkerOutputAdapter {
    fn writeln(&mut self, output: &str) {
        // let mut string_buffer = String::new();
        // string_buffer.push_str(output);
        // string_buffer.push('\n');

        let message = JsValue::from(output.to_string());
        match self.worker.post_message(&message) {
            Ok(_) => {
                // message was successfully posted
            }
            Err(_error) => { // TODO handle
                // an error occurred, handle it here
                //console_error!("Error posting message: {:?}", error);
            }
        }
    }

    fn is_stop_signalled(&self) -> bool {
        let signal_int: i32 = self.stop_signalling.get_index(0);
        signal_int == 1
    }
}

impl ToString for WebWorkerOutputAdapter {
    fn to_string(&self) -> String {
        "n/a".to_string()
        //self.string_buffer.clone()
    }
}
