### 接口名称
删除分组成员

### 接口路径
POST /api/group/del_member

### 请求参数

#### 请求头

参数名称      | 类型   | 出现要求 | 描述
:-------------|:-------|:-------|:------------
Authorization | string | 必须     | Token xxxxxx
Content-Type  | string | 非必须   | application/json

#### 请求体

参数名称   | 类型   | 出现要求 | 描述
:----------|:-------|:-------|:----
id         | string | 必须     | 分组名称
member_uid | int    | 必须     | 成员id

### 响应参数

#### 响应体

参数名称     | 类型 | 出现要求 | 描述
:------------|:-----|:-------|:-----
deleted_count | int  | 必须     | 删除条目数

### 响应码说明

响应码 | 说明
:------|:-------
40011  | 未登录
400    | 分组成员不存在
405    | 没有权限