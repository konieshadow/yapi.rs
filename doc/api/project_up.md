### 接口名称
更新项目

### 接口路径
POST /api/project/up

### 请求参数

#### 请求头

参数名称      | 类型   | 出现要求 | 描述
:-------------|:-------|:-------|:----------------
Authorization | string | 必须     | Token xxxxxx
Content-Type  | string | 非必须   | application/json

#### 请求体

参数名称      | 类型   | 出现要求 | 描述
:-------------|:-------|:-------|:--------------
id            | int    | 必须     | 项目id
name          | string | 非必须   | 项目名称
group_id      | int    | 非必须   | 分组id
basepath      | string | 非必须   | 接口基本路径
desc          | string | 非必须   | 项目描述
is_json5      | bool   | 非必须   | 是否开启json5
switch_notice | bool   | 非必须   | 是否开启消息通知
project_type  | string | 非必须   | public、private

### 响应参数

#### 响应体

参数名称       | 类型 | 出现要求 | 描述
:--------------|:-----|:-------|:-----
modified_count | int  | 必须     | 更新条目数

### 响应码说明

响应码 | 说明
:------|:--------------
40011  | 未登录
401    | basepath格式有误
405    | 没有权限