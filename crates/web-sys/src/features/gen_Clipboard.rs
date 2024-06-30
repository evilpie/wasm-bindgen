#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = EventTarget , extends = :: js_sys :: Object , js_name = Clipboard , typescript_type = "Clipboard")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Clipboard` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Clipboard)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Clipboard`*"]
    pub type Clipboard;
    # [wasm_bindgen (method , structural , js_class = "Clipboard" , js_name = read)]
    #[doc = "The `read()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Clipboard/read)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Clipboard`*"]
    pub fn read(this: &Clipboard) -> ::js_sys::Promise;
    # [wasm_bindgen (method , structural , js_class = "Clipboard" , js_name = readText)]
    #[doc = "The `readText()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Clipboard/readText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Clipboard`*"]
    pub fn read_text(this: &Clipboard) -> ::js_sys::Promise;
    # [wasm_bindgen (method , structural , js_class = "Clipboard" , js_name = write)]
    #[doc = "The `write()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Clipboard/write)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Clipboard`*"]
    pub fn write(this: &Clipboard, data: &::wasm_bindgen::JsValue) -> ::js_sys::Promise;
    # [wasm_bindgen (method , structural , js_class = "Clipboard" , js_name = writeText)]
    #[doc = "The `writeText()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Clipboard/writeText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Clipboard`*"]
    pub fn write_text(this: &Clipboard, data: &str) -> ::js_sys::Promise;
}
