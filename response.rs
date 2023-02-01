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

#![allow(unused_variables)]
use crate::context::Context;
use crate::http_signature::{HttpStringList};
use std::collections::HashMap;

type Response = Context;

impl Context {
    pub fn response(&mut self) -> &mut Response {
        self as &mut Response
    }
}

impl Response {
    pub fn status_code(&mut self) -> i32 {
        self.generated.response.status_code.clone()
    }

    pub fn body(&mut self) -> Vec<u8> {
        self.generated.response.body.clone()
    }

    pub fn set_body(&mut self, body: String) -> &mut Self {
        self.generated.response.body = body.as_bytes().to_vec();
        self
    }

    pub fn set_body_bytes(&mut self, bytes: Vec<u8>) -> &mut Self {
        self.generated.response.body = bytes;
        self
    }

    pub fn headers(&self) -> &HashMap<String, HttpStringList> {
        &self.generated.response.headers
    }

    pub fn get_headers(&self, key: &String) -> Option<&HttpStringList> {
        self.generated.response.headers.get(key)
    }

    pub fn set_headers(&mut self, key: String, value: Vec<String>) {
        self.generated.response.headers.insert(key, HttpStringList { value });
    }
}
