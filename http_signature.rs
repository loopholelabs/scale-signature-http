// Code generated by polyglot-rs v0.5.1, DO NOT EDIT.
// source: signature.proto

use polyglot_rs::{Decoder, DecodingError, Encoder, Kind};
use std::collections::HashMap;
use std::io;
use std::io::Cursor;

pub trait Encode {
    fn encode(self, b: &mut Cursor<Vec<u8>>) -> Result<&mut Cursor<Vec<u8>>, io::Error>;
    fn internal_error(self, b: &mut Cursor<Vec<u8>>, error: &str);
}

pub trait Decode {
    fn decode(b: &mut Cursor<&mut Vec<u8>>) -> Result<Option<Self>, DecodingError>
    where
        Self: Sized;
}

pub struct HttpContext {
    pub request: HttpRequest,
    pub response: HttpResponse,
}

impl Encode for HttpContext {
    fn encode(self, b: &mut Cursor<Vec<u8>>) -> Result<&mut Cursor<Vec<u8>>, io::Error> {
        self.request.encode(b)?;
        self.response.encode(b)?;
        Ok(b)
    }

    fn internal_error(self, b: &mut Cursor<Vec<u8>>, error: &str) {
        b.encode_error(error).unwrap();
    }
}

impl Decode for HttpContext {
    fn decode(b: &mut Cursor<&mut Vec<u8>>) -> Result<Option<HttpContext>, DecodingError> {
        if b.decode_none() {
            return Ok(None);
        }

        if let Err(err) = b.decode_error() {
            return Err(err)
        };

        Ok(Some(HttpContext {
            request: HttpRequest::decode(b)?.ok_or(DecodingError::InvalidStruct)?,
            response: HttpResponse::decode(b)?.ok_or(DecodingError::InvalidStruct)?,
        }))
    }
}

pub struct HttpRequest {
    pub headers: HashMap<String, HttpStringList>,
    pub uri: String,
    pub method: String,
    pub content_length: i64,
    pub protocol: String,
    pub ip: String,
    pub body: Vec<u8>,
}

impl Encode for HttpRequest {
    fn encode(self, b: &mut Cursor<Vec<u8>>) -> Result<&mut Cursor<Vec<u8>>, io::Error> {
        b.encode_string(&*self.uri)?
            .encode_string(&*self.method)?
            .encode_i64(self.content_length)?
            .encode_string(&*self.protocol)?
            .encode_string(&*self.ip)?
            .encode_bytes(&self.body)?;

        b.encode_map(self.headers.len(), Kind::String, Kind::Any)?;
        for (k, v) in self.headers {
            b.encode_string(&*k)?;
            v.encode(b)?;
        }
        Ok(b)
    }

    fn internal_error(self, b: &mut Cursor<Vec<u8>>, error: &str) {
        b.encode_error(error).unwrap();
    }
}

impl Decode for HttpRequest {
    fn decode(b: &mut Cursor<&mut Vec<u8>>) -> Result<Option<HttpRequest>, DecodingError> {
        if b.decode_none() {
            return Ok(None);
        }

        fn headers_decode(
            b: &mut Cursor<&mut Vec<u8>>,
        ) -> Result<Option<HashMap<String, HttpStringList>>, DecodingError> {
            if b.decode_none() {
                return Ok(None);
            }

            let size = b
                .decode_map(Kind::String, Kind::Any)
                .ok()
                .ok_or(DecodingError::InvalidU32)?;
            let mut map = HashMap::new();
            for _ in 0..size {
                let k = b.decode_string()?;
                let v = HttpStringList::decode(b)?.ok_or(DecodingError::InvalidMap)?;
                map.insert(k, v);
            }
            Ok(Some(map))
        }
        Ok(Some(HttpRequest {
            uri: b.decode_string()?,
            method: b.decode_string()?,
            content_length: b.decode_i64()?,
            protocol: b.decode_string()?,
            ip: b.decode_string()?,
            body: b.decode_bytes()?,
            headers: headers_decode(b)?.ok_or(DecodingError::InvalidMap)?,
        }))
    }
}

pub struct HttpResponse {
    pub headers: HashMap<String, HttpStringList>,
    pub status_code: i32,
    pub body: Vec<u8>,
}

impl Encode for HttpResponse {
    fn encode(self, b: &mut Cursor<Vec<u8>>) -> Result<&mut Cursor<Vec<u8>>, io::Error> {
        b.encode_i32(self.status_code)?.encode_bytes(&self.body)?;

        b.encode_map(self.headers.len(), Kind::String, Kind::Any)?;
        for (k, v) in self.headers {
            b.encode_string(&*k)?;
            v.encode(b)?;
        }
        Ok(b)
    }

    fn internal_error(self, b: &mut Cursor<Vec<u8>>, error: &str) {
        b.encode_error(error).unwrap();
    }
}

impl Decode for HttpResponse {
    fn decode(b: &mut Cursor<&mut Vec<u8>>) -> Result<Option<HttpResponse>, DecodingError> {
        if b.decode_none() {
            return Ok(None);
        }

        fn headers_decode(
            b: &mut Cursor<&mut Vec<u8>>,
        ) -> Result<Option<HashMap<String, HttpStringList>>, DecodingError> {
            if b.decode_none() {
                return Ok(None);
            }

            let size = b
                .decode_map(Kind::String, Kind::Any)
                .ok()
                .ok_or(DecodingError::InvalidU32)?;
            let mut map = HashMap::new();
            for _ in 0..size {
                let k = b.decode_string()?;
                let v = HttpStringList::decode(b)?.ok_or(DecodingError::InvalidMap)?;
                map.insert(k, v);
            }
            Ok(Some(map))
        }
        Ok(Some(HttpResponse {
            status_code: b.decode_i32()?,
            body: b.decode_bytes()?,
            headers: headers_decode(b)?.ok_or(DecodingError::InvalidMap)?,
        }))
    }
}

pub struct HttpStringList {
    pub value: Vec<String>,
}

impl Encode for HttpStringList {
    fn encode(self, b: &mut Cursor<Vec<u8>>) -> Result<&mut Cursor<Vec<u8>>, io::Error> {
        b.encode_array(self.value.len(), Kind::String)?;
        for item in self.value {
            b.encode_string(&*item)?;
        }
        Ok(b)
    }

    fn internal_error(self, b: &mut Cursor<Vec<u8>>, error: &str) {
        b.encode_error(error).unwrap();
    }
}

impl Decode for HttpStringList {
    fn decode(b: &mut Cursor<&mut Vec<u8>>) -> Result<Option<HttpStringList>, DecodingError> {
        if b.decode_none() {
            return Ok(None);
        }

        fn value_decode(
            b: &mut Cursor<&mut Vec<u8>>,
        ) -> Result<Option<Vec<String>>, DecodingError> {
            let value_size = b.decode_array(Kind::String)?;
            let mut temp = Vec::with_capacity(value_size);
            for _ in 0..value_size {
                temp.push(b.decode_string()?);
            }
            Ok(Some(temp))
        }
        Ok(Some(HttpStringList {
            value: value_decode(b)?.ok_or(DecodingError::InvalidArray)?,
        }))
    }
}