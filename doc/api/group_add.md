### 接口名称
添加分组

### 接口路径
POST /api/group/add

### 请求参数

#### 请求头

参数名称      | 类型   | 出现要求 | 描述
:-------------|:-------|:-------|:------------
Authorization | string | 必须     | Token xxxxxx

#### 请求体

参数名称     | 类型   | 出现要求 | 描述
:------------|:-------|:-------|:---------
group_name   | string | 必须     | 分组名称
group_desc   | string | 必须     | 分组描述
owner_uids   | Array  | 必须     | 分组拥有者id列表
&emsp;_item_ | int    | 必须     | 用户id

### 响应参数

#### 响应体

参数名称       | 类型   | 出现要求 | 描述
:--------------|:-------|:-------|:--------------
id             | int    | 必须     | 分组id
uid            | int    | 必须     | 创建者id
group_name     | string | 必须     | 分组名称
group_desc     | string | 必须     | 分组描述
type           | string | 必须     | public、private
member[]       | Array  | 必须     | 成员列表
&emsp;id       | int    | 必须     | 成员id
&emsp;username | string | 必须     | 用户名
&emsp;email    | string | 必须     | 邮箱
&emsp;role     | string | 必须     | 角色

### 响应码说明

响应码 | 说明
:------|:--------
40011  | 未登录
401    | 项目分组名已存在
402    | 所选成员不存在