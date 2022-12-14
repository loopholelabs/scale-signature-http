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

// Code generated by protoc-gen-ts-polyglot 0.2.0. DO NOT EDIT.
import { encodeString, decodeString, Kind, encodeArray, decodeArray } from "@loopholelabs/polyglot-ts";

import { Request } from "./request";
import { Response } from "./response";

export class Context {
  constructor(Request: Request, Response: Response) {
    this._Request = Request
    this._Response = Response
  }

  private _Request: Request;

  get Request(): Request {
    return this._Request
  }

  set Request(Request: Request) {
    this._Request = Request
  }

  private _Response: Response;

  get Response(): Response {
    return this._Response
  }

  set Response(Response: Response) {
    this._Response = Response
  }

  encode(buf: Uint8Array): Uint8Array {
    let encoded = buf
    encoded = this._Request.encode(encoded)
    encoded = this._Response.encode(encoded)
    return encoded
  }

  static decode(buf: Uint8Array): {
    buf: Uint8Array,
    value: Context
  } {
    let decoded = buf
    const Req = Request.decode(decoded)
    decoded = Req.buf
    const Resp = Response.decode(decoded)
    decoded = Resp.buf
    return {
      buf: decoded,
      value: new Context(Req.value, Resp.value)
    }
  }
}

export class StringList {
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
    value: StringList
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
      value: new StringList(Value.value)
    }
  }
}
