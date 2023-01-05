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

(window as any).crypto = { getRandomValues: require('polyfill-crypto.getrandomvalues') }


import { TextEncoder, TextDecoder } from "util";
import * as fs from "fs";
import { Context as HContext, StringList } from "./http.signature";
import { Request } from "./request";
import { Response } from "./response";

import { ScaleFunc, V1Alpha, Go } from "@loopholelabs/scalefile";
import { HttpContextFactory } from "./runtime";

import { GetRuntime } from "@loopholelabs/scale-ts";

window.TextEncoder = TextEncoder;
window.TextDecoder = TextDecoder as typeof window["TextDecoder"];

describe("runtime", () => {
  it("Can run a simple e2e one module", async () => {

    const modHttpEndpoint = fs.readFileSync(
      "./example_modules/http-endpoint.wasm"
    );

    const scalefnEndpoint = new ScaleFunc(V1Alpha, "Test.HttpEndpoint", "ExampleName@ExampleVersion", Go, [], modHttpEndpoint);

    const signatureFactory = HttpContextFactory;

    const r = await GetRuntime(signatureFactory, [scalefnEndpoint]);

    const i = await r.Instance(null);

    const enc = new TextEncoder();
    const body = enc.encode("Hello world this is a request body");
    i.Context().Request().Method = "GET";
    i.Context().Request().Body = body;
    i.Context().Request().ContentLength = BigInt(body.length);
    i.Context().Request().Headers.set("content", new StringList(["hello"]));

    i.Run();

    const resp = i.Context().Response();

    expect(resp).not.toBeNull();

    if (resp != null) {
      // check the returns...

      const dec = new TextDecoder();
      const bodyText = dec.decode(resp.Body);

      // The http-endpoint.wasm module copies the request body to the response body.
      expect(bodyText).toBe("Hello world this is a request body");
    }
  });

  it("Can run a simple e2e using runtime", async () => {

    // Now we can use context with a couple of wasm modules...

    const modHttpEndpoint = fs.readFileSync(
      "./example_modules/http-endpoint.wasm"
    );
    const modHttpMiddleware = fs.readFileSync(
      "./example_modules/http-middleware.wasm"
    );

    const scalefnEndpoint = new ScaleFunc(V1Alpha, "Test.HttpEndpoint", "ExampleName@ExampleVersion", Go, [], modHttpEndpoint);
    const scalefnMiddle = new ScaleFunc(V1Alpha, "Test.HttpMiddleware", "ExampleName@ExampleVersion", Go, [], modHttpMiddleware);

    const signatureFactory = HttpContextFactory;

    const r = await GetRuntime(signatureFactory, [scalefnMiddle, scalefnEndpoint]);

    const i = await r.Instance(null);

    const enc = new TextEncoder();
    const body = enc.encode("Hello world this is a request body");
    i.Context().Request().Method = "GET";
    i.Context().Request().Body = body;
    i.Context().Request().ContentLength = BigInt(body.length);
    i.Context().Request().Headers.set("content", new StringList(["hello"]));

    i.Run();

    const resp = i.Context().Response();

    expect(resp).not.toBeNull();

    if (resp != null) {
      // check the returns...

      const dec = new TextDecoder();
      const bodyText = dec.decode(resp.Body);

      // The http-endpoint.wasm module copies the request body to the response body.
      expect(bodyText).toBe("Hello world this is a request body");

      // The http-middleware.wasm adds a header
      const middle = resp.Headers.get("MIDDLEWARE");
      expect(middle).toBeDefined();
      const vals = middle?.Value;
      if (vals !== undefined) {
        expect(vals.length).toBe(1);
        expect(vals[0]).toBe("TRUE");
      }
    }
  });
});
