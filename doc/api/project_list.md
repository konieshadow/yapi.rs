### 接口名称
获取项目列表

### 接口路径
GET /api/project/list

### 请求参数

#### 请求头

参数名称      | 类型   | 出现要求 | 描述
:-------------|:-------|:-------|:------------
Authorization | string | 必须     | Token xxxxxx

#### 查询参数

参数名称 | 类型 | 出现要求 | 描述
:--------|:-----|:-------|:----
group_id | int  | 必须     | 分组id
page     | int  | 非必须   | 当前页数
limit    | int  | 非必须   | 每页数量

### 响应参数

#### 响应体

参数名称                | 类型   | 出现要求 | 描述
:-----------------------|:-------|:-------|:--------------
list[]                  | Array  | 必须     | 列表
&emsp;id                | int    | 必须     | 项目id
&emsp;uid               | int    | 必须     | 创建者id
&emsp;name              | string | 必须     | 项目名称
&emsp;basepath          | string | 必须     | 接口基本路径
&emsp;switch_notice     | bool   | 必须     | 是否开启消息通知
&emsp;desc              | string | 必须     | 描述
&emsp;group_id          | int    | 必须     | 分组id
&emsp;project_type      | string | 必须     | public、private
&emsp;color             | string | 必须     | 图标颜色
&emsp;icon              | string | 必须     | 图标
&emsp;add_time          | int    | 必须     | 创建时间戳
&emsp;up_time           | int    | 必须     | 更新时间戳

### 响应码说明

响应码 | 说明
:------|:---
40011  | 未登录
