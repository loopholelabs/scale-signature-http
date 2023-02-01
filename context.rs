use crate::http_signature::{HttpContext, HttpRequest, HttpResponse};
use std::collections::HashMap;

#[cfg(target_arch = "wasm32")]
pub struct Context {
    pub(crate) generated: HttpContext,
}

#[cfg(target_arch = "wasm32")]
pub fn new() -> Context {
    Context {
        generated: HttpContext {
            request: HttpRequest {
                headers: HashMap::new(),
                uri: "".to_string(),
                method: "".to_string(),
                content_length: 0,
                protocol: "".to_string(),
                ip: "".to_string(),
                body: Vec::new(),
            },
            response: HttpResponse {
                headers: HashMap::new(),
                status_code: 0,
                body: Vec::new(),
            },
        },
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub struct Context {
    pub(crate) generated: HttpContext,
    pub(crate) buffer: Vec<u8>,
}

#[cfg(not(target_arch = "wasm32"))]
pub fn new() -> Context {
    Context {
        generated: HttpContext {
            request: HttpRequest {
                headers: HashMap::new(),
                uri: "".to_string(),
                method: "".to_string(),
                content_length: 0,
                protocol: "".to_string(),
                ip: "".to_string(),
                body: Vec::new(),
            },
            response: HttpResponse {
                headers: HashMap::new(),
                status_code: 0,
                body: Vec::new(),
            },
        },
        buffer: Vec::new(),
    }
}