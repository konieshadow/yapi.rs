### 接口名称
更新接口

### 接口路径
POST /api/interface/add

### 请求参数

#### 请求头

参数名称      | 类型   | 出现要求 | 描述
:-------------|:-------|:-------|:----------------
Authorization | string | 必须     | Token xxxxxx
Content-Type  | string | 非必须   | application/json

#### 请求体

参数名称   | 类型   | 出现要求 | 描述
:----------|:-------|:-------|:----
cat_id     | int    | 必须     | 分类id
title      | string | 必须     | 接口名称
method     | string | 必须     | 请求方法
path       | string | 必须     | 请求路径

### 响应参数

#### 响应体

参数名称                | 类型   | 出现要求 | 描述
:-----------------------|:-------|:-------|:---------------------------------------
id                      | int    | 必须     | 接口id
uid                     | int    | 必须     | 创建者id
cat_id                  | int    | 必须     | 分类id
project_id              | int    | 必须     | 项目id
title                   | string | 必须     | 接口名称
method                  | string | 必须     | 请求方法
path                    | string | 必须     | 请求路径
status                  | string | 必须     | 完成状态 undone, done
api_opened              | bool   | 必须     | 是否是开放接口
type                    | string | 必须     | 类型 static, var
desc                    | string | 必须     | 描述html
markdown                | string | 必须     | 描述markdown
req_header[]            | Array  | 必须     | 请求头列表
&emsp;name              | string | 必须     | 请求头名称
&emsp;value             | string | 必须     | 请求头值
&emsp;required          | string | 必须     | 是否必须 0, 1
&emsp;example           | string | 必须     | 示例
&emsp;desc              | string | 必须     | 描述
req_query[]             | Array  | 必须     | 请求查询参数列表
&emsp;id                | int    | 必须     | 请求参数id
&emsp;name              | string | 必须     | 请求参数名称
&emsp;desc              | string | 必须     | 请求参数描述
&emsp;example           | string | 必须     | 请求参数示例
&emsp;required          | string | 必须     | 是否必须 0, 1
req_body_type           | string | 必须     | 请求体类型 form, json, text, file, raw
req_body_is_json_schema | bool   | 必须     | 请求体是否是json_schema
req_body_form[]         | Array  | 必须     | 请求表单列表
&emsp;id                | int    | 必须     | 请求表单id
&emsp;name              | string | 必须     | 请求表单名称
&emsp;type              | string | 必须     | 请求表单类型 text, file
&emsp;desc              | string | 必须     | 请求表单描述
&emsp;required          | string | 必须     | 是否必须 0, 1
req_body_other          | string | 必须     | 其他请求体
res_body_type           | string | 必须     | 响应体类型 json, text, xml, raw, json-schema
res_body_is_json_schema | bool   | 必须     | 响应体是否是json_schema
res_body                | string | 必须     | 响应体
add_time                | int    | 必须     | 创建时间戳
up_time                 | int    | 必须     | 更新时间戳

### 响应码说明

响应码 | 说明
:------|:-----
40011  | 未登录
401    | 分类不存在
405    | 没有权限