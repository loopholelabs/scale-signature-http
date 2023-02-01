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
use std::string::String;

pub trait Request {
    fn request(self) -> Self;
    fn method(&mut self) -> String;
    fn set_method(&mut self, method: String) -> &mut Self;
    fn uri(&mut self) -> String;
    fn remote_ip(&mut self) -> String;
    fn body(&mut self) -> Vec<u8>;
    fn set_body(&mut self, body: String) -> &mut Self;
    fn set_body_bytes(&mut self, bytes: Vec<u8>) -> &mut Self;
    fn headers(&self) -> &HashMap<String, HttpStringList>;
    fn get_headers(&self, key: &String) -> Option<&HttpStringList>;
    fn set_headers(&mut self, key: String, value: Vec<String>);
}

impl Request for Context {
    fn request(self) -> Self {
        self
    }

    fn method(&mut self) -> String {
        self.generated.request.method.clone()
    }

    fn set_method(&mut self, method: String) -> &mut Self {
        self.generated.request.method = method;
        self
    }

    fn uri(&mut self) -> String {
        self.generated.request.uri.clone()
    }

    fn remote_ip(&mut self) -> String {
        self.generated.request.ip.clone()
    }

    fn body(&mut self) -> Vec<u8> {
        self.generated.request.body.clone()
    }

    fn set_body(&mut self, body: String) -> &mut Self {
        self.generated.request.body = body.as_bytes().to_vec();
        self
    }

    fn set_body_bytes(&mut self, bytes: Vec<u8>) -> &mut Self {
        self.generated.request.body = bytes;
        self
    }

    fn headers(&self) -> &HashMap<String, HttpStringList> {
        &self.generated.request.headers
    }

    fn get_headers(&self, key: &String) -> Option<&HttpStringList> {
        self.generated.request.headers.get(key)
    }

    fn set_headers(&mut self, key: String, value: Vec<String>) {
        self.generated.request.headers.insert(key, HttpStringList { value });
    }
}
