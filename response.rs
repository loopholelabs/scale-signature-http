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

use crate::http_signature::{HttpResponse, HttpStringList};
use std::collections::HashMap;

pub type Response = HttpResponse;

impl Response {
    pub fn status_code(&mut self) -> i32 {
        self.status_code.clone()
    }

    pub fn body(&mut self) -> Vec<u8> {
        self.body.clone()
    }

    pub fn set_body(&mut self, body: String) -> &mut Self {
        self.body = body.as_bytes().to_vec();
        self
    }

    pub fn set_body_bytes(&mut self, bytes: Vec<u8>) -> &mut Self {
        self.body = bytes;
        self
    }

    pub fn headers(&self) -> &HashMap<String, HttpStringList> {
        &self.headers
    }

    pub fn get_headers(&self, key: &String) -> Option<&HttpStringList> {
        self.headers.get(key)
    }

    pub fn set_headers(&mut self, key: String, value: Vec<String>) {
        self.headers.insert(key, HttpStringList { value });
    }
}
