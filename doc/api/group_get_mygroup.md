### 接口名称
获取分组

### 接口路径
GET /api/group/get_mygroup

### 请求参数

#### 请求头

参数名称      | 类型   | 出现要求 | 描述
:-------------|:-------|:-------|:------------
Authorization | string | 必须     | Token xxxxxx

### 响应参数

#### 响应体

参数名称   | 类型   | 出现要求 | 描述
:----------|:-------|:-------|:--------------
id         | int    | 必须     | 分组id
uid        | int    | 必须     | 创建者id
group_name | string | 必须     | 分组名称
role       | string | 必须     | 角色
type       | string | 必须     | public、private
add_time   | int    | 必须     | 创建时间戳
up_time    | int    | 必须     | 更新时间戳

### 响应码说明

响应码 | 说明
:------|:-----
40011  | 未登录
401    | 分组不存在