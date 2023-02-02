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

import {HttpResponse as HttpContextResponse, StringList} from "./http.signature";

export class Response {
  private response: HttpContextResponse;
  private textEncoder = new TextEncoder();
  constructor(res : HttpContextResponse) {
    this.response = res;
  }

  public StatusCode() : number {
    return this.response.StatusCode;
  }

  public SetStatusCode(statusCode : number) {
    this.response.StatusCode = statusCode;
  }

  public Body() : Uint8Array {
    return this.response.Body;
  }

  public SetBody(body : string) {
    this.SetBodyBytes(this.textEncoder.encode(body));
  }

  public SetBodyBytes(body : Uint8Array) {
    this.response.Body = body;
  }

  public Headers() : Map<string, StringList> {
    return this.response.Headers;
  }

  public SetHeader(key : string, value : StringList) {
    this.response.Headers.set(key, value);
  }

  public GetHeader(key : string) : StringList | undefined {
    return this.response.Headers.get(key);
  }
}