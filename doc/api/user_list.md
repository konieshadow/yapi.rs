### 接口名称
获取用户列表

### 接口路径
GET /api/user/list

### 请求参数

#### 请求头

参数名称      | 类型   | 出现要求 | 描述
:-------------|:-------|:-------|:------------
Authorization | string | 必须     | Token xxxxxx

#### 查询参数

参数名称 | 类型 | 出现要求 | 描述
:--------|:-----|:------|:----
page     | int  | 非必须   | 当前页数
limit    | int  | 非必须   | 每页数量

### 响应参数

#### 响应体

参数名称       | 类型   | 出现要求 | 描述
:--------------|:-------|:-------|:-------------------------------
count          | int    | 必须     | 总数量
total          | int    | 必须     | 总页数
list[]         | Array  | 必须     | 列表
&emsp;_id      | int    | 必须     | 接口id
&emsp;username | string | 必须     | 用户名
&emsp;email    | string | 必须     | 邮箱
&emsp;role     | string | 必须     | 角色
&emsp;type     | string | 必须     | site用户是网站注册用户, third是第三方登录过来的用户
&emsp;study    | bool   | 必须     | 是否已过引导
&emsp;add_time | int    | 必须     | 创建时间戳
&emsp;up_time  | int    | 必须     | 更新时间戳

### 响应码说明

响应码 | 说明
:------|:---------
40011  | 未登录