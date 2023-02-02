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

import {HttpContext, HttpRequest, HttpResponse, StringList} from "./http.signature";

import {RuntimeContext as RuntimeContextInterface, Signature} from "@loopholelabs/scale-signature";

import {decodeError, Kind} from "@loopholelabs/polyglot-ts";

import {Response} from "./response";
import {Request} from "./request";

const EmptyBytes = new Uint8Array();

export class RuntimeContext implements RuntimeContextInterface {
    private generated: HttpContext;

    constructor(generated: HttpContext) {
        this.generated = generated;
    }

    Read(b: Uint8Array): Error | undefined {
        if (b.length > 0 && b[0] === Kind.Error) {
            return decodeError(b).value;
        }
        this.generated = HttpContext.decode(b).value;
        return undefined;
    }

    Write(): Uint8Array {
        return this.generated.encode(new Uint8Array());
    }

    Error(err: Error): Uint8Array {
        return this.generated.internalError(new Uint8Array(), err);
    }
}

export class Context implements Signature {
  private readonly generated: HttpContext;
  private readonly request: Request;
  private readonly response: Response;
  private readonly runtimeContext: RuntimeContext;

  constructor() {
    const reqBody = EmptyBytes;
    const reqHeaders = new Map<string, StringList>();
    const req = new HttpRequest(
      "",
      "",
      BigInt(reqBody.length),
      "",
      "",
      reqBody,
      reqHeaders
    );
    const respBody = EmptyBytes;
    const respHeaders = new Map<string, StringList>();
    const resp = new HttpResponse(0, respBody, respHeaders);

    this.generated = new HttpContext(req, resp);
    this.request = new Request(this.generated.Request);
    this.response = new Response(this.generated.Response);
    this.runtimeContext = new RuntimeContext(this.generated);
  }

  Request(): Request {
    return this.request;
  }

  Response(): Response {
    return this.response;
  }

  Generated(): HttpContext {
    return this.generated;
  }

  RuntimeContext(): RuntimeContext {
    return this.runtimeContext;
  }
}