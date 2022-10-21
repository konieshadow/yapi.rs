### 接口名称
添加分组成员

### 接口路径
POST /api/group/add_member

### 请求参数

#### 请求头

参数名称 | 类型   | 出现要求 | 描述
:--------|:-------|:-------|:---------------
Cookie   | string | 必须     | _yapi_token=xxx

#### 请求体

参数名称     | 类型   | 出现要求 | 描述
:------------|:-------|:-------|:--------------------
id           | string | 必须     | 项目id
member_uids  | Array  | 必须     | 成员id列表
&emsp;_item_ | int    | 必须     | 用户id
role         | string | 必须     | 角色 owner, dev, guest

### 响应参数

#### 响应体

参数名称        | 类型   | 出现要求 | 描述
:---------------|:-------|:-------|:--------------------
add_members[]   | Array  | 必须     | 添加的成员列表
&emsp;uid       | int    | 必须     | 用户id
&emsp;username  | string | 必须     | 用户名
&emsp;email     | string | 必须     | 邮箱
&emsp;role      | string | 必须     | 角色 owner, dev, guest
exist_members[] | Array  | 必须     | 已存在的成员列表
&emsp;uid       | int    | 必须     | 用户id
&emsp;username  | string | 必须     | 用户名
&emsp;email     | string | 必须     | 邮箱
&emsp;role      | string | 必须     | 角色 owner, dev, guest
no_members[]    | Array  | 必须     | 不存在的成员列表
&emsp;_item_    | int    | 必须     | 用户id
result          | Object | 必须     | 结果
&emsp;n         | int    | 必须     | 匹配条目数
&emsp;nModified | int    | 必须     | 更新条目数
&emsp;ok        | int    | 必须     | 0, 1

### 响应码说明

响应码 | 说明
:------|:---
40011  | 未登录