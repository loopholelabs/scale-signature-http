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

use polyglot_rs::{Decoder, DecodingError, Encoder, Kind};
use std::collections::HashMap;
use std::io::Cursor;

pub trait Encode {
    fn encode<'a>(&'a self, b: &'a mut Cursor<Vec<u8>>) -> Result<&mut Cursor<Vec<u8>>, Box<dyn std::error::Error>>;
    fn internal_error<'a>(&'a self, b: &'a mut Cursor<Vec<u8>>, error: Box<dyn std::error::Error>);
}

pub trait Decode {
    fn decode(b: &mut Cursor<&mut Vec<u8>>) -> Result<Option<Self>, Box<dyn std::error::Error>>
    where
        Self: Sized;
}

#[derive(Clone)]
pub struct HttpContext {
    pub(crate) request: HttpRequest,
    pub(crate) response: HttpResponse,
}

impl Encode for HttpContext {
    fn encode<'a>(&'a self, b: &'a mut Cursor<Vec<u8>>) -> Result<&mut Cursor<Vec<u8>>, Box<dyn std::error::Error>> {
        self.request.encode(b)?;
        self.response.encode(b)?;
        Ok(b)
    }

    fn internal_error<'a>(&'a self, b: &'a mut Cursor<Vec<u8>>, error: Box<dyn std::error::Error>) {
        b.encode_error(error).unwrap();
    }
}

impl Decode for HttpContext {
    fn decode(
        b: &mut Cursor<&mut Vec<u8>>,
    ) -> Result<Option<HttpContext>, Box<dyn std::error::Error>> {
        if b.decode_none() {
            return Ok(None);
        }

        if let Ok(error) = b.decode_error() {
            return Err(error);
        }

        Ok(Some(HttpContext {
            request: HttpRequest::decode(b)?.ok_or(DecodingError::InvalidStruct)?,
            response: HttpResponse::decode(b)?.ok_or(DecodingError::InvalidStruct)?,
        }))
    }
}

#[derive(Clone)]
pub struct HttpRequest {
    pub(crate) headers: HashMap<String, HttpStringList>,
    pub(crate) uri: String,
    pub(crate) method: String,
    pub(crate) content_length: i64,
    pub(crate) protocol: String,
    pub(crate) ip: String,
    pub(crate) body: Vec<u8>,
}

impl Encode for HttpRequest {
    fn encode<'a>(&'a self, b: &'a mut Cursor<Vec<u8>>) -> Result<&mut Cursor<Vec<u8>>, Box<dyn std::error::Error>> {
        b.encode_string(&*self.uri)?
            .encode_string(&*self.method)?
            .encode_i64(self.content_length)?
            .encode_string(&*self.protocol)?
            .encode_string(&*self.ip)?
            .encode_bytes(&self.body)?;

        b.encode_map(self.headers.len(), Kind::String, Kind::Any)?;
        for (k, v) in &self.headers {
            b.encode_string(&*k)?;
            v.encode(b)?;
        }
        Ok(b)
    }

    fn internal_error<'a>(&'a self, b: &'a mut Cursor<Vec<u8>>, error: Box<dyn std::error::Error>) {
        b.encode_error(error).unwrap();
    }
}

impl Decode for HttpRequest {
    fn decode(
        b: &mut Cursor<&mut Vec<u8>>,
    ) -> Result<Option<HttpRequest>, Box<dyn std::error::Error>> {
        if b.decode_none() {
            return Ok(None);
        }

        if let Ok(error) = b.decode_error() {
            return Err(error);
        }

        fn headers_decode(
            b: &mut Cursor<&mut Vec<u8>>,
        ) -> Result<Option<HashMap<String, HttpStringList>>, Box<dyn std::error::Error>> {
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

#[derive(Clone)]
pub struct HttpResponse {
    pub(crate) headers: HashMap<String, HttpStringList>,
    pub(crate) status_code: i32,
    pub(crate) body: Vec<u8>,
}

impl Encode for HttpResponse {
    fn encode<'a>(&'a self, b: &'a mut Cursor<Vec<u8>>) -> Result<&mut Cursor<Vec<u8>>, Box<dyn std::error::Error>> {
        b.encode_i32(self.status_code)?.encode_bytes(&self.body)?;

        b.encode_map(self.headers.len(), Kind::String, Kind::Any)?;
        for (k, v) in &self.headers {
            b.encode_string(&*k)?;
            v.encode(b)?;
        }
        Ok(b)
    }

    fn internal_error<'a>(&'a self, b: &'a mut Cursor<Vec<u8>>, error: Box<dyn std::error::Error>) {
        b.encode_error(error).unwrap();
    }
}

impl Decode for HttpResponse {
    fn decode(
        b: &mut Cursor<&mut Vec<u8>>,
    ) -> Result<Option<HttpResponse>, Box<dyn std::error::Error>> {
        if b.decode_none() {
            return Ok(None);
        }

        if let Ok(error) = b.decode_error() {
            return Err(error);
        }

        fn headers_decode(
            b: &mut Cursor<&mut Vec<u8>>,
        ) -> Result<Option<HashMap<String, HttpStringList>>, Box<dyn std::error::Error>> {
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

#[derive(Clone)]
pub struct HttpStringList {
    pub value: Vec<String>,
}

impl Encode for HttpStringList {
    fn encode<'a>(&'a self, b: &'a mut Cursor<Vec<u8>>) -> Result<&mut Cursor<Vec<u8>>, Box<dyn std::error::Error>> {
        b.encode_array(self.value.len(), Kind::String)?;
        for item in &self.value {
            b.encode_string(&*item)?;
        }
        Ok(b)
    }

    fn internal_error<'a>(&'a self, b: &'a mut Cursor<Vec<u8>>, error: Box<dyn std::error::Error>) {
        b.encode_error(error).unwrap();
    }
}

impl Decode for HttpStringList {
    fn decode(
        b: &mut Cursor<&mut Vec<u8>>,
    ) -> Result<Option<HttpStringList>, Box<dyn std::error::Error>> {
        if b.decode_none() {
            return Ok(None);
        }

        if let Ok(error) = b.decode_error() {
            return Err(error);
        }

        fn value_decode(
            b: &mut Cursor<&mut Vec<u8>>,
        ) -> Result<Option<Vec<String>>, Box<dyn std::error::Error>> {
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
