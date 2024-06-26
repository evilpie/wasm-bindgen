#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = GPUOrigin2DDict)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuOrigin2dDict` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuOrigin2dDict`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuOrigin2dDict;
    #[wasm_bindgen(method, setter = "x")]
    fn x_shim(this: &GpuOrigin2dDict, val: u32);
    #[wasm_bindgen(method, setter = "y")]
    fn y_shim(this: &GpuOrigin2dDict, val: u32);
}
#[cfg(web_sys_unstable_apis)]
impl GpuOrigin2dDict {
    #[doc = "Construct a new `GpuOrigin2dDict`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuOrigin2dDict`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `x` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuOrigin2dDict`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn x(&mut self, val: u32) -> &mut Self {
        self.x_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `y` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuOrigin2dDict`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn y(&mut self, val: u32) -> &mut Self {
        self.y_shim(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for GpuOrigin2dDict {
    fn default() -> Self {
        Self::new()
    }
}
