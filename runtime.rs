use http_signature::{Encode, Decode, Context, Request, Response};

struct RuntimeState {
    generated: Context,
    buffer: &mut Cursor<&mut Vec<u8>>,
}

pub trait RuntimeContext {
    fn new(self) -> Self;
    fn read(&mut self) -> Context;
    fn write(&self) -> Vec<u8>;
    fn error(&self, err: std::io::Error) -> Vec<u8>;
    fn generated(&self) -> &Context;
}

impl RuntimeContext for RuntimeState {
    fn new(self) -> Self {
        RuntimeContext {
            generated: Context {
                        request: Request {
                            headers: HashMap::new(),
                            method: "".to_string(),
                            content_length: 0,
                            protocol: "".to_string(),
                            i_p: "".to_string(),
                            body: Vec::new()
                        },
                        response: Response {
                            headers: HashMap::new(),
                            status_code: 0,
                            body: Vec::new()
                        },
                   }
            },
          buffer: &mut Cursor<&mut Vec<u8>>,
    }

    fn generated(&self) -> &Context {
        &self.generated
    }

    fn read(&mut self) -> Context {
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
