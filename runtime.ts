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

const EmptyBytes = new Uint8Array();

export function New(): Context {
    return new Context();
}

export class Context implements Signature {
    private readonly generated = new HttpContext(new HttpRequest("", "", BigInt(0), "", "", EmptyBytes, new Map<string, StringList>()), new HttpResponse(0, EmptyBytes, new Map<string, StringList>()));
    private readonly runtimeContext = new RuntimeContext(this.generated);

   constructor() {}

   Generated(): HttpContext {
     return this.generated;
   }

   RuntimeContext(): RuntimeContext {
     return this.runtimeContext;
   }
}

export class RuntimeContext implements RuntimeContextInterface {
    private readonly context: HttpContext;

    constructor(generated: HttpContext) {
        this.context = generated;
    }

    Read(b: Uint8Array): Error | undefined {
        if (b.length > 0 && b[0] === Kind.Error) {
            return decodeError(b).value;
        }
        Object.assign(this.context, HttpContext.decode(b).value);
        return undefined;
    }

    Write(): Uint8Array {
        return this.context.encode(new Uint8Array());
    }

    Error(err: Error): Uint8Array {
        return this.context.internalError(new Uint8Array(), err);
    }
}