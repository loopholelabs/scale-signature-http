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

import { Request } from "./request";
import { Response } from "./response";

import { HttpContext, HttpRequest, HttpResponse } from "./http.signature";

// TODO: This should move to scale-signature
const SCALE_NEXT: string = "scale_fn_next";

export class GuestContext implements GuestContextInterface {
  private _context: HttpContext;

  constructor(ctx: HttpContext) {
    this._context = ctx;
  }

  // NB This isn't needed for the js guest.
  public ToWriteBuffer(): number[] {
    return [0, 0];
  }

  // NB This isn't needed for the js guest
  public FromReadBuffer(): Error | undefined {
    return undefined;
  }

  // NB This isn't needed for the js guest
  public ErrorWriteBuffer(err: Error): number[] {
    return [0, 0];
  }

  // Return the context
  public Context(): HttpContext {
    return this._context;
  }

  // Chain to the next scale function
  public Next(): HttpContext {
    // context -> bytes
    let buf = this._context.encode(new Uint8Array());
    let data = Array.from(buf);

    // Call next()
    let nextfn = (global as any)[SCALE_NEXT];
    let newdata = nextfn(data);

    // bytes -> context
    const oContext = HttpContext.decode(Uint8Array.from(newdata)).value;
    return oContext;
  }

  get Request(): Request {
    return new Request(this._context.Request);
  }

  get Response(): Response {
    return new Response(this._context.Response);
  }
}