### 接口名称
添加项目

### 接口路径
POST /api/project/add

### 请求参数

#### 请求头

参数名称      | 类型   | 出现要求 | 描述
:-------------|:-------|:-------|:------------
Authorization | string | 必须     | Token xxxxxx

#### 请求体

参数名称     | 类型   | 出现要求 | 描述
:------------|:-------|:-------|:--------------
name         | string | 必须     | 项目名称
group_id     | int    | 必须     | 分组id
basepath     | string | 必须     | 基础路径
desc         | string | 必须     | 描述
project_type | string | 必须     | public、private

### 响应参数

#### 响应体

参数名称          | 类型   | 出现要求 | 描述
:-----------------|:-------|:-------|:--------------
id                | int    | 必须     | 项目id
uid               | int    | 必须     | 创建者id
name              | string | 必须     | 项目名称
basepath          | string | 必须     | 接口基本路径
switch_notice     | bool   | 必须     | 是否开启消息通知
desc              | string | 必须     | 描述
group_id          | int    | 必须     | 分组id
project_type      | string | 必须     | public、private
color             | string | 必须     | 图标颜色
icon              | string | 必须     | 图标
is_json5          | bool   | 必须     | 是否开启json5
is_mock_open      | bool   | 必须     | 是否开启mock
env[]             | Array  | 必须     | 环境列表
&emsp;id          | int    | 必须     | 环境id
&emsp;name        | string | 必须     | 环境名称
&emsp;domain      | string | 必须     | 环境域名
&emsp;header[]    | Array  | 必须     | Header列表
&emsp;&emsp;name  | string | 必须     | 名称
&emsp;&emsp;value | string | 必须     | 值
&emsp;global[]    | Array  | 必须     | 全局变量列表
&emsp;&emsp;name  | string | 必须     | 名称
&emsp;&emsp;value | string | 必须     | 值
add_time          | int    | 必须     | 创建时间戳
up_time           | int    | 必须     | 更新时间戳

### 响应码说明

响应码 | 说明
:------|:---------------------
40011  | 未登录
401    | 已存在的项目名, basepath格式有误
405    | 没有权限
