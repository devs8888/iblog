# iblog_mcpserver

## 概述 / Overview

`iblog_mcpserver` 是 iblog 系统的 MCP (Model Context Protocol) 服务器组件，通过标准输入输出（STDIO）与 MCP 客户端通信。它作为 iblog 后端 API 的 MCP 接口层，使 AI Agent 能够执行文章管理操作，实现知识与技能的共享同步。

`iblog_mcpserver` is the MCP (Model Context Protocol) server component of the iblog system. It communicates with MCP clients via standard input/output (STDIO). Acting as the MCP interface layer of the iblog backend API, it enables AI Agents to perform article management operations and achieve knowledge and skill sharing synchronization.

## 能力 / Capabilities

- **MCP Key 验证 / Key Authentication**: 通过 `X-MCP-Key` 请求头验证 MCP Key 有效性及权限
- **文章查询 / Article Query**: 获取文章列表、数量、版本信息、全文内容
- **文章管理 / Article Management**: 添加、修改、删除文章
- **文件上传 / File Upload**: 为文章上传附件文件
- **权限控制 / Permission Control**: 独立的 Read/Add/Change/Remove 四级权限体系

## 工作流程 / Workflow

1. MCP 客户端通过 STDIN 发送 JSON-RPC 请求
2. 服务器解析请求并调用对应工具函数
3. 函数通过 HTTP 请求访问 iblog 后端 API
4. 后端响应后，MCP 服务器将结果通过 STDOUT 返回 JSON-RPC 响应

## 可用工具 / Available Tools

### CheckKeyAndServer
- **功能**: 验证 MCP Key 和服务器可用性 / Verify MCP Key and server availability
- **参数**: 无 / None
- **返回**: Key 状态、服务器状态、四种权限状态 / Key status, server status, four permission states

### GetArticleNumber
- **功能**: 获取文章总数（支持关键词搜索）/ Get total article count (with optional keyword search)
- **参数**: `KeyWord?: string`（可选，用于模糊匹配标题或内容 / optional, for fuzzy matching in title or content）
- **返回**: 文章总数 / Total article number

### GetArticleInfoList
- **功能**: 获取文章列表（最新版本，仅含基本信息）/ Get article list (latest version, basic info only)
- **参数**: `Number?: number`（默认10）, `KeyWord?: string`（可选 / optional）
- **返回**: 文章列表，含 ID、标题、内容摘要、版本、文件标识 / Article list with ID, title, content summary, version, file flag

### GetArticleVersionList
- **功能**: 获取指定文章的所有版本 / Get all versions of a specific article
- **参数**: `Id: string`（必填 / required）
- **返回**: 版本列表，含版本号、标题、内容摘要、文件标识 / Version list with version number, title, content summary, file flag

### GetArticleFull
- **功能**: 获取指定文章的完整内容 / Get full content of a specific article version
- **参数**: `Id: string`, `Version: number`（均必填 / both required）
- **返回**: 完整标题、内容、文件下载链接列表 / Full title, content, file download link list

### DeleteArticle
- **功能**: 删除指定文章版本（需 Remove 权限）/ Delete specific article version (requires Remove permission)
- **参数**: `Id: string`, `Version: number`（均必填 / both required）
- **返回**: 操作成功状态 / Operation success status

### AddArticle
- **功能**: 添加新文章（需 Add 权限）/ Add new article (requires Add permission)
- **参数**: `Title: string`, `Content: string`, `FileList?: string[]`（标题和内容必填，文件列表可选 / title and content required, file list optional）
- **返回**: 操作状态及新文章 ID / Operation status and new article ID

### ChangeArticle
- **功能**: 修改文章（创建新版本，需 Change 权限）/ Modify article (creates new version, requires Change permission)
- **参数**: `Id: string`, `Title: string`, `Content: string`, `FileList?: string[]`
- **返回**: 操作状态及新版本号 / Operation status and new version number

## MCP Key 权限说明 / MCP Key Permission Reference

| 权限 | 描述 | 影响的操作 |
|------|------|-----------|
| Read | 读取文章信息 | GetArticleNumber, GetArticleInfoList, GetArticleVersionList, GetArticleFull |
| Add | 添加文章 | AddArticle, AddUploadArticleFile |
| Change | 修改文章 | ChangeArticle, ChangeUpdateArticleFile |
| Remove | 删除文章 | DeleteArticle |

## 错误处理 / Error Handling

- **Key 无效或缺失**: 返回 `"Key Status: Invalid"`
- **权限不足**: 返回对应操作的权限拒绝信息 / Returns permission denial for the specific operation
- **服务器请求失败**: 返回 `"Server request failed: {具体错误}"`
- **解析响应失败**: 返回 `"Failed to parse response: {具体错误}"`

## 典型使用场景 / Typical Usage Scenarios

### 1. 团队知识共享 / Team Knowledge Sharing
```
用户指令: "将我的 Python 爬虫经验文章上传到服务器"
Agent 操作: AddArticle(Title="Python爬虫经验总结", Content="...")
```

### 2. 跨智能体技能同步 / Cross-Agent Skill Synchronization
```
用户指令: "获取团队其他成员分享的机器学习文章"
Agent 操作: GetArticleInfoList(Number=20, KeyWord="机器学习")
```

### 3. 文档版本管理 / Document Version Management
```
用户指令: "查看这篇文档的所有历史版本"
Agent 操作: GetArticleVersionList(Id="abc123...")
```

## 环境变量 / Environment Variables

| 变量名 | 描述 | 必填 |
|--------|------|------|
| `MCP_KEY` | MCP Key，用于认证 | 是 / Yes |
| `DATABASE_URL` | 数据库连接 URL（仅后端使用） | 后端必填 / Backend only |

## 安全注意事项 / Security Notes

- MCP Key 存储在环境变量中，不要硬编码或提交到代码仓库
- MCP Key 是团队的共享凭证，泄漏后应立即在后台管理删除
- 所有操作均通过 MCP Key 的权限体系控制访问
- MCP Key should be stored in environment variables, never hardcoded or committed to repository
- MCP Key is a shared credential for the team; revoke immediately via admin panel if compromised
- All operations are access-controlled through the MCP Key permission system

## 限制 / Limitations

- 最大文件上传：2MB
- 文章内容截断显示：列表接口仅显示前5个字符

- Maximum file upload: 2MB
- Content truncation: List interfaces show only first 5 characters
