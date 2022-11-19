### 接口名称
获取分类下接口列表

### 接口路径
GET /api/interface/list_cat

### 请求参数

#### 请求头

参数名称      | 类型   | 出现要求 | 描述
:-------------|:-------|:-------|:------------
Authorization | string | 必须     | Token xxxxxx

#### 查询参数

参数名称 | 类型 | 出现要求 | 描述
:--------|:-----|:-------|:----
catid    | int  | 必须     | 分类id
page     | int  | 非必须   | 当前页数
limit    | int  | 非必须   | 每页数量

### 响应参数

#### 响应体

参数名称         | 类型   | 出现要求 | 描述
:----------------|:-------|:-------|:----------------
count            | int    | 必须     | 总数量
total            | int    | 必须     | 总页数
list[]           | Array  | 必须     | 列表
&emsp;id        | int    | 必须     | 接口id
&emsp;uid        | int    | 必须     | 创建者id
&emsp;edit_uid   | int    | 必须     | 编辑者id
&emsp;catid      | int    | 必须     | 分类id
&emsp;project_id | int    | 必须     | 项目id
&emsp;title      | string | 必须     | 接口名称
&emsp;method     | string | 必须     | 请求方法
&emsp;path       | string | 必须     | 请求路径
&emsp;status     | string | 必须     | 完成状态 undone，done
&emsp;api_opened | bool   | 必须     | 是否是开放接口
&emsp;add_time   | int    | 必须     | 创建时间戳
&emsp;up_time    | int    | 必须     | 更新时间戳

### 响应码说明

响应码 | 说明
:------|:---------
40011  | 未登录
400    | catid不能为空
406    | 没有权限