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


import {HttpRequest as HttpContextRequest, HttpStringList} from "./http.signature";

export class Request {
  private request: HttpContextRequest;
  private textEncoder = new TextEncoder();
  constructor(req : HttpContextRequest) {
    this.request = req;
  }

  public Method() : string {
    return this.request.Method;
  }

  public SetMethod(method : string) {
    this.request.Method = method;
  }

  public URI() : string {
    return this.request.URI;
  }

  public SetURI(uri : string) {
    this.request.URI = uri;
  }

  public Body() : Uint8Array {
    return this.request.Body;
  }

  public SetBody(body : string) {
    this.SetBodyBytes(this.textEncoder.encode(body));
  }

  public SetBodyBytes(body : Uint8Array) {
    this.request.Body = body;
    this.request.ContentLength = BigInt(body.length);
  }

  public ContentLength() : bigint {
    return this.request.ContentLength;
  }

  public RemoteIP() : string {
    return this.request.IP;
  }

  public Protocol() : string {
    return this.request.Protocol;
  }

  public Headers() : Map<string, HttpStringList> {
    return this.request.Headers;
  }

  public SetHeader(key : string, value : HttpStringList) {
    this.request.Headers.set(key, value);
  }

  public GetHeader(key : string) : HttpStringList | undefined {
    return this.request.Headers.get(key);
  }
}