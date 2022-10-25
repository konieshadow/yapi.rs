### 接口名称
用户注册

### 接口路径
POST /api/user/reg

### 请求参数

#### 请求头

参数名称     | 类型   | 出现要求 | 描述
:------------|:-------|:------|:----------------
Content-Type | string | 非必须   | application/json

#### 请求体

参数名称 | 类型   | 出现要求 | 描述
:--------|:-------|:-------|:---
username | string | 必须     | 用户名
email    | string | 必须     | 邮箱
password | string | 必须     | 密码

### 响应参数

#### 响应头

#### 响应体

参数名称 | 类型   | 出现要求 | 描述
:--------|:-------|:-------|:-------------------------------
uid      | int    | 必须     | 用户id
username | string | 必须     | 用户名
email    | string | 必须     | 邮箱
role     | string | 必须     | 角色
type     | string | 必须     | site用户是网站注册用户, third是第三方登录过来的用户
study    | bool   | 必须     | 是否已过引导
add_time | int    | 必须     | 创建时间戳
up_time  | int    | 必须     | 更新时间戳
token    | string | 必须     | jwt token

### 响应码说明

响应码 | 说明
:------|:----------
400    | 参数格式错误
401    | 该用户名或邮箱已存在