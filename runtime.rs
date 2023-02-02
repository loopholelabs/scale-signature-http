#![cfg(not(target_arch = "wasm32"))]

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
                self.generated = context.unwrap();
                None
            }
            Err(err) => Some(err),
        };
    }

    fn write(&self) -> Vec<u8> {
        let mut cursor = Cursor::new(Vec::new());
        let _ = Encode::encode(self.generated.clone(), &mut cursor);
        cursor.into_inner()
    }

    fn error(&self, error: Box<dyn std::error::Error>) -> Vec<u8> {
        let mut cursor = Cursor::new(Vec::new());
        Encode::internal_error(self.generated.clone(), &mut cursor, error);
        cursor.into_inner()
    }
}

impl Context {
    pub fn generated(&self) -> &HttpContext {
        &self.generated
    }
}
