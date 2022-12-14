// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { ReqBodyForm } from "./ReqBodyForm";
import type { ReqHeader } from "./ReqHeader";
import type { ReqParam } from "./ReqParam";
import type { ReqQuery } from "./ReqQuery";

export interface InterfaceDetail { id: number, uid: number, cat_id: number, project_id: number, title: string, method: string, path: string, status: string, api_opened: boolean, desc: string, markdown: string, req_params: Array<ReqParam>, req_header: Array<ReqHeader>, req_query: Array<ReqQuery>, req_body_type: string, req_body_is_json_schema: boolean, req_body_form: Array<ReqBodyForm>, req_body_other: string, res_body_type: string, res_body_is_json_schema: boolean, res_body: string, add_time: number, up_time: number, }