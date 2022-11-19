### 接口名称
获取接口菜单列表

### 接口路径
GET /api/interface/list_menu

### 请求参数

#### 请求头

参数名称      | 类型   | 出现要求 | 描述
:-------------|:-------|:-------|:------------
Authorization | string | 必须     | Token xxxxxx

#### 查询参数

参数名称   | 类型 | 出现要求 | 描述
:----------|:-----|:-------|:----
project_id | int  | 必须     | 项目id

### 响应参数

#### 响应体

参数名称               | 类型   | 出现要求 | 描述
:----------------------|:-------|:-------|:----------------
[]                     | Array  | 必须     | 列表
&emsp;id              | int    | 必须     | 分类id
&emsp;uid              | int    | 必须     | 创建者id
&emsp;project_id       | int    | 必须     | 项目id
&emsp;name             | string | 必须     | 分类名称
&emsp;list[]           | Array  | 必须     | 接口列表
&emsp;&emsp;id         | int    | 必须     | 接口id
&emsp;&emsp;uid        | int    | 必须     | 创建者id
&emsp;&emsp;edit_uid   | int    | 必须     | 编辑者id
&emsp;&emsp;catid      | int    | 必须     | 分类id
&emsp;&emsp;project_id | int    | 必须     | 项目id
&emsp;&emsp;title      | string | 必须     | 接口名称
&emsp;&emsp;method     | string | 必须     | 请求方法
&emsp;&emsp;path       | string | 必须     | 请求路径
&emsp;&emsp;status     | string | 必须     | 完成状态 undone，done
&emsp;&emsp;add_time   | int    | 必须     | 创建时间戳
&emsp;&emsp;up_time    | int    | 必须     | 更新时间戳
&emsp;add_time         | int    | 必须     | 创建时间戳
&emsp;up_time          | int    | 必须     | 更新时间戳

### 响应码说明

响应码 | 说明
:------|:---
40011  | 未登录
