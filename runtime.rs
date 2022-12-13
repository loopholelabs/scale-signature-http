use crate::http_signature::{Encode, Decode, HttpContext, HttpRequest, HttpResponse};
//use scale_signature::RuntimeContext;

struct Context {
    generated: HttpContext,
    buffer: &mut Cursor<&mut Vec<u8>>,
}

pub trait RuntimeContext {
    fn read(&mut self) -> HttpContext;
    fn write(&self) -> Vec<u8>;
    fn error(&self, err: std::io::Error) -> Vec<u8>;
    fn generated(&self) -> &HttpContext;
    fn new(self) -> Self;
}

impl RuntimeContext for Context {
    fn new(self) -> Self {
        Context {
            generated: HttpContext {
                        request: HttpRequest {
                            headers: HashMap::new(),
                            uri: "".to_string(),
                            method: "".to_string(),
                            content_length: 0,
                            protocol: "".to_string(),
                            ip: "".to_string(),
                            body: Vec::new()
                        },
                        response: HttpResponse {
                            headers: HashMap::new(),
                            status_code: 0,
                            body: Vec::new()
                        },
                   }
            },
          buffer: &mut Cursor<&mut Vec<u8>>,
    }

    fn generated(&self) -> &HttpContext {
        &self.generated
    }

    fn read(&mut self) -> HttpContext {
        Decode::decode(self.buffer).unwrap().unwrap()
    }


    fn write(&self) -> Vec<u8> {
        let mut cursor = Cursor::new(Vec::new());
        let _ = Encode::encode(self, &mut cursor);
        cursor.into_inner()
    }

    fn error(&self, err: std::io::Error) -> Vec<u8> {
        let mut cursor = Cursor::new(Vec::new());
        let _ = Error::error(self, &mut cursor);
        cursor.into_inner()
    }
}
