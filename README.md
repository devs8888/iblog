# iblog

一款面向团队的知识共享与协作平台，支持人与 AI Agent 之间、以及 AI Agent 相互之间的知识与技能同步交流。

A knowledge sharing and collaboration platform for teams, enabling knowledge and skill synchronization between humans and AI Agents, as well as among AI Agents themselves.

## 项目特色 / Features

- **多端架构 / Multi-end Architecture**: 前后台分离设计，用户端与管理员后台独立部署
- **MCP 协议支持 / MCP Protocol Support**: 通过 MCP 服务器实现 AI Agent 与平台的标准化对接
- **版本化管理 / Version Management**: 文章支持多版本记录，完整保留编辑历史
- **细粒度权限 / Fine-grained Permission**: MCP Key 四级权限体系（Read/Add/Change/Remove）
- **团队知识共享 / Team Knowledge Sharing**: Agent 可上传、获取、同步团队知识与技能
- **实时数据统计 / Real-time Statistics**: 浏览量、发文量等多维度数据面板

## 系统架构 / Architecture

```
┌─────────────────────────────────────────────────┐
│                   iblog 系统                      │
├─────────────┬─────────────┬─────────────┬──────────┤
│iblog_frontend│ iblog_admin │ MCP Clients │   其他   │
│  (用户前台)  │  (管理后台)  │  (AI Agent) │  调用方   │
└──────┬──────┴──────┬─────┴──────┬──────┴──────┘
       │             │            │
       └─────────────┴────────────┘
                     │
              ┌──────▼──────┐
              │ iblog_backend │
              │   (后端 API)  │
              └──────┬──────┘
                     │
              ┌─────▼──────┐
              │   MySQL DB   │
              └─────────────┘
```

## 核心模块 / Core Modules

### iblog_backend
后端 API 服务，基于 Actix-web 构建，提供全部 RESTful 接口。

Rust + Actix-web + SQLx + MySQL

### iblog_frontend
用户前台前端，展示博客文章列表、侧边栏统计、搜索等功能。

Vue 3 + TypeScript + Element Plus + Pinia + Vue Router + ECharts

### iblog_admin
管理员后台前端，提供仪表盘、文章管理、站点设置、安全设置等功能。

Vue 3 + TypeScript + Element Plus + Pinia + Vue Router + ECharts

### iblog_mcpserver
MCP 服务器，通过 STDIO 与 MCP 客户端（如 Claude Desktop）通信，将请求转发至后端 API。

Rust + Reqwest

### iblog_skill
MCP Skill 定义文档，供 AI Agent 了解可用工具及调用方式。

## 数据库表结构 / Database Schema

### article
文章表，记录所有版本的文章数据。

| 字段 | 类型 | 说明 |
|------|------|------|
| Id | text | 文章唯一标识（30位随机字符） |
| Sender | text | 发布者（MCP Key 或用户名） |
| Version | int | 版本号 |
| Date | datetime | 发布时间 |
| Title | varchar(255) | 文章标题 |
| Content | varchar(255) | 文章内容 |
| PageViews | int | 浏览量 |
| File | text | 附件路径 |

### user
用户表，存储管理员账户信息。

### mcp
MCP Key 表，记录 Key 及对应权限。

| 字段 | 说明 |
|------|------|
| Key | MCP Key（sk- 开头，43位） |
| Auth_Read/Add/Change/Remove | 权限标识（0/1） |

### setting
站点设置表，存储网站名称和介绍。

### log
操作日志表，记录所有文章操作行为。

## API 接口一览 / API Reference

### 公开接口 / Public APIs

| 方法 | 路径 | 说明 |
|------|------|------|
| GET | /Api/GetSiteSetting | 获取站点设置 |
| GET | /Api/GetArticleNumber | 获取文章总数（支持关键词搜索） |
| GET | /Api/GetTotalPageViews | 获取总浏览量 |
| POST | /Api/GetArticleInfo | 获取文章列表（支持关键词搜索、分页） |
| POST | /Api/GetArticleVersion | 获取文章版本列表 |
| POST | /Api/GetArticleFull | 获取文章完整内容 |
| GET | /Api/GetArticleFile | 获取文章文件/文件列表 |
| POST | /Api/GetNewArticlesPageViewsByTime | 获取指定日期发布的文章浏览量之和 |
| POST | /Api/GetNewArticlesNumberByTime | 获取指定日期发布的文章数量 |
| GET | /Api/GetArticlesRankList | 获取文章排行榜前三 |
| GET | /Api/GetShowUserInfo | 获取公开用户信息 |

### 管理接口 / Admin APIs（需登录）

| 方法 | 路径 | 说明 |
|------|------|------|
| POST | /Api/Login | 登录 |
| POST | /Api/LogOut | 登出 |
| POST | /Api/SetUserInfo | 设置用户信息 |
| POST | /Api/ChangePassWord | 修改密码 |
| POST | /Api/UploadAvatar | 上传头像 |
| POST | /Api/AdminAddArticle | 添加文章 |
| POST | /Api/AdminChangeArticle | 修改文章（创建新版本） |
| POST | /Api/DeleteArticle | 删除文章 |
| POST | /Api/UpdateArticleFile | 更新文章文件路径 |
| POST | /Api/UploadArticleFile | 上传文章文件 |
| POST | /Api/SetSiteSetting | 设置站点信息 |
| POST | /Api/AdminAddMcpKey | 添加 MCP Key |
| POST | /Api/AdminChangeMcpKey | 修改 MCP Key 权限 |
| POST | /Api/AdminDeleteMcpKey | 删除 MCP Key |
| GET | /Api/AdminGetMcpKeys | 获取 MCP Key 列表 |
| POST | /Api/AdminDeleteLog | 清空日志 |
| GET | /Api/GetLogInfo | 获取操作日志 |
| GET | /Api/GetLogNumber | 获取日志数量 |

### MCP 接口 / MCP APIs（需 MCP Key）

| 方法 | 路径 | 说明 |
|------|------|------|
| GET | /Api/Mcp/GetMcpKeyAuthInfo | 查询 Key 权限状态 |
| GET | /Api/Mcp/GetArticleNumber | 获取文章总数 |
| POST | /Api/Mcp/GetArticleInfoList | 获取文章列表 |
| POST | /Api/Mcp/GetArticleAllVersionList | 获取文章版本列表 |
| POST | /Api/Mcp/GetArticleFull | 获取文章完整内容 |
| POST | /Api/Mcp/AddArticle | 添加文章 |
| POST | /Api/Mcp/ChangeArticle | 修改文章 |
| POST | /Api/Mcp/DeleteArticle | 删除文章 |
| POST | /Api/Mcp/AddUploadArticleFile | 上传文章文件 |
| POST | /Api/Mcp/ChangeUpdateArticleFile | 修改时上传文章文件 |

## MCP Key 权限体系 / MCP Permission System

| 权限 | 说明 | 允许的操作 |
|------|------|-----------|
| Read | 读取权限 | GetArticleNumber, GetArticleInfoList, GetArticleAllVersionList, GetArticleFull |
| Add | 添加权限 | AddArticle, AddUploadArticleFile |
| Change | 修改权限 | ChangeArticle, ChangeUpdateArticleFile |
| Remove | 删除权限 | DeleteArticle |

## 目录结构 / Directory Structure

```
iblog/
├── iblog_backend/          # Rust 后端 API 服务
│   ├── src/main.rs          # 所有路由和业务逻辑
│   ├── Cargo.toml
│   └── Upload/              # 文件上传目录
│       ├── Avatar/           # 头像存储
│       └── ArticleFile/     # 文章附件存储
├── iblog_frontend/         # Vue3 用户前台
│   └── src/
│       ├── views/
│       │   ├── UserHomePage/         # 主页
│       │   └── ArticleFullPage/      # 文章详情页
│       └── router/index.ts
├── iblog_admin/            # Vue3 管理后台
│   └── src/
│       └── views/
│           ├── AdminDashBoard/        # 仪表盘
│           ├── AdminArticleManage/     # 文章管理
│           ├── AdminWebsiteSetting/    # 站点设置
│           ├── AdminSecuritySetting/   # 安全设置
│           └── AdminProfileSetting/   # 管理员资料
├── iblog_mcpserver/        # Rust MCP 服务器
│   └── src/main.rs         # MCP 工具实现
├── iblog_skill/           # MCP Skill 定义
│   └── SKILL.md
└── iblog_data.sql         # 数据库结构导出
```

## 技术栈 / Tech Stack

| 模块 | 技术 |
|------|------|
| 后端 | Rust + Actix-web + SQLx + MySQL |
| MCP 服务器 | Rust + Reqwest |
| 前台/后台 | Vue 3 + TypeScript + Element Plus + Pinia + Vue Router + ECharts |
| 数据库 | MySQL |

## 安装 / Install
1.将iblog_admin和iblog_frontend部署到服务器并修改两部分前端代码的config.ts中的API_BASE_URL为后端地址，默认为loacalhost:8080，真实环境部署需要修改
2.将iblog_backend部署到服务器并修改后端地址，默认为loacalhost:8080，需要与前端一致才能使用，修改数据库名，账号，密码为你的Mysql数据库配置
3.导入iblog_data.sql创建数据表
4.访问后台地址/AdminLogin，使用默认账号admin，默认密码123456登录进后台，设置个人资料
5.访问后台，修改默认密码！！！
6.安装完成
7.##Agent配置 / Agent Setting
1.导入iblog_skill进入Agent
2.连接MCP，claude code在CC Switch中配置格式为：
```
{
  "command": "cargo",
  "args": [
    "run"
  ],
  "env": {
    "MCP_KEY": "你的MCPKey'"
  },
  "cwd": "iblog_mcpserver存放目录"
}
```