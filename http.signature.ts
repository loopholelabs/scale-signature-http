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

import {
  encodeString,
  decodeString,
  Kind,
  encodeArray,
  decodeArray,
  encodeError,
  encodeInt64, encodeUint8Array, encodeMap, decodeInt64, decodeUint8Array, decodeMap, encodeInt32, decodeInt32
} from "@loopholelabs/polyglot-ts";

export class HttpContext {
  constructor(Request: HttpRequest, Response: HttpResponse) {
    this._Request = Request
    this._Response = Response
  }

  private _Request: HttpRequest;

  get Request(): HttpRequest {
    return this._Request
  }

  set Request(Request: HttpRequest) {
    this._Request = Request
  }

  private _Response: HttpResponse;

  get Response(): HttpResponse {
    return this._Response
  }

  set Response(Response: HttpResponse) {
    this._Response = Response
  }

  encode(buf: Uint8Array): Uint8Array {
    let encoded = buf
    encoded = this._Request.encode(encoded)
    encoded = this._Response.encode(encoded)
    return encoded
  }

  internalError(buf: Uint8Array, err: Error): Uint8Array {
    return encodeError(buf, err)
  }

  static decode(buf: Uint8Array): {
    buf: Uint8Array,
    value: HttpContext
  } {
    let decoded = buf
    const Req = HttpRequest.decode(decoded)
    decoded = Req.buf
    const Resp = HttpResponse.decode(decoded)
    decoded = Resp.buf
    return {
      buf: decoded,
      value: new HttpContext(Req.value, Resp.value)
    }
  }
}

export class HttpStringList {
  constructor(Value: string[]) {
    this._Value = Value
  }

  private _Value: string[];

  get Value(): string[] {
    return this._Value
  }

  set Value(Value: string[]) {
    this._Value = Value
  }

  encode(buf: Uint8Array): Uint8Array {
    let encoded = buf
    encoded = encodeArray(encoded, this._Value.length, Kind.String)
    this._Value.forEach(field => {
      encoded = encodeString(encoded, field);
    })
    return encoded
  }

  static decode(buf: Uint8Array): {
    buf: Uint8Array,
    value: HttpStringList
  } {
    let decoded = buf
    const ValueArray = decodeArray(decoded)
    decoded = ValueArray.buf
    const Value: { value: string[] } = {
      value: [],
    }

    for (let i = 0; i < ValueArray.size; i++) {
      const element = decodeString(decoded);
      decoded = element.buf;
      Value.value.push(element.value);
    }

    return {
      buf: decoded,
      value: new HttpStringList(Value.value)
    }
  }
}

export class HttpRequest {
  constructor(Method: string, URI: string, ContentLength: bigint, Protocol: string, IP: string, Body: Uint8Array, Headers: Map<string, HttpStringList>) {
    this._Method = Method
    this._URI = URI
    this._ContentLength = ContentLength
    this._Protocol = Protocol
    this._IP = IP
    this._Body = Body
    this._Headers = Headers
  }

  private _Method: string;

  get Method(): string {
    return this._Method
  }

  set Method(Method: string) {
    this._Method = Method
  }

  private _ContentLength: bigint;

  get ContentLength(): bigint {
    return this._ContentLength
  }

  set ContentLength(ContentLength: bigint) {
    this._ContentLength = ContentLength
  }

  private _Protocol: string;

  get Protocol(): string {
    return this._Protocol
  }

  set Protocol(Protocol: string) {
    this._Protocol = Protocol
  }

  private _IP: string;

  get IP(): string {
    return this._IP
  }

  set IP(IP: string) {
    this._IP = IP
  }

  private _Body: Uint8Array;

  get Body(): Uint8Array {
    return this._Body
  }

  set Body(Body: Uint8Array) {
    this._Body = Body
  }

  private _Headers: Map<string, HttpStringList>;

  get Headers(): Map<string, HttpStringList> {
    return this._Headers
  }

  set Headers(Headers: Map<string, HttpStringList>) {
    this._Headers = Headers
  }

  private _URI: string;

  get URI(): string {
      return this._URI
  }

  set URI(URI: string) {
      this._URI = URI
  }

  encode(buf: Uint8Array): Uint8Array {
    let encoded = buf
    encoded = encodeString(encoded, this._URI)
    encoded = encodeString(encoded, this._Method)
    encoded = encodeInt64(encoded, this._ContentLength)
    encoded = encodeString(encoded, this._Protocol)
    encoded = encodeString(encoded, this._IP)
    encoded = encodeUint8Array(encoded, this._Body)
    encoded = encodeMap(encoded, this._Headers.size,
        Kind.String, Kind.Any)
    this._Headers.forEach((value, key) => {
      encoded = encodeString(encoded, key);
      encoded = value.encode(encoded);
    })
    return encoded
  }

  static decode(buf: Uint8Array): {
    buf: Uint8Array,
    value: HttpRequest
  } {
    let decoded = buf
    const URI = decodeString(decoded)
    decoded = URI.buf
    const Method = decodeString(decoded)
    decoded = Method.buf
    const ContentLength = decodeInt64(decoded)
    decoded = ContentLength.buf
    const Protocol = decodeString(decoded)
    decoded = Protocol.buf
    const IP = decodeString(decoded)
    decoded = IP.buf
    const Body = decodeUint8Array(decoded)
    decoded = Body.buf
    const HeadersMap = decodeMap(decoded)
    decoded = HeadersMap.buf
    const Headers: { value: Map<string, HttpStringList> } = {
      value: new Map<string, HttpStringList>(),
    }

    for (let i = 0; i < HeadersMap.size; i++) {
      const key = decodeString(decoded);
      decoded = key.buf;
      const value = HttpStringList.decode(decoded);
      decoded = value.buf;
      Headers.value.set(key.value, value.value);
    }

    return {
      buf: decoded,
      value: new HttpRequest(Method.value, URI.value, ContentLength.value, Protocol.value, IP.value, Body.value, Headers.value)
    }
  }
}

export class HttpResponse {
  constructor(StatusCode: number, Body: Uint8Array, Headers: Map<string, HttpStringList>) {
    this._StatusCode = StatusCode
    this._Body = Body
    this._Headers = Headers
  }

  private _StatusCode: number;

  get StatusCode(): number {
    return this._StatusCode
  }

  set StatusCode(StatusCode: number) {
    this._StatusCode = StatusCode
  }

  private _Body: Uint8Array;

  get Body(): Uint8Array {
    return this._Body
  }

  set Body(Body: Uint8Array) {
    this._Body = Body
  }

  private _Headers: Map<string, HttpStringList>;

  get Headers(): Map<string, HttpStringList> {
    return this._Headers
  }

  set Headers(Headers: Map<string, HttpStringList>) {
    this._Headers = Headers
  }

  encode(buf: Uint8Array): Uint8Array {
    let encoded = buf
    encoded = encodeInt32(encoded, this._StatusCode)
    encoded = encodeUint8Array(encoded, this._Body)
    encoded = encodeMap(encoded, this._Headers.size,
        Kind.String, Kind.Any)
    this._Headers.forEach((value, key) => {
      encoded = encodeString(encoded, key);
      encoded = value.encode(encoded);
    })
    return encoded
  }

  static decode(buf: Uint8Array): {
    buf: Uint8Array,
    value: HttpResponse
  } {
    let decoded = buf
    const StatusCode = decodeInt32(decoded)
    decoded = StatusCode.buf
    const Body = decodeUint8Array(decoded)
    decoded = Body.buf
    const HeadersMap = decodeMap(decoded)
    decoded = HeadersMap.buf
    const Headers: { value: Map<string, HttpStringList> } = {
      value: new Map<string, HttpStringList>(),
    }

    for (let i = 0; i < HeadersMap.size; i++) {
      const key = decodeString(decoded);
      decoded = key.buf;
      const value = HttpStringList.decode(decoded);
      decoded = value.buf;
      Headers.value.set(key.value, value.value);
    }

    return {
      buf: decoded,
      value: new HttpResponse(StatusCode.value, Body.value, Headers.value)
    }
  }
}