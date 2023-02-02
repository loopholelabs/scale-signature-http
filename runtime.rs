#![cfg(not(target_arch = "wasm32"))]

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

use crate::context::Context;
use crate::http_signature::{Decode, Encode, HttpContext};
use scale_signature::{RuntimeContext as RuntimeContextTrait, Signature as SignatureTrait};
use std::io::Cursor;

pub type RuntimeContext = Context;

impl SignatureTrait for Context {
    fn runtime_context(&mut self) -> &mut dyn RuntimeContextTrait {
        self
    }
}

impl RuntimeContextTrait for RuntimeContext {
    fn read(&mut self, b: &mut Vec<u8>) -> Option<Box<dyn std::error::Error>> {
        let mut cursor = Cursor::new(b);
        let result = HttpContext::decode(&mut cursor);
        return match result {
            Ok(context) => {
                *self = context.unwrap();
                None
            }
            Err(err) => Some(err),
        };
    }

    fn write(&self) -> Vec<u8> {
        let mut cursor = Cursor::new(Vec::new());
        let _ = Encode::encode(self.clone(), &mut cursor);
        cursor.into_inner()
    }

    fn error(&self, error: Box<dyn std::error::Error>) -> Vec<u8> {
        let mut cursor = Cursor::new(Vec::new());
        Encode::internal_error(self.clone(), &mut cursor, error);
        cursor.into_inner()
    }
}
