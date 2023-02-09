/*
    Copyright 2022 Loophole Labs

    Licensed under the Apache License, Version 2.0 (the "License");
    you may not use this file except in compliance with the License.
    You may obtain a copy of the License at

           http://www.apache.org/licenses/LICENSE-2.0

    Unless required by applicable law or agreed to in writing, software
    distributed under the License is distributed on an "AS IS" BASIS,
    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    See the License for the specific language governing permissions and
    limitations under the License.
*/

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
pub type Context = HttpContext;

#[cfg(not(target_arch = "wasm32"))]
pub fn new() -> Context {
    Context {
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
    }
}
