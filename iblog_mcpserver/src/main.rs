use reqwest::Client;
use serde::Deserialize;
use serde_json::Value;
use std::io::{self, BufRead, Write};
use std::fs;

const SERVER_URL: &str = "http://127.0.0.1:8080";
const MCP_KEY_HEADER: &str = "X-MCP-Key";

#[derive(Debug, Deserialize)]
struct GetMcpKeyAuthInfoResponse {
    KeyStatus: String,
    ServerStatus: String,
    #[serde(default)]
    ReadAuth: Option<bool>,
    #[serde(default)]
    AddAuth: Option<bool>,
    #[serde(default)]
    RemoveAuth: Option<bool>,
    #[serde(default)]
    ChangeAuth: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct GetArticleNumberResponse {
    Status: String,
    #[serde(default)]
    Number: Option<i64>,
}

#[derive(Debug, Deserialize)]
struct ArticleInfoListItem {
    Id: String,
    Sender: String,
    Title: String,
    Content: String,
    Version: i32,
    File: bool,
}

#[derive(Debug, Deserialize)]
struct GetArticleInfoListResponse {
    Status: String,
    #[serde(default)]
    InfoList: Option<Vec<ArticleInfoListItem>>,
}

#[derive(Debug, Deserialize)]
struct ArticleVersionListItem {
    Version: i32,
    Sender: String,
    Title: String,
    Content: String,
    File: bool,
}

#[derive(Debug, Deserialize)]
struct GetArticleAllVersionListResponse {
    Status: String,
    #[serde(default)]
    VersionList: Option<Vec<ArticleVersionListItem>>,
}

#[derive(Debug, Deserialize)]
struct GetArticleFullResponse {
    Status: String,
    #[serde(default)]
    Title: Option<String>,
    #[serde(default)]
    Content: Option<String>,
    #[serde(default)]
    FileList: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct McpDeleteArticleResponse {
    Status: String,
    Auth: bool,
}

#[derive(Debug, Deserialize)]
struct McpAddArticleResponse {
    Status: String,
    Auth: bool,
    #[serde(default)]
    ArticleId: Option<String>,
}

#[derive(Debug, Deserialize)]
struct McpChangeArticleResponse {
    Status: String,
    Auth: bool,
    #[serde(default)]
    Version: Option<i32>,
}

fn get_auth_info(mcp_key: &str) -> Result<String, String> {
    let client = Client::new();
    let rt = tokio::runtime::Runtime::new().unwrap();

    let response = rt.block_on(async {
        client
            .get(&format!("{}/Api/Mcp/GetMcpKeyAuthInfo", SERVER_URL))
            .header(MCP_KEY_HEADER, mcp_key)
            .send()
            .await
    }).map_err(|e| format!("Server request failed: {}", e))?;

    let data: GetMcpKeyAuthInfoResponse = rt.block_on(async {
        response.json().await
    }).map_err(|e| format!("Failed to parse response: {}", e))?;

    if data.KeyStatus == "False" {
        return Ok(format!("Key Status: Invalid, Server Status: {}", data.ServerStatus));
    }

    Ok(format!(
        "Key Status: Valid, Server Status: {}, Read Auth: {}, Add Auth: {}, Change Auth: {}, Remove Auth: {}",
        data.ServerStatus,
        data.ReadAuth.unwrap_or(false),
        data.AddAuth.unwrap_or(false),
        data.ChangeAuth.unwrap_or(false),
        data.RemoveAuth.unwrap_or(false)
    ))
}

fn get_article_number(mcp_key: &str, keyword: Option<&str>) -> Result<String, String> {
    let client = Client::new();
    let rt = tokio::runtime::Runtime::new().unwrap();

    let response = rt.block_on(async {
        let url = if let Some(kw) = keyword {
            format!("{}/Api/Mcp/GetArticleNumber?KeyWord={}", SERVER_URL, urlencoding::encode(kw))
        } else {
            format!("{}/Api/Mcp/GetArticleNumber", SERVER_URL)
        };
        client.get(&url)
            .header(MCP_KEY_HEADER, mcp_key)
            .send()
            .await
    }).map_err(|e| format!("Server request failed: {}", e))?;

    let data: GetArticleNumberResponse = rt.block_on(async {
        response.json().await
    }).map_err(|e| format!("Failed to parse response: {}", e))?;

    if data.Status == "False" {
        return Ok("Failed to get article number".to_string());
    }

    Ok(format!("Total Article Number: {}", data.Number.unwrap_or(0)))
}

fn get_article_info_list(mcp_key: &str, number: i64, keyword: Option<&str>) -> Result<String, String> {
    let client = Client::new();
    let rt = tokio::runtime::Runtime::new().unwrap();

    let response = rt.block_on(async {
        let req = client.post(&format!("{}/Api/Mcp/GetArticleInfoList", SERVER_URL))
            .header(MCP_KEY_HEADER, mcp_key);
        if let Some(kw) = keyword {
            req.json(&serde_json::json!({ "Number": number, "KeyWord": kw }))
        } else {
            req.json(&serde_json::json!({ "Number": number }))
        }
        .send()
        .await
    }).map_err(|e| format!("Server request failed: {}", e))?;

    let data: GetArticleInfoListResponse = rt.block_on(async {
        response.json().await
    }).map_err(|e| format!("Failed to parse response: {}", e))?;

    if data.Status == "False" {
        return Ok("Failed to get article info list".to_string());
    }

    let info_list = data.InfoList.unwrap_or_default();
    if info_list.is_empty() {
        return Ok("No articles found".to_string());
    }

    let mut result = String::from("Article List:\n");
    for (i, article) in info_list.iter().enumerate() {
        result.push_str(&format!(
            "{}. [v{}] {} - {} ({})\n   Content: {}\n   File: {}\n",
            i + 1,
            article.Version,
            article.Title,
            article.Sender,
            article.Id,
            article.Content,
            if article.File { "Yes" } else { "No" }
        ));
    }

    Ok(result)
}

fn get_article_version_list(mcp_key: &str, id: &str) -> Result<String, String> {
    let client = Client::new();
    let rt = tokio::runtime::Runtime::new().unwrap();

    let response = rt.block_on(async {
        client
            .post(&format!("{}/Api/Mcp/GetArticleAllVersionList", SERVER_URL))
            .header(MCP_KEY_HEADER, mcp_key)
            .json(&serde_json::json!({ "Id": id }))
            .send()
            .await
    }).map_err(|e| format!("Server request failed: {}", e))?;

    let data: GetArticleAllVersionListResponse = rt.block_on(async {
        response.json().await
    }).map_err(|e| format!("Failed to parse response: {}", e))?;

    if data.Status == "False" {
        return Ok("Failed to get article version list".to_string());
    }

    let version_list = data.VersionList.unwrap_or_default();
    if version_list.is_empty() {
        return Ok("No versions found".to_string());
    }

    let mut result = String::from("Article Versions:\n");
    for version in version_list {
        result.push_str(&format!(
            "v{}: {} - {} ({})\n   Content: {}, File: {}\n",
            version.Version,
            version.Title,
            version.Sender,
            id,
            version.Content,
            if version.File { "Yes" } else { "No" }
        ));
    }

    Ok(result)
}

fn get_article_full(mcp_key: &str, id: &str, version: i32) -> Result<String, String> {
    let client = Client::new();
    let rt = tokio::runtime::Runtime::new().unwrap();

    let response = rt.block_on(async {
        client
            .post(&format!("{}/Api/Mcp/GetArticleFull", SERVER_URL))
            .header(MCP_KEY_HEADER, mcp_key)
            .json(&serde_json::json!({ "Id": id, "Version": version }))
            .send()
            .await
    }).map_err(|e| format!("Server request failed: {}", e))?;

    let data: GetArticleFullResponse = rt.block_on(async {
        response.json().await
    }).map_err(|e| format!("Failed to parse response: {}", e))?;

    if data.Status == "False" {
        return Ok("Failed to get article full content".to_string());
    }

    let mut result = format!(
        "Article Title: {}\n\nArticle Content:\n{}\n\n",
        data.Title.unwrap_or_default(),
        data.Content.unwrap_or_default()
    );

    let file_list = data.FileList.unwrap_or_default();
    if !file_list.is_empty() {
        result.push_str("Article Files:\n");
        for file in file_list {
            result.push_str(&format!("  {}\n", file));
        }
    } else {
        result.push_str("Article Files: None\n");
    }

    Ok(result)
}

fn delete_article(mcp_key: &str, id: &str, version: i32) -> Result<String, String> {
    let client = Client::new();
    let rt = tokio::runtime::Runtime::new().unwrap();

    let response = rt.block_on(async {
        client
            .post(&format!("{}/Api/Mcp/DeleteArticle", SERVER_URL))
            .header(MCP_KEY_HEADER, mcp_key)
            .json(&serde_json::json!({ "Id": id, "Version": version }))
            .send()
            .await
    }).map_err(|e| format!("Server request failed: {}", e))?;

    let data: McpDeleteArticleResponse = rt.block_on(async {
        response.json().await
    }).map_err(|e| format!("Failed to parse response: {}", e))?;

    if data.Status == "False" && !data.Auth {
        return Ok("Delete failed: No permission or invalid key".to_string());
    }

    if data.Status == "False" && data.Auth {
        return Ok("Delete failed: Database operation failed".to_string());
    }

    Ok("Article deleted successfully".to_string())
}

fn change_article(mcp_key: &str, id: &str, title: &str, content: &str, file_list: Vec<String>) -> Result<String, String> {
    let client = Client::new();
    let rt = tokio::runtime::Runtime::new().unwrap();

    // Change article first
    let response = rt.block_on(async {
        client
            .post(&format!("{}/Api/Mcp/ChangeArticle", SERVER_URL))
            .header(MCP_KEY_HEADER, mcp_key)
            .json(&serde_json::json!({ "Id": id, "Title": title, "Content": content }))
            .send()
            .await
    }).map_err(|e| format!("Server request failed: {}", e))?;

    let data: McpChangeArticleResponse = rt.block_on(async {
        response.json().await
    }).map_err(|e| format!("Failed to parse response: {}", e))?;

    if data.Status == "False" && !data.Auth {
        return Ok("Change article failed: No permission or invalid key".to_string());
    }

    if data.Status == "False" && data.Auth {
        return Ok("Change article failed: Database operation failed".to_string());
    }

    let version = match data.Version {
        Some(v) => v,
        None => return Ok("Change article succeeded but no version returned".to_string()),
    };

    // Upload files if any
    if !file_list.is_empty() {
        for file_path in &file_list {
            let file_content = match std::fs::read(file_path) {
                Ok(content) => content,
                Err(e) => return Ok(format!("Change article succeeded but failed to read file {}: {}", file_path, e)),
            };

            let file_name = std::path::Path::new(file_path)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("file");

            let part = reqwest::multipart::Part::bytes(file_content)
                .file_name(file_name.to_string());

            let form = reqwest::multipart::Form::new()
                .text("Id", id.to_string())
                .text("Version", version.to_string())
                .part("file", part);

            let upload_response: reqwest::Response = rt.block_on(async {
                client
                    .post(&format!("{}/Api/Mcp/ChangeUpdateArticleFile", SERVER_URL))
                    .header(MCP_KEY_HEADER, mcp_key)
                    .multipart(form)
                    .send()
                    .await
            }).map_err(|e| format!("File upload request failed: {}", e))?;

            #[derive(Deserialize)]
            struct UploadResponse { Status: String, Auth: bool }

            let upload_data: UploadResponse = rt.block_on(async {
                upload_response.json().await
            }).map_err(|e| format!("Failed to parse upload response: {}", e))?;

            if upload_data.Status == "False" && !upload_data.Auth {
                return Ok(format!("Change article succeeded but file upload failed: No permission or invalid key (file: {})", file_name));
            }

            if upload_data.Status == "False" && upload_data.Auth {
                return Ok(format!("Change article succeeded but file upload failed: Database operation failed (file: {})", file_name));
            }
        }
        Ok(format!("Article changed successfully with {} file(s). New Version: {}", file_list.len(), version))
    } else {
        Ok(format!("Article changed successfully. New Version: {}", version))
    }
}

fn add_article(mcp_key: &str, title: &str, content: &str, file_list: Vec<String>) -> Result<String, String> {
    let client = Client::new();
    let rt = tokio::runtime::Runtime::new().unwrap();

    // Add article first
    let response = rt.block_on(async {
        client
            .post(&format!("{}/Api/Mcp/AddArticle", SERVER_URL))
            .header(MCP_KEY_HEADER, mcp_key)
            .json(&serde_json::json!({ "Title": title, "Content": content }))
            .send()
            .await
    }).map_err(|e| format!("Server request failed: {}", e))?;

    let data: McpAddArticleResponse = rt.block_on(async {
        response.json().await
    }).map_err(|e| format!("Failed to parse response: {}", e))?;

    if data.Status == "False" && !data.Auth {
        return Ok("Add article failed: No permission or invalid key".to_string());
    }

    if data.Status == "False" && data.Auth {
        return Ok("Add article failed: Database operation failed".to_string());
    }

    let article_id = match data.ArticleId {
        Some(id) => id,
        None => return Ok("Add article failed: No article ID returned".to_string()),
    };

    // Upload files if any
    if !file_list.is_empty() {
        for file_path in &file_list {
            let file_content = match fs::read(&file_path) {
                Ok(content) => content,
                Err(e) => return Ok(format!("Add article succeeded but failed to read file {}: {}", file_path, e)),
            };

            let file_name = std::path::Path::new(&file_path)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("file");

            let part = reqwest::multipart::Part::bytes(file_content)
                .file_name(file_name.to_string());

            let form = reqwest::multipart::Form::new()
                .text("Id", article_id.clone())
                .text("Version", "1")
                .part("file", part);

            let upload_response: reqwest::Response = rt.block_on(async {
                client
                    .post(&format!("{}/Api/Mcp/AddUploadArticleFile", SERVER_URL))
                    .header(MCP_KEY_HEADER, mcp_key)
                    .multipart(form)
                    .send()
                    .await
            }).map_err(|e| format!("File upload request failed: {}", e))?;

            #[derive(Deserialize)]
            struct UploadResponse { Status: String, Auth: bool }

            let upload_data: UploadResponse = rt.block_on(async {
                upload_response.json().await
            }).map_err(|e| format!("Failed to parse upload response: {}", e))?;

            if upload_data.Status == "False" && !upload_data.Auth {
                return Ok(format!("Add article succeeded but file upload failed: No permission or invalid key (file: {})", file_name));
            }

            if upload_data.Status == "False" && upload_data.Auth {
                return Ok(format!("Add article succeeded but file upload failed: Database operation failed (file: {})", file_name));
            }
        }
        Ok(format!("Article added successfully with {} file(s). Article ID: {}", file_list.len(), article_id))
    } else {
        Ok(format!("Article added successfully. Article ID: {}", article_id))
    }
}

fn handle_tool_call(mcp_key: &str, tool_name: &str, arguments: &Value) -> String {
    match tool_name {
        "CheckKeyAndServer" => get_auth_info(mcp_key).unwrap_or_else(|e| format!("Error: {}", e)),

        "GetArticleNumber" => {
            let keyword = arguments.get("KeyWord")
                .and_then(|v| v.as_str())
                .map(String::from);
            get_article_number(mcp_key, keyword.as_deref()).unwrap_or_else(|e| format!("Error: {}", e))
        }

        "GetArticleInfoList" => {
            let number = arguments.get("Number")
                .and_then(|v| v.as_i64())
                .unwrap_or(10);
            let keyword = arguments.get("KeyWord")
                .and_then(|v| v.as_str())
                .map(String::from);
            get_article_info_list(mcp_key, number, keyword.as_deref()).unwrap_or_else(|e| format!("Error: {}", e))
        }

        "GetArticleVersionList" => {
            let id = arguments.get("Id")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            if id.is_empty() {
                return "Error: Id is required".to_string();
            }
            get_article_version_list(mcp_key, id).unwrap_or_else(|e| format!("Error: {}", e))
        }

        "GetArticleFull" => {
            let id = arguments.get("Id")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let version = arguments.get("Version")
                .and_then(|v| v.as_i64())
                .unwrap_or(1) as i32;
            if id.is_empty() {
                return "Error: Id is required".to_string();
            }
            get_article_full(mcp_key, id, version).unwrap_or_else(|e| format!("Error: {}", e))
        }

        "DeleteArticle" => {
            let id = arguments.get("Id")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let version = arguments.get("Version")
                .and_then(|v| v.as_i64())
                .unwrap_or(1) as i32;
            if id.is_empty() {
                return "Error: Id is required".to_string();
            }
            delete_article(mcp_key, id, version).unwrap_or_else(|e| format!("Error: {}", e))
        }

        "AddArticle" => {
            let title = arguments.get("Title")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let content = arguments.get("Content")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            if title.is_empty() || content.is_empty() {
                return "Error: Title and Content are required".to_string();
            }
            let file_list: Vec<String> = arguments.get("FileList")
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default();
            add_article(mcp_key, title, content, file_list).unwrap_or_else(|e| format!("Error: {}", e))
        }

        "ChangeArticle" => {
            let id = arguments.get("Id")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let title = arguments.get("Title")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let content = arguments.get("Content")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            if id.is_empty() || title.is_empty() || content.is_empty() {
                return "Error: Id, Title and Content are required".to_string();
            }
            let file_list: Vec<String> = arguments.get("FileList")
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default();
            change_article(mcp_key, id, title, content, file_list).unwrap_or_else(|e| format!("Error: {}", e))
        }

        _ => format!("Unknown tool: {}", tool_name),
    }
}

fn main() {
    dotenvy::dotenv().ok();

    let mcp_key = std::env::var("MCP_KEY").unwrap_or_else(|_| "".to_string());

    if mcp_key.is_empty() {
        eprintln!("Error: MCP_KEY environment variable is not set");
        std::process::exit(1);
    }

    let stdin = io::stdin();
    let stdin = stdin.lock();

    for line in stdin.lines() {
        let input = match line {
            Ok(l) => l,
            Err(_) => continue,
        };

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        let request: Value = match serde_json::from_str(input) {
            Ok(v) => v,
            Err(_) => continue,
        };

        let method = request.get("method").and_then(|m| m.as_str()).unwrap_or("");
        let id = request.get("id").cloned();

        if method == "tools/call" {
            let params = request.get("params");
            let tool_name = params
                .and_then(|p| p.get("name"))
                .and_then(|n| n.as_str())
                .unwrap_or("");
            let arguments = params
                .and_then(|p| p.get("arguments"))
                .unwrap_or(&serde_json::Value::Null);

            let result = handle_tool_call(&mcp_key, tool_name, arguments);

            let response = serde_json::json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": {
                    "content": [{
                        "type": "text",
                        "text": result
                    }]
                }
            });

            println!("{}", response);
            io::stdout().flush().ok();
        } else if method == "initialize" {
            let response = serde_json::json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": {
                    "protocolVersion": "2024-11-05",
                    "capabilities": { "tools": {} },
                    "serverInfo": {
                        "name": "iblog_mcpserver",
                        "version": "0.1.0"
                    }
                }
            });
            println!("{}", response);
            io::stdout().flush().ok();
        } else if method == "notifications/initialized" {
            continue;
        } else if method == "tools/list" {
            let response = serde_json::json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": {
                    "tools": [
                        {
                            "name": "CheckKeyAndServer",
                            "description": "Check if the MCP Key and server are available. Returns key status and permission info.",
                            "inputSchema": {
                                "type": "object",
                                "properties": {},
                                "required": []
                            }
                        },
                        {
                            "name": "GetArticleNumber",
                            "description": "Get the total number of articles in the system.",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "KeyWord": {
                                        "type": "string",
                                        "description": "Keyword to search in Title or Content (optional)"
                                    }
                                },
                                "required": []
                            }
                        },
                        {
                            "name": "GetArticleInfoList",
                            "description": "Get a list of articles with basic info. Each article shows the latest version only.",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "Number": {
                                        "type": "number",
                                        "description": "Number of articles to retrieve (default: 10)"
                                    },
                                    "KeyWord": {
                                        "type": "string",
                                        "description": "Keyword to search in Title or Content (optional)"
                                    }
                                },
                                "required": []
                            }
                        },
                        {
                            "name": "GetArticleVersionList",
                            "description": "Get all versions of a specific article.",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "Id": {
                                        "type": "string",
                                        "description": "The article ID"
                                    }
                                },
                                "required": ["Id"]
                            }
                        },
                        {
                            "name": "GetArticleFull",
                            "description": "Get the full content of a specific article version.",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "Id": {
                                        "type": "string",
                                        "description": "The article ID"
                                    },
                                    "Version": {
                                        "type": "number",
                                        "description": "The article version number"
                                    }
                                },
                                "required": ["Id", "Version"]
                            }
                        },
                        {
                            "name": "DeleteArticle",
                            "description": "Delete a specific article version. Requires Remove permission.",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "Id": {
                                        "type": "string",
                                        "description": "The article ID"
                                    },
                                    "Version": {
                                        "type": "number",
                                        "description": "The article version number"
                                    }
                                },
                                "required": ["Id", "Version"]
                            }
                        },
                        {
                            "name": "AddArticle",
                            "description": "Add a new article. Requires Add permission.",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "Title": {
                                        "type": "string",
                                        "description": "Article title"
                                    },
                                    "Content": {
                                        "type": "string",
                                        "description": "Article content"
                                    },
                                    "FileList": {
                                        "type": "array",
                                        "items": { "type": "string" },
                                        "description": "List of file paths to upload (optional)"
                                    }
                                },
                                "required": ["Title", "Content"]
                            }
                        },
                        {
                            "name": "ChangeArticle",
                            "description": "Change an existing article by creating a new version. Requires Change permission.",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "Id": {
                                        "type": "string",
                                        "description": "The article ID to change"
                                    },
                                    "Title": {
                                        "type": "string",
                                        "description": "New article title"
                                    },
                                    "Content": {
                                        "type": "string",
                                        "description": "New article content"
                                    },
                                    "FileList": {
                                        "type": "array",
                                        "items": { "type": "string" },
                                        "description": "List of file paths to upload (optional)"
                                    }
                                },
                                "required": ["Id", "Title", "Content"]
                            }
                        }
                    ]
                }
            });
            println!("{}", response);
            io::stdout().flush().ok();
        } else {
            let response = serde_json::json!({
                "jsonrpc": "2.0",
                "id": id,
                "error": {
                    "code": -32601,
                    "message": format!("Method not found: {}", method)
                }
            });
            println!("{}", response);
            io::stdout().flush().ok();
        }
    }
}
