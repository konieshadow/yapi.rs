### 接口名称
更新接口

### 接口路径
POST /api/interface/up

### 请求参数

#### 请求头

参数名称      | 类型   | 出现要求 | 描述
:-------------|:-------|:-------|:------------
Authorization | string | 必须     | Token xxxxxx
Content-Type  | string | 非必须   | application/json

#### 请求体

参数名称                | 类型   | 出现要求 | 描述
:-----------------------|:-------|:-------|:---------------------------------------
id                      | int    | 必须     | 接口id
name                    | string | 非必须   | 接口名称
catid                   | int    | 非必须   | 分类id
method                  | string | 非必须   | 请求方法
path                    | string | 非必须   | 请求路径
tag                     | Array  | 非必须   | 标签列表
&emsp;_item_            | string | 必须     | 标签
status                  | string | 非必须   | 完成状态 undone, done
desc                    | string | 非必须   | 描述html
markdown                | string | 非必须   | 描述markdown
req_header[]            | Array  | 必须     | 请求头列表
&emsp;id               | int    | 必须     | 请求头id
&emsp;name              | string | 必须     | 请求头名称
&emsp;value             | string | 必须     | 请求头值
&emsp;required          | string | 必须     | 是否必须 0, 1
req_query[]             | Array  | 非必须   | 请求查询参数列表
&emsp;id               | int    | 必须     | 请求参数id
&emsp;name              | string | 必须     | 请求参数名称
&emsp;desc              | string | 必须     | 请求参数描述
&emsp;example           | string | 必须     | 请求参数示例
&emsp;required          | string | 必须     | 是否必须 0, 1
req_body_type           | string | 非必须   | 请求体类型 form, json, text, file, raw
req_body_is_json_schema | bool   | 非必须   | 请求体是否是json_schema
req_body_form[]         | Array  | 非必须   | 请求表单列表
&emsp;id               | int    | 必须     | 请求表单id
&emsp;name              | string | 必须     | 请求表单名称
&emsp;type              | string | 必须     | 请求表单类型 text, file
&emsp;desc              | string | 必须     | 请求表单描述
&emsp;required          | string | 必须     | 是否必须 0, 1
req_body_other          | string | 非必须   | 其他请求体
res_body_type           | string | 非必须   | 响应体类型 json, text, xml, raw, json-schema
res_body_is_json_schema | bool   | 非必须   | 响应体是否是json_schema
res_body                | string | 非必须   | 响应体

### 响应参数

#### 响应体

参数名称      | 类型 | 出现要求 | 描述
:-------------|:-----|:-------|:-----
modified_count | int  | 必须     | 更新条目数
### 响应码说明

响应码 | 说明
:------|:------------
40011  | 未登录
400    | 不存在的接口, 没有权限