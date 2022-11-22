### 接口名称
更新用户信息

### 接口路径
POST /api/user/update

### 请求参数

#### 请求头

参数名称      | 类型   | 出现要求 | 描述
:-------------|:-------|:-------|:----------------
Authorization | string | 必须     | Token xxxxxx
Content-Type  | string | 非必须   | application/json

#### 请求体

参数名称 | 类型   | 出现要求 | 描述
:--------|:-------|:-------|:----
id       | string | 必须     | 用户id
username | string | 非必须   | 用户名
email    | string | 非必须   | 用户名
role     | string | 非必须   | 角色

### 响应参数

#### 响应体

参数名称      | 类型 | 出现要求 | 描述
:-------------|:-----|:-------|:-----
modified_count | int  | 必须     | 更新条目数

### 响应码说明

响应码 | 说明
:------|:----------------
40011  | 未登录
400    | uid不能为空, uid不存在
401    | 该email已经注册