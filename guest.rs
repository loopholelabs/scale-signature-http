#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use lazy_static::lazy_static;
use std::sync::Mutex;

use std::io::Cursor;
use std::mem;
use std::collections::HashMap;
use http_signature::{Encode, Decode, HttpContext, HttpRequest, HttpResponse};

lazy_static! {
    pub static ref PTR: Mutex<u32> = Mutex::new(0);
    pub static ref LEN: Mutex<u32> = Mutex::new(0);
    pub static ref READ_BUFFER: Mutex<Vec<u8>> = Mutex::new(Vec::with_capacity(0));
}

pub trait GuestContext {
    fn new() -> Self;
    fn from_read_buffer(self, read_buff: &mut Cursor<&mut Vec<u8>>) -> Self;
    fn to_write_buffer(self) -> (u32, u32);
    fn error_write_buffer(self) -> (u32, u32);
    fn next(self) -> Self;
    fn request(&mut self) -> &mut HttpRequest;
    fn response(&mut self) -> &mut HttpResponse;
}

impl GuestContext for Context {
    fn new()  -> Context {
            Context {
                    request: HttpRequest {
                        headers: HashMap::new(),
                        method: "".to_string(),
                        content_length: 0,
                        protocol: "".to_string(),
                        i_p: "".to_string(),
                        body: Vec::new()
                    },
                    response: HttpResponse {
                        headers: HashMap::new(),
                        status_code: 0,
                        body: Vec::new()
                    },
           }
    }

    fn from_read_buffer(self, read_buff: &mut Cursor<&mut Vec<u8>>) -> Self {
          Decode::decode(read_buff).unwrap().unwrap()
    }

    fn to_write_buffer(self) -> (u32, u32) {
        let mut cursor = Cursor::new(Vec::new());
        let _ = Encode::encode(self, &mut cursor);
        let mut vec = cursor.into_inner();
        vec.shrink_to_fit();

        let ptr = vec.as_ptr() as u32;
        let len = vec.len() as u32;

        //allows us to re-use allocations between invocations
        *READ_BUFFER.lock().unwrap() = vec;
        return (ptr, len)
    }

    fn error_write_buffer(self) -> (u32, u32) {
        let mut cursor = Cursor::new(Vec::new());

                // See: http.signature.go .internal_error, error checking in .decode
        let _ = Encode::encode_error(self, &mut cursor, error);
        let mut vec = cursor.into_inner();
        vec.shrink_to_fit();

        let ptr = vec.as_ptr() as u32;
        let len = vec.len() as u32;

        //allows us to re-use allocations between invocations
        *READ_BUFFER.lock().unwrap() = vec;
        return (ptr, len)
    }

    fn request(&mut self) -> &mut HttpRequest {
        &mut self.request
    }

    fn response(&mut self) -> &mut HttpResponse {
        &mut self.response
    }

    fn next(self) -> Self {
           let ptr_len = self.to_write_buffer();

           unsafe {
           //  calls resize from host side, which sets PTR and LEN
           _next(ptr_len.0, ptr_len.1);

           let ptr = PTR.lock().unwrap().clone();
           let len = LEN.lock().unwrap().clone();
           let mut vec = Vec::from_raw_parts(ptr as *mut u8, len as usize, len as usize);
           let mut constructed = Cursor::new(&mut vec);

           let empty_context: Context = Self::new();

           let from_buf = empty_context.from_read_buffer(&mut constructed);
           return from_buf;
           }
    }
}

#[link(wasm_import_module = "env")]
extern "C" {
    #[link_name = "next"]
    fn _next(ptr: u32, size: u32);
}
