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

/* eslint no-bitwise: off */

import { Context, StringList } from "./http.signature";

import { Signature, RuntimeContext } from "@loopholelabs/scale-signature";

import { Kind, encodeError, decodeError } from "@loopholelabs/polyglot-ts";

import { Response as HttpResponse } from "./response";
import { Request as HttpRequest } from "./request";

export function HttpContextFactory(): HttpContext {
  return new HttpContext();
}

const EmptyBytes = new Uint8Array();

export class HttpContext implements Signature, RuntimeContext {
  private generated: Context;

  constructor() {
    // Create an empty context
    const body = EmptyBytes;
    const headers = new Map<string, StringList>();
    const req = new HttpRequest(
      "",
      BigInt(body.length),
      "",
      "",
      body,
      headers
    );
    const respBody = EmptyBytes;
    const respHeaders = new Map<string, StringList>();
    const resp = new HttpResponse(0, respBody, respHeaders);

    this.generated = new Context(req, resp);
  }

  Request(): HttpRequest {
    return this.generated.Request;
  }

  Response(): HttpResponse {
    return this.generated.Response;
  }

  Generated(): Context {
    return this.generated;
  }

  RuntimeContext(): RuntimeContext {
    return this;
  }

  Read(d: Uint8Array) {
    if (d.length > 0 && d[0] === Kind.Error) {
      const e = decodeError(d).value;
      throw (e);
    }
    this.generated = Context.decode(d).value;
  }

  Write(): Uint8Array {
    if (this.generated === undefined) throw (new Error("generated undefined"));
    return this.generated.encode(new Uint8Array());
  }

  Error(e: Error): Uint8Array {
    return encodeError(new Uint8Array(), e);
  }

  // Helper just to show the context
  private static stringHeaders(h: Map<string, StringList>): string {
    let r = "";
    for (let k of h.keys()) {
      let values = h.get(k);
      if (values != undefined) {
        for (let i of values.Value.values()) {
          r = r + " " + k + "=" + i;
        }
      } else {
        r = r + " " + k;
      }
    }
    return r;
  }

  public show() {
      const req = this.generated.Request;
      const reqBody = new TextDecoder().decode(req.Body);
      console.log(`== Context ==
    Request method=${req.Method}, proto=${req.Protocol}, ip=${req.IP}, len=${req.ContentLength}
    Headers: ${HttpContext.stringHeaders(req.Headers)}
    Body: ${reqBody}`);

      const resp = this.generated.Response;
      const respBody = new TextDecoder().decode(resp.Body);
      console.log(`Response code=${resp.StatusCode}
    Headers: ${HttpContext.stringHeaders(resp.Headers)}
    Body: ${respBody}`);
  }
}