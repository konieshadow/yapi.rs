### 接口名称
获取分组列表

### 接口路径
GET /api/group/list

### 请求参数

#### 请求头

参数名称 | 类型   | 出现要求 | 描述
:--------|:-------|:-------|:---------------
Cookie   | string | 必须     | _yapi_token=xxx

### 响应参数

#### 响应体

参数名称         | 类型   | 出现要求 | 描述
:----------------|:-------|:-------|:--------------
[]               | Array  | 必须     | 列表
&emsp;_id        | int    | 必须     | 分组id
&emsp;group_name | string | 必须     | 分组名称
&emsp;role       | string | 必须     | 角色
&emsp;type       | string | 必须     | public、private
&emsp;add_time   | int    | 必须     | 创建时间戳
&emsp;up_time    | int    | 必须     | 更新时间戳

### 响应码说明

响应码 | 说明
:------|:---
40011  | 未登录
