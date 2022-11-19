### 接口名称
获取项目成员列表

### 接口路径
GET /api/project/get_member_list

### 请求参数

#### 请求头

参数名称      | 类型   | 出现要求 | 描述
:-------------|:-------|:-------|:------------
Authorization | string | 必须     | Token xxxxxx

#### 查询参数

参数名称 | 类型 | 出现要求 | 描述
:--------|:-----|:-------|:----
id       | int  | 必须     | 分组id

### 响应参数

#### 响应体

参数名称           | 类型   | 出现要求 | 描述
:------------------|:-------|:-------|:--------------------
[]                 | Array  | 必须     | 列表
&emsp;id          | int    | 必须     | id
&emsp;uid          | int    | 必须     | 用户id
&emsp;username     | string | 必须     | 用户名
&emsp;email        | string | 必须     | 邮箱
&emsp;email_notice | bool   | 必须     | 邮件消息通知
&emsp;role         | string | 必须     | 角色 owner, dev, guest

### 响应码说明

响应码 | 说明
:------|:---
40011  | 未登录
