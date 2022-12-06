use http_signature::{Encode, Decode, HttpContext, HttpRequest, HttpResponse};

struct RuntimeContext {
    generated: HttpContext,
    buffer: &mut Cursor<&mut Vec<u8>>,
}

pub trait RuntimeCtx {
    fn new(self) -> Self;
    fn read(&mut self) -> HttpContext;
    fn write(&self) -> Vec<u8>;
    fn error(&self, err: std::io::Error) -> Vec<u8>;
    fn generated(&self) -> &HttpContext;
}

impl RuntimeCtx for RuntimeContext {
    fn new(self) -> Self {
        RuntimeContext {
            generated: HttpContext {
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
