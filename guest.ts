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

import {GuestContext as GuestContextInterface} from "@loopholelabs/scale-signature";
import {encodeError} from "@loopholelabs/polyglot-ts";

import {Request} from "./request";
import {Response} from "./response";

import {HttpContext} from "./http.signature";

// TODO: These maybe should move to scale-signature
const SCALE_NEXT: string = "scale_fn_next";
const SCALE_ADDRESS_OF: string = "getaddr";

let writeBuffer: ArrayBuffer = new Uint8Array().buffer;
let readBuffer: ArrayBuffer = new Uint8Array().buffer;

export class GuestContext implements GuestContextInterface {
  private _context: HttpContext;

  constructor(ctx: HttpContext) {
    this._context = ctx;
  }

// ToWriteBuffer serializes the Context into the global writeBuffer and returns the pointer to the buffer and its size
//
// This method should only be used to read the Context from the Scale Runtime.
// Users should not use this method.
  public ToWriteBuffer(): number[] {
    writeBuffer = this._context.encode(new Uint8Array()).buffer;
    let addrof = (global as any)[SCALE_ADDRESS_OF];
    let ptr = addrof(writeBuffer);
    let len = writeBuffer.byteLength;
    return [ptr, len];
  }

// FromReadBuffer deserializes the data into the Context from the global readBuffer
//
// It assumes that the readBuffer has been filled with the data from the Scale Runtime after
// a call to the Resize method
  public FromReadBuffer(): Error | undefined {
    let ret = HttpContext.decode(new Uint8Array(readBuffer));
    this._context = ret.value;
    return undefined;
  }

// ErrorWriteBuffer serializes an error into the global writeBuffer and returns a pointer to the buffer and its size
//
// This method should only be used to write an error to the Scale Runtime, in place of the ToWriteBuffer method.
// Users should not use this method.
  public ErrorWriteBuffer(err: Error): number[] {
    writeBuffer = encodeError(new Uint8Array(), err).buffer;
    let addrof = (global as any)[SCALE_ADDRESS_OF];
    let ptr = addrof(writeBuffer);
    let len = writeBuffer.byteLength;
    return [ptr, len];
  }

// Next calls the next host function after writing the Context into the global writeBuffer,
// then it reads the result from the global readBuffer back into the Context
  public Next(): GuestContext {
    // context -> bytes
    let [ptr, len] = this.ToWriteBuffer();

    // Call next()
    let nextfn = (global as any)[SCALE_NEXT];
    nextfn([ptr, len]);

    this.FromReadBuffer();
    return this;
  }

// Request returns the Request object for the Context
  get Request(): Request {
    return new Request(this._context.Request);
  }

// Response returns the Response object for the Context
  get Response(): Response {
    return new Response(this._context.Response);
  }

}

export function Resize(size: number): number {
    readBuffer = new Uint8Array(size).buffer;
    let addrof = (global as any)[SCALE_ADDRESS_OF];
    return addrof(readBuffer);
}
