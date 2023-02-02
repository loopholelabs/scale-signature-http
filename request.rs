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

use crate::http_signature::{HttpRequest, HttpStringList};
use std::collections::HashMap;
use std::string::String;

pub type Request = HttpRequest;

impl Request {
    pub fn method(&mut self) -> String {
        self.method.clone()
    }

    pub fn set_method(&mut self, method: String) -> &mut Self {
        self.method = method;
        self
    }

    pub fn uri(&mut self) -> String {
        self.uri.clone()
    }

    pub fn set_uri(&mut self, uri: String) -> &mut Self {
        self.uri = uri;
        self
    }

    pub fn body(&mut self) -> Vec<u8> {
        self.body.clone()
    }

    pub fn set_body(&mut self, body: String) -> &mut Self {
        return self.set_body_bytes(body.as_bytes().to_vec());
    }

    pub fn set_body_bytes(&mut self, bytes: Vec<u8>) -> &mut Self {
        self.body = bytes;
        self.content_length = self.body.len() as i64;
        self
    }

    pub fn content_length(&mut self) -> i64 {
        self.content_length
    }

    pub fn remote_ip(&mut self) -> String {
        self.ip.clone()
    }

    pub fn protocol(&mut self) -> String {
        self.protocol.clone()
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
