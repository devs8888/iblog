use actix_web::{web, App, HttpResponse, HttpServer, HttpRequest, cookie::Cookie};
use actix_cors::Cors;
use actix_multipart::Multipart;
use futures_util::StreamExt;
use sqlx::mysql::MySqlPool;
use serde::{Deserialize, Serialize};
use rand::Rng;
use sha2::{Sha256, Digest};
use std::io::Write;
use std::path::Path;
use urlencoding;

const COOKIE_NAME: &str = "iblog_token";
const SESSION_TIMEOUT_MINUTES: i64 = 30;
const UPLOAD_DIR: &str = "./Upload/Avatar";
const ARTICLE_FILE_DIR: &str = "./Upload/ArticleFile";
const MAX_FILE_SIZE: usize = 2 * 1024 * 1024; // 2MB

// 数据库连接池状态
struct AppState {
    db: MySqlPool,
}

// 请求结构
#[derive(Debug, Deserialize)]
struct LoginRequest {
    UserName: String,
    PassWord: String,
}

#[derive(Debug, Deserialize)]
struct LogoutRequest {
    Token: String,
}

#[derive(Debug, Deserialize)]
struct UserNameRequest {
    UserName: String,
}

#[derive(Debug, Deserialize)]
struct SetUserInfoRequest {
    UserName: String,
    DisplayName: Option<String>,
    Signature: Option<String>,
    Introduction: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ChangePassWordRequest {
    NewPassWord: String,
}

#[derive(Debug, Deserialize)]
struct AddArticleRequest {
    Sender: String,
    Title: String,
    Content: String,
}

#[derive(Debug, Deserialize)]
struct UpdateArticleFileRequest {
    ArticleId: String,
    FilePath: String,
}

#[derive(Debug, Deserialize)]
struct GetArticleInfoRequest {
    Page: i64,
    #[serde(default)]
    KeyWord: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GetArticleNumberRequest {
    #[serde(default)]
    KeyWord: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GetNewArticlesPageViewsByTimeRequest {
    Time: String,
}

#[derive(Debug, Deserialize)]
struct GetNewArticlesNumberByTimeRequest {
    Time: String,
}

#[derive(Debug, Serialize)]
struct RankItem {
    Date: String,
    Title: String,
    PageViews: i32,
}

#[derive(Debug, Serialize)]
struct GetArticlesRankListResponse {
    Status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    Rank1: Option<RankItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    Rank2: Option<RankItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    Rank3: Option<RankItem>,
}

#[derive(Debug, Serialize)]
struct GetSiteSettingResponse {
    Status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    SiteName: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    SiteIntroduction: Option<String>,
}

#[derive(Debug, Deserialize)]
struct SetSiteSettingRequest {
    SiteName: String,
    SiteIntroduction: String,
}

#[derive(Debug, Deserialize)]
struct DeleteArticleRequest {
    Id: String,
    Version: i32,
}

#[derive(Debug, Deserialize)]
struct GetArticleFullRequest {
    Id: String,
    Version: i32,
}

#[derive(Debug, Deserialize)]
struct GetArticleVersionRequest {
    Id: String,
}

#[derive(Debug, Deserialize)]
struct GetMcpKeyInfoRequest {
    Page: i64,
}

#[derive(Debug, Serialize)]
struct GetMcpKeyNumberResponse {
    Status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    Number: Option<i64>,
}

#[derive(Debug, Deserialize)]
struct ChangeArticleRequest {
    Id: String,
    Sender: String,
    Title: String,
    Content: String,
    #[serde(default)]
    DeletedFiles: Vec<String>,
}

// MCP Key 请求结构
#[derive(Debug, Deserialize)]
struct AddMcpKeyRequest {
    Auth_Read: i32,
    Auth_Add: i32,
    Auth_Change: i32,
    Auth_Remove: i32,
}

#[derive(Debug, Deserialize)]
struct ChangeMcpKeyRequest {
    Key: String,
    Auth_Read: i32,
    Auth_Add: i32,
    Auth_Change: i32,
    Auth_Remove: i32,
}

#[derive(Debug, Serialize)]
struct GetMcpKeyAuthInfoResponse {
    KeyStatus: String,
    ServerStatus: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    ReadAuth: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    AddAuth: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    RemoveAuth: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ChangeAuth: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct GetArticleInfoListRequest {
    Number: i64,
    #[serde(default)]
    KeyWord: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GetArticleAllVersionListRequest {
    Id: String,
}

#[derive(Debug, Deserialize)]
struct McpGetArticleFullRequest {
    Id: String,
    Version: i32,
}

#[derive(Debug, Deserialize)]
struct McpDeleteArticleRequest {
    Id: String,
    Version: i32,
}

#[derive(Debug, Deserialize)]
struct McpAddArticleRequest {
    Title: String,
    Content: String,
}

#[derive(Debug, Deserialize)]
struct McpChangeArticleRequest {
    Id: String,
    Title: String,
    Content: String,
}

#[derive(Debug, Serialize)]
struct McpChangeArticleResponse {
    Status: String,
    Auth: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    Version: Option<i32>,
}

#[derive(Debug, Serialize)]
struct McpDeleteArticleResponse {
    Status: String,
    Auth: bool,
}

#[derive(Debug, Serialize)]
struct LogInfo {
    Date: String,
    Operator: String,
    Action: String,
    Object: String,
}

#[derive(Debug, Serialize)]
struct GetLogInfoResponse {
    Status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    LogInfo: Option<Vec<LogInfo>>,
}

#[derive(Debug, Serialize)]
struct GetLogNumberResponse {
    Status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    Number: Option<i64>,
}

#[derive(Debug, Deserialize)]
struct GetLogInfoRequest {
    Page: i64,
}

#[derive(Debug, Serialize)]
struct ApiResponse {
    Status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    Token: Option<String>,
}

#[derive(Debug, Serialize)]
struct McpAddArticleResponse {
    Status: String,
    Auth: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    ArticleId: Option<String>,
}

#[derive(Debug, Serialize)]
struct McpFileUploadResponse {
    Status: String,
    Auth: bool,
}

#[derive(Debug, Serialize)]
struct ArticleInfoListItem {
    Id: String,
    Sender: String,
    Title: String,
    Content: String,
    Version: i32,
    File: bool,
}

#[derive(Debug, Serialize)]
struct ArticleVersionListItem {
    Version: i32,
    Sender: String,
    Title: String,
    Content: String,
    File: bool,
}

#[derive(Debug, Serialize)]
struct GetArticleInfoListResponse {
    Status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    InfoList: Option<Vec<ArticleInfoListItem>>,
}

#[derive(Debug, Serialize)]
struct GetArticleAllVersionListResponse {
    Status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    VersionList: Option<Vec<ArticleVersionListItem>>,
}

#[derive(Debug, Serialize)]
struct McpGetArticleFullResponse {
    Status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    Title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    Content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    FileList: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct DeleteMcpKeyRequest {
    Key: String,
}

#[derive(Debug, Serialize)]
struct AddMcpKeyResponse {
    Status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    Key: Option<String>,
}

#[derive(Debug, Serialize)]
struct McpKeyInfo {
    Date: String,
    Key: String,
    Auth_Read: i32,
    Auth_Add: i32,
    Auth_Change: i32,
    Auth_Remove: i32,
}

#[derive(Debug, Serialize)]
struct GetMcpKeysResponse {
    Status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    McpKeys: Option<Vec<McpKeyInfo>>,
}

#[derive(Debug, Serialize)]
struct ArticleInfo {
    Id: String,
    Sender: String,
    Version: i32,
    Date: String,
    Title: String,
    Content: String,
    PageViews: i32,
    File: Option<String>,
}

#[derive(Debug, Serialize)]
struct GetArticleInfoResponse {
    Status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    ArticleInfo: Option<Vec<ArticleInfo>>,
}

#[derive(Debug, Serialize)]
struct GetArticleFullAdminResponse {
    Status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    Article: Option<ArticleInfo>,
}

#[derive(Debug, Serialize)]
struct ArticleVersionInfo {
    Version: i32,
    Date: String,
    Title: String,
    Content: String,
    PageViews: i32,
}

#[derive(Debug, Serialize)]
struct GetArticleVersionResponse {
    Status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    ArticleInfo: Option<Vec<ArticleVersionInfo>>,
}

// 用户信息响应
#[derive(Debug, Serialize)]
struct UserInfoResponse {
    Status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    UserName: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    DisplayName: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    Signature: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    Introduction: Option<String>,
}

#[derive(Debug, Serialize)]
struct ShowUserInfoResponse {
    Status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    DisplayName: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    Signature: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    Introduction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    AvatarUrl: Option<String>,
}

#[derive(Debug, Serialize)]
struct AvatarUrlResponse {
    Status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    AvatarUrl: Option<String>,
}

#[derive(Debug, Serialize)]
struct ArticleNumberResponse {
    Status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    Number: Option<i64>,
}

#[derive(Debug, Serialize)]
struct TotalPageViewsResponse {
    Status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    PageViews: Option<i64>,
}

#[derive(Debug, Serialize)]
struct AddArticleResponse {
    Status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    ArticleId: Option<String>,
}

#[derive(Debug, Serialize)]
struct ChangeArticleResponse {
    Status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    Version: Option<i32>,
}

// 生成20位随机Token
fn generate_token() -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = rand::thread_rng();
    (0..20)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

// 输入验证
fn validate_input(s: &str) -> bool {
    !s.is_empty() && s.len() <= 100 && !s.contains('<') && !s.contains('>') && !s.contains('\"') && !s.contains('\'')
}

// 验证文件名（防路径遍历和危险字符）
fn validate_filename(filename: &str) -> bool {
    if filename.is_empty() || filename.len() > 255 {
        return false;
    }
    // 只允许字母、数字、下划线、点和短横线
    filename.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-' || c == '.')
}

// 获取文件扩展名
fn get_file_extension(filename: &str) -> Option<String> {
    Path::new(filename)
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_lowercase())
}

// 检查session是否过期
async fn is_session_expired(db: &MySqlPool, token: &str) -> bool {
    let result = sqlx::query_as::<_, (Option<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>>,)>(
        "SELECT LastActivity FROM User WHERE Token = ?"
    )
    .bind(token)
    .fetch_optional(db)
    .await;

    match result {
        Ok(Some((Some(last_activity,),))) => {
            let now = sqlx::types::chrono::Utc::now();
            let elapsed = now.signed_duration_since(last_activity);
            elapsed.num_minutes() >= 5
        }
        _ => true
    }
}

// 更新最后活动时间
async fn update_last_activity(db: &MySqlPool, token: &str) -> bool {
    let result = sqlx::query("UPDATE User SET LastActivity = NOW() WHERE Token = ?")
        .bind(token)
        .execute(db)
        .await;
    result.is_ok()
}

// 从 Cookie 中获取 Token
fn get_token_from_request(req: &HttpRequest) -> Option<String> {
    req.cookie(COOKIE_NAME)
        .map(|cookie| cookie.value().to_string())
}

// 创建 HttpOnly Cookie
fn create_token_cookie(token: &str) -> Cookie<'static> {
    Cookie::build(COOKIE_NAME, token.to_string())
        .path("/")
        .http_only(true)
        .secure(false)
        .max_age(actix_web::cookie::time::Duration::minutes(SESSION_TIMEOUT_MINUTES))
        .finish()
}

// 创建删除 Cookie
fn create_delete_cookie() -> Cookie<'static> {
    Cookie::build(COOKIE_NAME, "".to_string())
        .path("/")
        .http_only(true)
        .max_age(actix_web::cookie::time::Duration::ZERO)
        .finish()
}

// 登录接口
async fn login(
    data: web::Data<AppState>,
    body: web::Json<LoginRequest>,
) -> HttpResponse {
    if !validate_input(&body.UserName) || !validate_input(&body.PassWord) {
        return HttpResponse::BadRequest().json(ApiResponse {
            Status: "False".to_string(),
            Token: None,
        });
    }

    let result = sqlx::query_as::<_, (String,)>(
        "SELECT EncryptedPassWord FROM User WHERE UserName = ?"
    )
    .bind(&body.UserName)
    .fetch_optional(&data.db)
    .await;

    match result {
        Ok(Some((stored_password,))) => {
            if body.PassWord == stored_password {
                let token = generate_token();

                let update_result = sqlx::query(
                    "UPDATE User SET Token = ?, LastActivity = NOW() WHERE UserName = ?"
                )
                .bind(&token)
                .bind(&body.UserName)
                .execute(&data.db)
                .await;

                match update_result {
                    Ok(_) => {
                        let cookie = create_token_cookie(&token);
                        HttpResponse::Ok()
                            .cookie(cookie)
                            .json(ApiResponse {
                                Status: "True".to_string(),
                                Token: Some(token),
                            })
                    }
                    Err(_) => HttpResponse::InternalServerError().json(ApiResponse {
                        Status: "False".to_string(),
                        Token: None,
                    }),
                }
            } else {
                HttpResponse::Ok().json(ApiResponse {
                    Status: "False".to_string(),
                    Token: None,
                })
            }
        }
        Ok(None) => HttpResponse::Ok().json(ApiResponse {
            Status: "False".to_string(),
            Token: None,
        }),
        Err(_) => HttpResponse::InternalServerError().json(ApiResponse {
            Status: "False".to_string(),
            Token: None,
        }),
    }
}

// 登出接口
async fn logout(
    data: web::Data<AppState>,
    req: HttpRequest,
) -> HttpResponse {
    let token = get_token_from_request(&req);

    if let Some(token) = token {
        if token.len() == 20 {
            let _ = sqlx::query("UPDATE User SET Token = NULL, LastActivity = NULL WHERE Token = ?")
                .bind(&token)
                .execute(&data.db)
                .await;
        }
    }

    let delete_cookie = create_delete_cookie();
    HttpResponse::Ok()
        .cookie(delete_cookie)
        .json(ApiResponse {
            Status: "True".to_string(),
            Token: None,
        })
}

// 心跳接口
async fn heartbeat(
    data: web::Data<AppState>,
    req: HttpRequest,
) -> HttpResponse {
    let token = match get_token_from_request(&req) {
        Some(t) if t.len() == 20 => t,
        _ => {
            return HttpResponse::Ok().json(ApiResponse {
                Status: "False".to_string(),
                Token: None,
            });
        }
    };

    if is_session_expired(&data.db, &token).await {
        return HttpResponse::Ok().json(ApiResponse {
            Status: "False".to_string(),
            Token: None,
        });
    }

    if update_last_activity(&data.db, &token).await {
        HttpResponse::Ok().json(ApiResponse {
            Status: "True".to_string(),
            Token: None,
        })
    } else {
        HttpResponse::InternalServerError().json(ApiResponse {
            Status: "False".to_string(),
            Token: None,
        })
    }
}

// Token 验证接口
async fn verify_token(
    data: web::Data<AppState>,
    req: HttpRequest,
) -> HttpResponse {
    let token = match get_token_from_request(&req) {
        Some(t) if t.len() == 20 => t,
        _ => {
            return HttpResponse::Ok().json(ApiResponse {
                Status: "False".to_string(),
                Token: None,
            });
        }
    };

    let result = sqlx::query("SELECT UserName FROM User WHERE Token = ?")
        .bind(&token)
        .fetch_optional(&data.db)
        .await;

    match result {
        Ok(Some(_)) => {
            if is_session_expired(&data.db, &token).await {
                let _ = sqlx::query("UPDATE User SET Token = NULL, LastActivity = NULL WHERE Token = ?")
                    .bind(&token)
                    .execute(&data.db)
                    .await;
                let delete_cookie = create_delete_cookie();
                return HttpResponse::Ok()
                    .cookie(delete_cookie)
                    .json(ApiResponse {
                        Status: "False".to_string(),
                        Token: None,
                    });
            }

            let _ = update_last_activity(&data.db, &token).await;

            HttpResponse::Ok().json(ApiResponse {
                Status: "True".to_string(),
                Token: None,
            })
        }
        _ => HttpResponse::Ok().json(ApiResponse {
            Status: "False".to_string(),
            Token: None,
        }),
    }
}

// 获取公开用户信息（无需认证）
async fn get_show_user_info(
    data: web::Data<AppState>,
) -> HttpResponse {
    let result = sqlx::query_as::<_, (Option<String>, Option<String>, Option<String>, Option<String>)>(
        "SELECT DisplayName, Signature, Introduction, AvatarUrl FROM User LIMIT 1"
    )
    .fetch_optional(&data.db)
    .await;

    match result {
        Ok(Some((display_name, signature, introduction, avatar_url))) => {
            HttpResponse::Ok().json(ShowUserInfoResponse {
                Status: "True".to_string(),
                DisplayName: display_name,
                Signature: signature,
                Introduction: introduction,
                AvatarUrl: avatar_url,
            })
        }
        _ => HttpResponse::Ok().json(ShowUserInfoResponse {
            Status: "False".to_string(),
            DisplayName: None,
            Signature: None,
            Introduction: None,
            AvatarUrl: None,
        }),
    }
}

// 获取用户信息
async fn get_user_info(
    data: web::Data<AppState>,
    req: HttpRequest,
) -> HttpResponse {
    let token = match get_token_from_request(&req) {
        Some(t) if t.len() == 20 => t,
        _ => {
            return HttpResponse::Ok().json(UserInfoResponse {
                Status: "False".to_string(),
                UserName: None,
                DisplayName: None,
                Signature: None,
                Introduction: None,
            });
        }
    };

    // 验证 session
    if is_session_expired(&data.db, &token).await {
        return HttpResponse::Ok().json(UserInfoResponse {
            Status: "False".to_string(),
            UserName: None,
            DisplayName: None,
            Signature: None,
            Introduction: None,
        });
    }

    // 获取用户信息
    let result = sqlx::query_as::<_, (String, Option<String>, Option<String>, Option<String>, Option<String>)>(
        "SELECT UserName, DisplayName, Signature, Introduction, AvatarUrl FROM User WHERE Token = ?"
    )
    .bind(&token)
    .fetch_optional(&data.db)
    .await;

    match result {
        Ok(Some((user_name, display_name, signature, introduction, _avatar_url))) => {
            let _ = update_last_activity(&data.db, &token).await;
            HttpResponse::Ok().json(UserInfoResponse {
                Status: "True".to_string(),
                UserName: Some(user_name),
                DisplayName: display_name,
                Signature: signature,
                Introduction: introduction,
            })
        }
        _ => HttpResponse::Ok().json(UserInfoResponse {
            Status: "False".to_string(),
            UserName: None,
            DisplayName: None,
            Signature: None,
            Introduction: None,
        }),
    }
}

// 设置用户信息
async fn set_user_info(
    data: web::Data<AppState>,
    req: HttpRequest,
    body: web::Json<SetUserInfoRequest>,
) -> HttpResponse {
    let token = match get_token_from_request(&req) {
        Some(t) if t.len() == 20 => t,
        _ => {
            return HttpResponse::Ok().json(ApiResponse {
                Status: "False".to_string(),
                Token: None,
            });
        }
    };

    // 验证 session
    if is_session_expired(&data.db, &token).await {
        return HttpResponse::Ok().json(ApiResponse {
            Status: "False".to_string(),
            Token: None,
        });
    }

    // 输入验证 - 防止 XSS
    if !validate_input(&body.UserName) {
        return HttpResponse::Ok().json(ApiResponse {
            Status: "False".to_string(),
            Token: None,
        });
    }

    // 更新用户信息
    let result = sqlx::query(
        "UPDATE User SET DisplayName = ?, Signature = ?, Introduction = ? WHERE Token = ?"
    )
    .bind(&body.DisplayName)
    .bind(&body.Signature)
    .bind(&body.Introduction)
    .bind(&token)
    .execute(&data.db)
    .await;

    match result {
        Ok(_) => {
            let _ = update_last_activity(&data.db, &token).await;
            HttpResponse::Ok().json(ApiResponse {
                Status: "True".to_string(),
                Token: None,
            })
        }
        Err(_) => HttpResponse::InternalServerError().json(ApiResponse {
            Status: "False".to_string(),
            Token: None,
        }),
    }
}

// 修改密码
async fn change_password(
    data: web::Data<AppState>,
    req: HttpRequest,
    body: web::Json<ChangePassWordRequest>,
) -> HttpResponse {
    let token = match get_token_from_request(&req) {
        Some(t) if t.len() == 20 => t,
        _ => {
            return HttpResponse::Ok().json(ApiResponse {
                Status: "False".to_string(),
                Token: None,
            });
        }
    };

    // 验证 session
    if is_session_expired(&data.db, &token).await {
        return HttpResponse::Ok().json(ApiResponse {
            Status: "False".to_string(),
            Token: None,
        });
    }

    // 输入验证
    if body.NewPassWord.is_empty() || body.NewPassWord.len() > 100 {
        return HttpResponse::Ok().json(ApiResponse {
            Status: "False".to_string(),
            Token: None,
        });
    }

    // SHA256 加密密码
    let mut hasher = Sha256::new();
    hasher.update(body.NewPassWord.as_bytes());
    let encrypted_password = format!("{:x}", hasher.finalize());

    // 更新密码
    let result = sqlx::query("UPDATE User SET EncryptedPassWord = ? WHERE Token = ?")
        .bind(&encrypted_password)
        .bind(&token)
        .execute(&data.db)
        .await;

    match result {
        Ok(_) => {
            let _ = update_last_activity(&data.db, &token).await;
            HttpResponse::Ok().json(ApiResponse {
                Status: "True".to_string(),
                Token: None,
            })
        }
        Err(_) => HttpResponse::Ok().json(ApiResponse {
            Status: "False".to_string(),
            Token: None,
        }),
    }
}

// 上传头像
async fn upload_avatar(
    data: web::Data<AppState>,
    req: HttpRequest,
    mut payload: Multipart,
) -> HttpResponse {
    let token = match get_token_from_request(&req) {
        Some(t) if t.len() == 20 => t,
        _ => {
            return HttpResponse::Ok().json(ApiResponse {
                Status: "False".to_string(),
                Token: None,
            });
        }
    };

    // 验证 session
    if is_session_expired(&data.db, &token).await {
        return HttpResponse::Ok().json(ApiResponse {
            Status: "False".to_string(),
            Token: None,
        });
    }

    // 获取用户名
    let user_result = sqlx::query_as::<_, (String,)>(
        "SELECT UserName FROM User WHERE Token = ?"
    )
    .bind(&token)
    .fetch_optional(&data.db)
    .await;

    let user_name = match user_result {
        Ok(Some((name,))) => name,
        _ => {
            return HttpResponse::Ok().json(ApiResponse {
                Status: "False".to_string(),
                Token: None,
            });
        }
    };

    // 确保上传目录存在
    if let Err(_) = std::fs::create_dir_all(UPLOAD_DIR) {
        return HttpResponse::InternalServerError().json(ApiResponse {
            Status: "False".to_string(),
            Token: None,
        });
    }

    // 处理文件上传
    while let Some(item) = payload.next().await {
        let mut field = match item {
            Ok(f) => f,
            Err(_) => continue,
        };

        let content_disposition = field.content_disposition();
        let filename = match content_disposition {
            Some(cd) => cd.get_filename().unwrap_or_default().to_string(),
            None => continue,
        };

        // 验证文件类型
        let ext = get_file_extension(&filename);
        if ext.is_none() || !["jpg", "jpeg", "png"].contains(&ext.as_ref().unwrap().as_str()) {
            continue;
        }

        // 读取文件内容
        let mut file_data: Vec<u8> = Vec::new();
        while let Some(chunk_result) = field.next().await {
            match chunk_result {
                Ok(data) => {
                    file_data.extend_from_slice(&data);
                    // 检查文件大小
                    if file_data.len() > MAX_FILE_SIZE {
                        return HttpResponse::Ok().json(ApiResponse {
                            Status: "False".to_string(),
                            Token: None,
                        });
                    }
                }
                Err(_) => break,
            }
        }

        // 验证文件大小
        if file_data.is_empty() || file_data.len() > MAX_FILE_SIZE {
            continue;
        }

        // 生成安全文件名：用户名.扩展名
        let safe_filename = format!("{}.{}", user_name, ext.unwrap());
        let file_path = format!("{}/{}", UPLOAD_DIR, safe_filename);

        // 删除旧头像（如果存在且不同）
        let old_avatar_path = Path::new(&file_path);
        if old_avatar_path.exists() {
            let _ = std::fs::remove_file(old_avatar_path);
        }

        // 写入文件
        match std::fs::File::create(&file_path) {
            Ok(mut file) => {
                if file.write_all(&file_data).is_ok() {
                    // 更新数据库
                    let avatar_url = format!("/Api/GetAvatar?AvatarName={}", safe_filename);
                    let update_result = sqlx::query(
                        "UPDATE User SET AvatarUrl = ? WHERE Token = ?"
                    )
                    .bind(&avatar_url)
                    .bind(&token)
                    .execute(&data.db)
                    .await;

                    let _ = update_last_activity(&data.db, &token).await;

                    if update_result.is_ok() {
                        return HttpResponse::Ok().json(ApiResponse {
                            Status: "True".to_string(),
                            Token: None,
                        });
                    }
                }
            }
            Err(_) => continue,
        }
    }

    HttpResponse::Ok().json(ApiResponse {
        Status: "False".to_string(),
        Token: None,
    })
}

// 获取头像图片
async fn get_avatar(req: HttpRequest) -> HttpResponse {
    let avatar_name = req
        .query_string()
        .split('&')
        .find(|pair| pair.starts_with("AvatarName="))
        .map(|pair| pair.split('=').nth(1).unwrap_or(""))
        .unwrap_or("");

    // 验证文件名
    if !validate_filename(avatar_name) {
        return HttpResponse::Ok().json(ApiResponse {
            Status: "False".to_string(),
            Token: None,
        });
    }

    let file_path = format!("{}/{}", UPLOAD_DIR, avatar_name);

    // 检查文件是否存在
    if !Path::new(&file_path).exists() {
        return HttpResponse::Ok().json(ApiResponse {
            Status: "False".to_string(),
            Token: None,
        });
    }

    // 根据扩展名确定内容类型
    let content_type = match get_file_extension(avatar_name).as_ref().map(|s| s.as_str()) {
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("png") => "image/png",
        _ => "application/octet-stream",
    };

    // 读取并返回文件
    match std::fs::read(&file_path) {
        Ok(data) => HttpResponse::Ok()
            .content_type(content_type)
            .body(data),
        Err(_) => HttpResponse::Ok().json(ApiResponse {
            Status: "False".to_string(),
            Token: None,
        }),
    }
}

// 根据用户名获取头像URL
async fn get_avatar_url_by_username(
    data: web::Data<AppState>,
    body: web::Json<UserNameRequest>,
) -> HttpResponse {
    if !validate_input(&body.UserName) {
        return HttpResponse::Ok().json(AvatarUrlResponse {
            Status: "False".to_string(),
            AvatarUrl: None,
        });
    }

    let result = sqlx::query_as::<_, (Option<String>,)>(
        "SELECT AvatarUrl FROM User WHERE UserName = ?"
    )
    .bind(&body.UserName)
    .fetch_optional(&data.db)
    .await;

    match result {
        Ok(Some((Some(avatar_url),))) => {
            HttpResponse::Ok().json(AvatarUrlResponse {
                Status: "True".to_string(),
                AvatarUrl: Some(avatar_url),
            })
        }
        _ => HttpResponse::Ok().json(AvatarUrlResponse {
            Status: "False".to_string(),
            AvatarUrl: None,
        }),
    }
}

// 添加文章
async fn admin_add_article(
    data: web::Data<AppState>,
    req: HttpRequest,
    body: web::Json<AddArticleRequest>,
) -> HttpResponse {
    // 验证 Token
    let token = match get_token_from_request(&req) {
        Some(t) if t.len() == 20 => t,
        _ => {
            return HttpResponse::Ok().json(ApiResponse {
                Status: "False".to_string(),
                Token: None,
            });
        }
    };

    // 验证 session
    if is_session_expired(&data.db, &token).await {
        return HttpResponse::Ok().json(ApiResponse {
            Status: "False".to_string(),
            Token: None,
        });
    }

    // 验证 Token 对应的用户名是否与 Sender 匹配
    let user_result = sqlx::query_as::<_, (String,)>(
        "SELECT UserName FROM User WHERE Token = ?"
    )
    .bind(&token)
    .fetch_optional(&data.db)
    .await;

    let token_user_name = match user_result {
        Ok(Some((name,))) => name,
        _ => {
            return HttpResponse::Ok().json(ApiResponse {
                Status: "False".to_string(),
                Token: None,
            });
        }
    };

    if token_user_name != body.Sender {
        return HttpResponse::Ok().json(ApiResponse {
            Status: "False".to_string(),
            Token: None,
        });
    }

    // 输入验证 - 防止 XSS
    if !validate_input(&body.Title) || !validate_input(&body.Content) {
        return HttpResponse::Ok().json(ApiResponse {
            Status: "False".to_string(),
            Token: None,
        });
    }

    // 生成30位唯一 Id
    let article_id = generate_unique_id(&data.db).await;

    // 插入文章
    let result = sqlx::query(
        "INSERT INTO article (Id, Sender, Version, Date, Title, Content, PageViews) VALUES (?, ?, 1, NOW(), ?, ?, 0)"
    )
    .bind(&article_id)
    .bind(&body.Sender)
    .bind(&body.Title)
    .bind(&body.Content)
    .execute(&data.db)
    .await;

    let _ = update_last_activity(&data.db, &token).await;

    match result {
        Ok(_) => {
            write_log(&data.db, &token_user_name, "Add", &article_id).await;
            HttpResponse::Ok().json(AddArticleResponse {
            Status: "True".to_string(),
            ArticleId: Some(article_id),
        })},
        Err(_) => HttpResponse::Ok().json(AddArticleResponse {
            Status: "False".to_string(),
            ArticleId: None,
        }),
    }
}

// 生成30位唯一 Id
async fn generate_unique_id(db: &MySqlPool) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

    loop {
        let id: String = (0..30)
            .map(|_| {
                let idx = rand::thread_rng().gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect();

        // 检查是否已存在
        let exists = sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM article WHERE Id = ?")
            .bind(&id)
            .fetch_optional(db)
            .await;

        if let Ok(Some((count,))) = exists {
            if count == 0 {
                return id;
            }
        }
    }
}

// 获取文章数量（无需认证）
async fn get_article_number(
    data: web::Data<AppState>,
    body: web::Json<GetArticleNumberRequest>,
) -> HttpResponse {
    let result = if let Some(ref keyword) = body.KeyWord {
        if keyword.is_empty() {
            sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM article")
                .fetch_one(&data.db)
                .await
        } else {
            let pattern = format!("%{}%", keyword);
            sqlx::query_as::<_, (i64,)>(
                "SELECT COUNT(*) FROM article WHERE Title LIKE ? OR Content LIKE ?"
            )
            .bind(&pattern)
            .bind(&pattern)
            .fetch_one(&data.db)
            .await
        }
    } else {
        sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM article")
            .fetch_one(&data.db)
            .await
    };

    match result {
        Ok((count,)) => HttpResponse::Ok().json(ArticleNumberResponse {
            Status: "True".to_string(),
            Number: Some(count),
        }),
        Err(_) => HttpResponse::Ok().json(ArticleNumberResponse {
            Status: "False".to_string(),
            Number: None,
        }),
    }
}

// 获取所有文章所有版本的PageViews之和
async fn get_total_page_views(
    data: web::Data<AppState>,
) -> HttpResponse {
    let result = sqlx::query_as::<_, (i64,)>("SELECT CAST(COALESCE(SUM(PageViews), 0) AS SIGNED) FROM article")
        .fetch_one(&data.db)
        .await;

    match result {
        Ok((sum,)) => HttpResponse::Ok().json(TotalPageViewsResponse {
            Status: "True".to_string(),
            PageViews: Some(sum),
        }),
        Err(_) => HttpResponse::Ok().json(TotalPageViewsResponse {
            Status: "False".to_string(),
            PageViews: None,
        }),
    }
}

// 获取指定日期发布的文章的所有版本的PageViews之和
async fn get_new_articles_page_views_by_time(
    data: web::Data<AppState>,
    body: web::Json<GetNewArticlesPageViewsByTimeRequest>,
) -> HttpResponse {
    let result = sqlx::query_as::<_, (i64,)>(
        &format!("SELECT CAST(COALESCE(SUM(PageViews), 0) AS SIGNED) FROM article WHERE LEFT(Date, 10) = '{}'", body.Time)
    )
    .fetch_one(&data.db)
    .await;

    match result {
        Ok((sum,)) => HttpResponse::Ok().json(TotalPageViewsResponse {
            Status: "True".to_string(),
            PageViews: Some(sum),
        }),
        Err(_) => HttpResponse::Ok().json(TotalPageViewsResponse {
            Status: "False".to_string(),
            PageViews: None,
        }),
    }
}

// 获取指定日期发布的文章的所有版本数量
async fn get_new_articles_number_by_time(
    data: web::Data<AppState>,
    body: web::Json<GetNewArticlesNumberByTimeRequest>,
) -> HttpResponse {
    let result = sqlx::query_as::<_, (i64,)>(
        &format!("SELECT COUNT(*) FROM article WHERE LEFT(Date, 10) = '{}'", body.Time)
    )
    .fetch_one(&data.db)
    .await;

    match result {
        Ok((count,)) => HttpResponse::Ok().json(ArticleNumberResponse {
            Status: "True".to_string(),
            Number: Some(count),
        }),
        Err(_) => HttpResponse::Ok().json(ArticleNumberResponse {
            Status: "False".to_string(),
            Number: None,
        }),
    }
}

// 获取文章排行榜前三
async fn get_articles_rank_list(
    data: web::Data<AppState>,
) -> HttpResponse {
    let result = sqlx::query_as::<_, (String, String, i32, sqlx::types::chrono::NaiveDateTime)>(
        "SELECT a.Id, a.Title, a.PageViews, a.Date FROM article a
         INNER JOIN (
             SELECT Id, MAX(Version) as MaxVersion
             FROM article
             GROUP BY Id
         ) b ON a.Id = b.Id AND a.Version = b.MaxVersion
         ORDER BY a.PageViews DESC, a.Date ASC
         LIMIT 3"
    )
    .fetch_all(&data.db)
    .await;

    match result {
        Ok(rows) => {
            let mut rank1 = None;
            let mut rank2 = None;
            let mut rank3 = None;
            for (i, row) in rows.into_iter().enumerate() {
                let (id, title, page_views, date) = row;
                let date_str = date.format("%Y-%m-%d").to_string();
                let item = RankItem {
                    Date: date_str,
                    Title: title,
                    PageViews: page_views,
                };
                match i {
                    0 => rank1 = Some(item),
                    1 => rank2 = Some(item),
                    2 => rank3 = Some(item),
                    _ => {}
                }
            }
            HttpResponse::Ok().json(GetArticlesRankListResponse {
                Status: "True".to_string(),
                Rank1: rank1,
                Rank2: rank2,
                Rank3: rank3,
            })
        }
        Err(_) => HttpResponse::Ok().json(GetArticlesRankListResponse {
            Status: "False".to_string(),
            Rank1: None,
            Rank2: None,
            Rank3: None,
        }),
    }
}

// 获取站点设置
async fn get_site_setting(
    data: web::Data<AppState>,
) -> HttpResponse {
    let result = sqlx::query_as::<_, (Option<String>, Option<String>)>(
        "SELECT SiteName, SiteIntroduction FROM setting LIMIT 1"
    )
    .fetch_optional(&data.db)
    .await;

    match result {
        Ok(Some((site_name, site_introduction))) => HttpResponse::Ok().json(GetSiteSettingResponse {
            Status: "True".to_string(),
            SiteName: site_name,
            SiteIntroduction: site_introduction,
        }),
        _ => HttpResponse::Ok().json(GetSiteSettingResponse {
            Status: "False".to_string(),
            SiteName: None,
            SiteIntroduction: None,
        }),
    }
}

// 设置站点设置
async fn set_site_setting(
    data: web::Data<AppState>,
    req: HttpRequest,
    body: web::Json<SetSiteSettingRequest>,
) -> HttpResponse {
    let token = match get_token_from_request(&req) {
        Some(t) if t.len() == 20 => t,
        _ => {
            return HttpResponse::Ok().json(ApiResponse {
                Status: "False".to_string(),
                Token: None,
            });
        }
    };

    if is_session_expired(&data.db, &token).await {
        return HttpResponse::Ok().json(ApiResponse {
            Status: "False".to_string(),
            Token: None,
        });
    }

    let result = sqlx::query(
        "UPDATE setting SET SiteName = ?, SiteIntroduction = ? LIMIT 1"
    )
    .bind(&body.SiteName)
    .bind(&body.SiteIntroduction)
    .execute(&data.db)
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().json(ApiResponse {
            Status: "True".to_string(),
            Token: None,
        }),
        Err(_) => HttpResponse::Ok().json(ApiResponse {
            Status: "False".to_string(),
            Token: None,
        }),
    }
}

// 获取文章列表（分页，无需认证）
async fn get_article_info(
    data: web::Data<AppState>,
    body: web::Json<GetArticleInfoRequest>,
) -> HttpResponse {
    let page = if body.Page <= 0 { 1 } else { body.Page };
    let offset = (page - 1) * 3;

    let result = if let Some(ref keyword) = body.KeyWord {
        if keyword.is_empty() {
            sqlx::query_as::<_, (String, String, i32, sqlx::types::chrono::NaiveDateTime, String, String, i32, Option<String>)>(
                "SELECT a.Id, a.Sender, a.Version, a.Date, a.Title, a.Content, a.PageViews, a.File FROM article a
                 INNER JOIN (
                     SELECT Id, MAX(Version) as MaxVersion
                     FROM article
                     GROUP BY Id
                 ) b ON a.Id = b.Id AND a.Version = b.MaxVersion
                 ORDER BY a.Date DESC LIMIT 3 OFFSET ?"
            )
            .bind(offset)
            .fetch_all(&data.db)
            .await
        } else {
            let pattern = format!("%{}%", keyword);
            sqlx::query_as::<_, (String, String, i32, sqlx::types::chrono::NaiveDateTime, String, String, i32, Option<String>)>(
                "SELECT a.Id, a.Sender, a.Version, a.Date, a.Title, a.Content, a.PageViews, a.File FROM article a
                 INNER JOIN (
                     SELECT Id, MAX(Version) as MaxVersion
                     FROM article
                     GROUP BY Id
                 ) b ON a.Id = b.Id AND a.Version = b.MaxVersion
                 WHERE a.Title LIKE ? OR a.Content LIKE ?
                 ORDER BY a.Date DESC LIMIT 3 OFFSET ?"
            )
            .bind(&pattern)
            .bind(&pattern)
            .bind(offset)
            .fetch_all(&data.db)
            .await
        }
    } else {
        sqlx::query_as::<_, (String, String, i32, sqlx::types::chrono::NaiveDateTime, String, String, i32, Option<String>)>(
            "SELECT a.Id, a.Sender, a.Version, a.Date, a.Title, a.Content, a.PageViews, a.File FROM article a
             INNER JOIN (
                 SELECT Id, MAX(Version) as MaxVersion
                 FROM article
                 GROUP BY Id
             ) b ON a.Id = b.Id AND a.Version = b.MaxVersion
             ORDER BY a.Date DESC LIMIT 3 OFFSET ?"
        )
        .bind(offset)
        .fetch_all(&data.db)
        .await
    };

    match result {
        Ok(rows) => {
            let articles: Vec<ArticleInfo> = rows.into_iter().map(|row| {
                let (id, sender, version, date, title, content, page_views, file) = row;
                let truncated_content = if content.chars().count() > 5 {
                    format!("{}...", content.chars().take(5).collect::<String>())
                } else {
                    content
                };
                let date_str = date.format("%Y-%m-%d %H:%M:%S").to_string();
                let file_value = file;
                ArticleInfo {
                    Id: id,
                    Sender: mask_mcp_key(&sender),
                    Version: version,
                    Date: date_str,
                    Title: title,
                    Content: truncated_content,
                    PageViews: page_views,
                    File: file_value,
                }
            }).collect();

            HttpResponse::Ok().json(GetArticleInfoResponse {
                Status: "True".to_string(),
                ArticleInfo: Some(articles),
            })
        }
        Err(_) => HttpResponse::Ok().json(GetArticleInfoResponse {
            Status: "False".to_string(),
            ArticleInfo: None,
        }),
    }
}

// 删除文章
async fn delete_article(
    data: web::Data<AppState>,
    req: HttpRequest,
    body: web::Json<DeleteArticleRequest>,
) -> HttpResponse {
    // 验证 Token
    let token = match get_token_from_request(&req) {
        Some(t) if t.len() == 20 => t,
        _ => {
            return HttpResponse::Ok().json(ApiResponse {
                Status: "False".to_string(),
                Token: None,
            });
        }
    };

    // 验证 session
    if is_session_expired(&data.db, &token).await {
        return HttpResponse::Ok().json(ApiResponse {
            Status: "False".to_string(),
            Token: None,
        });
    }

    // 删除文章
    let result = sqlx::query("DELETE FROM article WHERE Id = ? AND Version = ?")
        .bind(&body.Id)
        .bind(body.Version)
        .execute(&data.db)
        .await;

    // 删除文章对应版本的文件目录
    let article_dir = format!("{}/{}/{}", ARTICLE_FILE_DIR, body.Id, body.Version);
    if Path::new(&article_dir).exists() {
        let _ = std::fs::remove_dir_all(&article_dir);
    }

    let _ = update_last_activity(&data.db, &token).await;

    match result {
        Ok(_) => {
            write_log(&data.db, &token, "Remove", &body.Id).await;
            HttpResponse::Ok().json(ApiResponse {
            Status: "True".to_string(),
            Token: None,
        })},
        Err(_) => HttpResponse::Ok().json(ApiResponse {
            Status: "False".to_string(),
            Token: None,
        }),
    }
}

// 获取单个文章完整内容
async fn get_article_full(
    data: web::Data<AppState>,
    body: web::Json<GetArticleFullRequest>,
) -> HttpResponse {
    let _ = sqlx::query("UPDATE article SET PageViews = PageViews + 1 WHERE Id = ? AND Version = ?")
        .bind(&body.Id)
        .bind(body.Version)
        .execute(&data.db)
        .await;

    let result = sqlx::query_as::<_, (String, String, i32, sqlx::types::chrono::NaiveDateTime, String, String, i32, Option<String>)>(
        "SELECT Id, Sender, Version, Date, Title, Content, PageViews, File FROM article WHERE Id = ? AND Version = ?"
    )
    .bind(&body.Id)
    .bind(body.Version)
    .fetch_optional(&data.db)
    .await;

    match result {
        Ok(Some(row)) => {
            let (id, sender, version, date, title, content, page_views, file) = row;
            let date_str = date.format("%Y-%m-%d %H:%M:%S").to_string();
            HttpResponse::Ok().json(GetArticleFullAdminResponse {
                Status: "True".to_string(),
                Article: Some(ArticleInfo {
                    Id: id,
                    Sender: mask_mcp_key(&sender),
                    Version: version,
                    Date: date_str,
                    Title: title,
                    Content: content,
                    PageViews: page_views,
                    File: file,
                }),
            })
        }
        _ => HttpResponse::Ok().json(GetArticleFullAdminResponse {
            Status: "False".to_string(),
            Article: None,
        }),
    }
}

// 获取文章所有版本
async fn get_article_version(
    data: web::Data<AppState>,
    body: web::Json<GetArticleVersionRequest>,
) -> HttpResponse {
    let result = sqlx::query_as::<_, (i32, sqlx::types::chrono::NaiveDateTime, String, String, i32)>(
        "SELECT Version, Date, Title, Content, PageViews FROM article WHERE Id = ? ORDER BY Version DESC"
    )
    .bind(&body.Id)
    .fetch_all(&data.db)
    .await;

    match result {
        Ok(rows) => {
            let versions: Vec<ArticleVersionInfo> = rows.into_iter().map(|row| {
                let (version, date, title, content, page_views) = row;
                let truncated_content = if content.chars().count() > 5 {
                    format!("{}...", content.chars().take(5).collect::<String>())
                } else {
                    content
                };
                let date_str = date.format("%Y-%m-%d %H:%M:%S").to_string();
                ArticleVersionInfo {
                    Version: version,
                    Date: date_str,
                    Title: title,
                    Content: truncated_content,
                    PageViews: page_views,
                }
            }).collect();

            HttpResponse::Ok().json(GetArticleVersionResponse {
                Status: "True".to_string(),
                ArticleInfo: Some(versions),
            })
        }
        Err(_) => HttpResponse::Ok().json(GetArticleVersionResponse {
            Status: "False".to_string(),
            ArticleInfo: None,
        }),
    }
}

// 修改文章（创建新版本）
async fn change_article(
    data: web::Data<AppState>,
    req: HttpRequest,
    body: web::Json<ChangeArticleRequest>,
) -> HttpResponse {
    // 验证 Token
    let token = match get_token_from_request(&req) {
        Some(t) if t.len() == 20 => t,
        _ => {
            return HttpResponse::Ok().json(ApiResponse {
                Status: "False".to_string(),
                Token: None,
            });
        }
    };

    // 验证 session
    if is_session_expired(&data.db, &token).await {
        return HttpResponse::Ok().json(ApiResponse {
            Status: "False".to_string(),
            Token: None,
        });
    }

    // 获取当前最大 Version
    let max_version_result = sqlx::query_as::<_, (Option<i32>,)>(
        "SELECT MAX(Version) FROM article WHERE Id = ?"
    )
    .bind(&body.Id)
    .fetch_optional(&data.db)
    .await;

    let (new_version, old_version) = match max_version_result {
        Ok(Some((Some(v),))) => (v + 1, Some(v)),
        _ => (1, None), // 如果没有旧版本，设为1
    };

    // 验证 Token 对应的用户名是否与 Sender 匹配
    let user_result = sqlx::query_as::<_, (String,)>(
        "SELECT UserName FROM User WHERE Token = ?"
    )
    .bind(&token)
    .fetch_optional(&data.db)
    .await;

    let token_user_name = match user_result {
        Ok(Some((name,))) => name,
        _ => {
            return HttpResponse::Ok().json(ApiResponse {
                Status: "False".to_string(),
                Token: None,
            });
        }
    };

    if token_user_name != body.Sender {
        return HttpResponse::Ok().json(ApiResponse {
            Status: "False".to_string(),
            Token: None,
        });
    }

    // 输入验证 - 防止 XSS
    if !validate_input(&body.Title) || !validate_input(&body.Content) {
        return HttpResponse::Ok().json(ApiResponse {
            Status: "False".to_string(),
            Token: None,
        });
    }

    // 获取现有文章的 File 字段
    let existing_file: Option<String> = match sqlx::query_as::<_, (Option<String>,)>(
        "SELECT File FROM article WHERE Id = ? LIMIT 1"
    )
    .bind(&body.Id)
    .fetch_optional(&data.db)
    .await
    {
        Ok(Some((file,))) => file,
        _ => None,
    };

    // 复制旧版本目录中的文件到新版本目录（跳过已删除的文件）
    let mut has_remaining_files = false;
    if let Some(old_v) = old_version {
        let old_dir = format!("{}/{}/{}", ARTICLE_FILE_DIR, body.Id, old_v);
        let new_dir = format!("{}/{}/{}", ARTICLE_FILE_DIR, body.Id, new_version);

        // 创建新版本目录
        let _ = std::fs::create_dir_all(&new_dir);

        // 复制文件（跳过已删除的文件）
        if Path::new(&old_dir).exists() {
            if let Ok(entries) = std::fs::read_dir(&old_dir) {
                for entry in entries.flatten() {
                    if let Some(name) = entry.file_name().to_str() {
                        let name_string = name.to_string();
                        // 如果文件在已删除列表中，跳过复制
                        if body.DeletedFiles.contains(&name_string) {
                            continue;
                        }
                        has_remaining_files = true;
                        let src = Path::new(&old_dir).join(&name_string);
                        let dst = Path::new(&new_dir).join(&name_string);
                        let _ = std::fs::copy(&src, &dst);
                    }
                }
            }
        }
    }

    // 如果没有剩余文件，File设为None；否则设为新版本的URL
    let file_value: Option<&str> = if has_remaining_files {
        Some(&format!("/Api/GetArticleFile?Id={}&Version={}", body.Id, new_version))
    } else {
        None
    };

    // 插入新版本文章
    let result = sqlx::query(
        "INSERT INTO article (Id, Sender, Version, Date, Title, Content, PageViews, File) VALUES (?, ?, ?, NOW(), ?, ?, 0, ?)"
    )
    .bind(&body.Id)
    .bind(&body.Sender)
    .bind(new_version)
    .bind(&body.Title)
    .bind(&body.Content)
    .bind(file_value)
    .execute(&data.db)
    .await;

    let _ = update_last_activity(&data.db, &token).await;

    match result {
        Ok(_) => {
            write_log(&data.db, &token_user_name, "Change", &body.Id).await;
            HttpResponse::Ok().json(ChangeArticleResponse {
            Status: "True".to_string(),
            Version: Some(new_version),
        })},
        Err(_) => HttpResponse::Ok().json(ChangeArticleResponse {
            Status: "False".to_string(),
            Version: None,
        }),
    }
}

// 上传文章文件
async fn upload_article_file(
    data: web::Data<AppState>,
    req: HttpRequest,
    mut payload: Multipart,
) -> HttpResponse {
    // 验证 Token
    let token = match get_token_from_request(&req) {
        Some(t) if t.len() == 20 => t,
        _ => {
            return HttpResponse::Ok().json(ApiResponse {
                Status: "False".to_string(),
                Token: None,
            });
        }
    };

    // 验证 session
    if is_session_expired(&data.db, &token).await {
        return HttpResponse::Ok().json(ApiResponse {
            Status: "False".to_string(),
            Token: None,
        });
    }

    let mut article_id: Option<String> = None;
    let mut version: Option<i32> = None;

    // 处理文件上传
    while let Some(item) = payload.next().await {
        let mut field = match item {
            Ok(f) => f,
            Err(_) => continue,
        };

        let content_disposition = match field.content_disposition() {
            Some(cd) => cd,
            None => continue,
        };

        let field_name = content_disposition
            .get_name()
            .unwrap_or_default()
            .to_string();

        // 获取 article_id
        if field_name == "Id" {
            let mut id_data = Vec::new();
            while let Some(chunk_result) = field.next().await {
                match chunk_result {
                    Ok(data) => id_data.extend_from_slice(&data),
                    Err(_) => break,
                }
            }
            if let Ok(id_str) = String::from_utf8(id_data) {
                article_id = Some(id_str.trim().to_string());
            }
            continue;
        }

        // 获取 version
        if field_name == "Version" {
            let mut version_data = Vec::new();
            while let Some(chunk_result) = field.next().await {
                match chunk_result {
                    Ok(data) => version_data.extend_from_slice(&data),
                    Err(_) => break,
                }
            }
            if let Ok(version_str) = String::from_utf8(version_data) {
                version = version_str.trim().parse::<i32>().ok();
            }
            continue;
        }

        // 处理文件字段
        if field_name == "file" {
            if article_id.is_none() || version.is_none() {
                continue;
            }

            // 创建版本目录: {ARTICLE_FILE_DIR}/{Id}/{Version}/
            let article_dir = format!("{}/{}/{}", ARTICLE_FILE_DIR, article_id.as_ref().unwrap(), version.unwrap());

            // 创建目录
            if let Err(_) = std::fs::create_dir_all(&article_dir) {
                continue;
            }

            let filename = content_disposition
                .get_filename()
                .unwrap_or_default()
                .to_string();

            // 验证文件名
            if filename.is_empty() || filename.len() > 255 {
                continue;
            }

            // 读取文件内容
            let mut file_data: Vec<u8> = Vec::new();
            while let Some(chunk_result) = field.next().await {
                match chunk_result {
                    Ok(data) => {
                        file_data.extend_from_slice(&data);
                        if file_data.len() > MAX_FILE_SIZE {
                            return HttpResponse::Ok().json(ApiResponse {
                                Status: "False".to_string(),
                                Token: None,
                            });
                        }
                    }
                    Err(_) => break,
                }
            }

            if file_data.is_empty() || file_data.len() > MAX_FILE_SIZE {
                continue;
            }

            // 写入文件
            let file_path = format!("{}/{}", article_dir, filename);
            match std::fs::File::create(&file_path) {
                Ok(mut file) => {
                    if file.write_all(&file_data).is_err() {
                        continue;
                    }
                }
                Err(_) => continue,
            }
        }
    }

    let _ = update_last_activity(&data.db, &token).await;

    HttpResponse::Ok().json(ApiResponse {
        Status: "True".to_string(),
        Token: None,
    })
}

// 获取文章文件列表或单个文件
async fn get_article_file(req: HttpRequest) -> HttpResponse {
    let query_string = req.query_string();

    // 解析参数
    let mut article_id: Option<String> = None;
    let mut file_name: Option<String> = None;
    let mut version: Option<i32> = None;

    for pair in query_string.split('&') {
        let parts: Vec<&str> = pair.split('=').collect();
        if parts.len() == 2 {
            match parts[0] {
                "Id" => article_id = Some(parts[1].to_string()),
                "FileName" => file_name = Some(urlencoding::decode(parts[1]).unwrap().to_string()),
                "Version" => version = parts[1].parse::<i32>().ok(),
                _ => {}
            }
        }
    }

    let article_id = match article_id {
        Some(id) => id,
        None => {
            return HttpResponse::Ok().json(ApiResponse {
                Status: "False".to_string(),
                Token: None,
            });
        }
    };

    // 如果没有指定Version，使用版本目录
    let article_dir = if let Some(v) = version {
        format!("{}/{}/{}", ARTICLE_FILE_DIR, article_id, v)
    } else {
        format!("{}/{}", ARTICLE_FILE_DIR, article_id)
    };

    // 检查目录是否存在
    if !Path::new(&article_dir).exists() {
        return HttpResponse::Ok().json(ApiResponse {
            Status: "False".to_string(),
            Token: None,
        });
    }

    // 如果有 FileName 参数，返回单个文件
    if let Some(filename) = file_name {
        let file_path = format!("{}/{}", article_dir, filename);
        if !Path::new(&file_path).exists() {
            return HttpResponse::Ok().json(ApiResponse {
                Status: "False".to_string(),
                Token: None,
            });
        }

        // 根据扩展名确定内容类型
        let content_type = match Path::new(&filename)
            .extension()
            .and_then(|ext| ext.to_str())
        {
            Some("pdf") => "application/pdf",
            Some("doc") | Some("docx") => "application/msword",
            Some("jpg") | Some("jpeg") => "image/jpeg",
            Some("png") => "image/png",
            Some("gif") => "image/gif",
            Some("txt") => "text/plain",
            _ => "application/octet-stream",
        };

        match std::fs::read(&file_path) {
            Ok(data) => HttpResponse::Ok()
                .content_type(content_type)
                .body(data),
            Err(_) => HttpResponse::Ok().json(ApiResponse {
                Status: "False".to_string(),
                Token: None,
            }),
        }
    } else {
        // 返回文件列表
        match std::fs::read_dir(&article_dir) {
            Ok(entries) => {
                let mut files: Vec<String> = Vec::new();
                for entry in entries.flatten() {
                    if let Some(name) = entry.file_name().to_str() {
                        files.push(name.to_string());
                    }
                }

                let mut response_map = serde_json::Map::new();
                response_map.insert("Status".to_string(), serde_json::Value::String("True".to_string()));
                for (i, file) in files.iter().enumerate() {
                    response_map.insert(
                        format!("File{}", i + 1),
                        serde_json::Value::String(file.clone()),
                    );
                }

                HttpResponse::Ok().json(serde_json::Value::Object(response_map))
            }
            Err(_) => HttpResponse::Ok().json(ApiResponse {
                Status: "False".to_string(),
                Token: None,
            }),
        }
    }
}

// 生成43位 MCP Key (sk- + 40位随机字符)
fn generate_mcp_key() -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = rand::thread_rng();
    let random_part: String = (0..40)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    format!("sk-{}", random_part)
}

// 脱敏 MCP Key
fn mask_mcp_key(key: &str) -> String {
    if key.starts_with("sk-") && key.len() > 8 {
        format!("sk-{}...{}", &key[3..7], &key[key.len()-4..])
    } else {
        key.to_string()
    }
}

// 写入操作日志
async fn write_log(db: &MySqlPool, operator: &str, action: &str, object: &str) {
    let _ = sqlx::query("INSERT INTO log (Date, Operator, Action, Object) VALUES (NOW(), ?, ?, ?)")
        .bind(operator)
        .bind(action)
        .bind(object)
        .execute(db)
        .await;
}

// 添加 MCP Key
async fn admin_add_mcp_key(
    data: web::Data<AppState>,
    req: HttpRequest,
    body: web::Json<AddMcpKeyRequest>,
) -> HttpResponse {
    let token = match get_token_from_request(&req) {
        Some(t) if t.len() == 20 => t,
        _ => {
            return HttpResponse::Ok().json(AddMcpKeyResponse {
                Status: "False".to_string(),
                Key: None,
            });
        }
    };

    if is_session_expired(&data.db, &token).await {
        return HttpResponse::Ok().json(AddMcpKeyResponse {
            Status: "False".to_string(),
            Key: None,
        });
    }

    let mcp_key = generate_mcp_key();

    let result = sqlx::query(
        "INSERT INTO mcp (Date, `Key`, Auth_Read, Auth_Add, Auth_Change, Auth_Remove) VALUES (NOW(), ?, ?, ?, ?, ?)"
    )
    .bind(&mcp_key)
    .bind(body.Auth_Read)
    .bind(body.Auth_Add)
    .bind(body.Auth_Change)
    .bind(body.Auth_Remove)
    .execute(&data.db)
    .await;

    let _ = update_last_activity(&data.db, &token).await;

    match result {
        Ok(_) => HttpResponse::Ok().json(AddMcpKeyResponse {
            Status: "True".to_string(),
            Key: Some(mcp_key),
        }),
        Err(_) => HttpResponse::Ok().json(AddMcpKeyResponse {
            Status: "False".to_string(),
            Key: None,
        }),
    }
}

// 修改 MCP Key
async fn admin_change_mcp_key(
    data: web::Data<AppState>,
    req: HttpRequest,
    body: web::Json<ChangeMcpKeyRequest>,
) -> HttpResponse {
    let token = match get_token_from_request(&req) {
        Some(t) if t.len() == 20 => t,
        _ => {
            return HttpResponse::Ok().json(ApiResponse {
                Status: "False".to_string(),
                Token: None,
            });
        }
    };

    if is_session_expired(&data.db, &token).await {
        return HttpResponse::Ok().json(ApiResponse {
            Status: "False".to_string(),
            Token: None,
        });
    }

    let result = sqlx::query(
        "UPDATE mcp SET Auth_Read = ?, Auth_Add = ?, Auth_Change = ?, Auth_Remove = ? WHERE `Key` = ?"
    )
    .bind(body.Auth_Read)
    .bind(body.Auth_Add)
    .bind(body.Auth_Change)
    .bind(body.Auth_Remove)
    .bind(&body.Key)
    .execute(&data.db)
    .await;

    let _ = update_last_activity(&data.db, &token).await;

    match result {
        Ok(_) => HttpResponse::Ok().json(ApiResponse {
            Status: "True".to_string(),
            Token: None,
        }),
        Err(_) => HttpResponse::Ok().json(ApiResponse {
            Status: "False".to_string(),
            Token: None,
        }),
    }
}

// 清空日志
async fn admin_delete_log(
    data: web::Data<AppState>,
    req: HttpRequest,
) -> HttpResponse {
    let token = match get_token_from_request(&req) {
        Some(t) if t.len() == 20 => t,
        _ => {
            return HttpResponse::Ok().json(ApiResponse {
                Status: "False".to_string(),
                Token: None,
            });
        }
    };

    if is_session_expired(&data.db, &token).await {
        return HttpResponse::Ok().json(ApiResponse {
            Status: "False".to_string(),
            Token: None,
        });
    }

    let result = sqlx::query("DELETE FROM log")
        .execute(&data.db)
        .await;

    let _ = update_last_activity(&data.db, &token).await;

    match result {
        Ok(_) => HttpResponse::Ok().json(ApiResponse {
            Status: "True".to_string(),
            Token: None,
        }),
        Err(_) => HttpResponse::Ok().json(ApiResponse {
            Status: "False".to_string(),
            Token: None,
        }),
    }
}

// 获取日志数量
async fn get_log_number(
    data: web::Data<AppState>,
    req: HttpRequest,
) -> HttpResponse {
    let token = match get_token_from_request(&req) {
        Some(t) if t.len() == 20 => t,
        _ => {
            return HttpResponse::Ok().json(GetLogNumberResponse {
                Status: "False".to_string(),
                Number: None,
            });
        }
    };

    if is_session_expired(&data.db, &token).await {
        return HttpResponse::Ok().json(GetLogNumberResponse {
            Status: "False".to_string(),
            Number: None,
        });
    }

    let result = sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM log")
        .fetch_one(&data.db)
        .await;

    let _ = update_last_activity(&data.db, &token).await;

    match result {
        Ok((count,)) => HttpResponse::Ok().json(GetLogNumberResponse {
            Status: "True".to_string(),
            Number: Some(count),
        }),
        Err(_) => HttpResponse::Ok().json(GetLogNumberResponse {
            Status: "False".to_string(),
            Number: None,
        }),
    }
}

// 获取日志列表
async fn get_log_info(
    data: web::Data<AppState>,
    req: HttpRequest,
    body: web::Json<GetLogInfoRequest>,
) -> HttpResponse {
    let token = match get_token_from_request(&req) {
        Some(t) if t.len() == 20 => t,
        _ => {
            return HttpResponse::Ok().json(GetLogInfoResponse {
                Status: "False".to_string(),
                LogInfo: None,
            });
        }
    };

    if is_session_expired(&data.db, &token).await {
        return HttpResponse::Ok().json(GetLogInfoResponse {
            Status: "False".to_string(),
            LogInfo: None,
        });
    }

    let page = if body.Page <= 0 { 1 } else { body.Page };
    let offset = (page - 1) * 3;

    let result = sqlx::query_as::<_, (sqlx::types::chrono::NaiveDateTime, String, String, String)>(
        "SELECT Date, Operator, Action, Object FROM log ORDER BY Date DESC LIMIT 3 OFFSET ?"
    )
    .bind(offset)
    .fetch_all(&data.db)
    .await;

    let _ = update_last_activity(&data.db, &token).await;

    match result {
        Ok(rows) => {
            let logs: Vec<LogInfo> = rows.into_iter().map(|row| {
                let (date, operator, action, object) = row;
                LogInfo {
                    Date: date.format("%Y-%m-%d %H:%M:%S").to_string(),
                    Operator: mask_mcp_key(&operator),
                    Action: action,
                    Object: object,
                }
            }).collect();

            HttpResponse::Ok().json(GetLogInfoResponse {
                Status: "True".to_string(),
                LogInfo: Some(logs),
            })
        }
        Err(_) => HttpResponse::Ok().json(GetLogInfoResponse {
            Status: "False".to_string(),
            LogInfo: None,
        }),
    }
}

// 获取 MCP Key 列表
async fn admin_get_mcp_keys(
    data: web::Data<AppState>,
    req: HttpRequest,
    body: web::Json<GetMcpKeyInfoRequest>,
) -> HttpResponse {
    let token = match get_token_from_request(&req) {
        Some(t) if t.len() == 20 => t,
        _ => {
            return HttpResponse::Ok().json(GetMcpKeysResponse {
                Status: "False".to_string(),
                McpKeys: None,
            });
        }
    };

    if is_session_expired(&data.db, &token).await {
        return HttpResponse::Ok().json(GetMcpKeysResponse {
            Status: "False".to_string(),
            McpKeys: None,
        });
    }

    let page = if body.Page <= 0 { 1 } else { body.Page };
    let offset = (page - 1) * 3;

    let result = sqlx::query_as::<_, (sqlx::types::chrono::NaiveDateTime, String, i32, i32, i32, i32)>(
        "SELECT Date, `Key`, Auth_Read, Auth_Add, Auth_Change, Auth_Remove FROM mcp ORDER BY Date DESC LIMIT 3 OFFSET ?"
    )
    .bind(offset)
    .fetch_all(&data.db)
    .await;

    let _ = update_last_activity(&data.db, &token).await;

    match result {
        Ok(rows) => {
            let keys: Vec<McpKeyInfo> = rows.into_iter().map(|row| {
                let (date, key, auth_read, auth_add, auth_change, auth_remove) = row;
                McpKeyInfo {
                    Date: date.format("%Y-%m-%d %H:%M:%S").to_string(),
                    Key: key,
                    Auth_Read: auth_read,
                    Auth_Add: auth_add,
                    Auth_Change: auth_change,
                    Auth_Remove: auth_remove,
                }
            }).collect();

            HttpResponse::Ok().json(GetMcpKeysResponse {
                Status: "True".to_string(),
                McpKeys: Some(keys),
            })
        }
        Err(_) => HttpResponse::Ok().json(GetMcpKeysResponse {
            Status: "False".to_string(),
            McpKeys: None,
        }),
    }
}

// 获取 MCP Key 数量
async fn admin_get_mcp_key_number(
    data: web::Data<AppState>,
    req: HttpRequest,
) -> HttpResponse {
    let token = match get_token_from_request(&req) {
        Some(t) if t.len() == 20 => t,
        _ => {
            return HttpResponse::Ok().json(GetMcpKeyNumberResponse {
                Status: "False".to_string(),
                Number: None,
            });
        }
    };

    if is_session_expired(&data.db, &token).await {
        return HttpResponse::Ok().json(GetMcpKeyNumberResponse {
            Status: "False".to_string(),
            Number: None,
        });
    }

    let result = sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM mcp")
        .fetch_one(&data.db)
        .await;

    let _ = update_last_activity(&data.db, &token).await;

    match result {
        Ok((count,)) => HttpResponse::Ok().json(GetMcpKeyNumberResponse {
            Status: "True".to_string(),
            Number: Some(count),
        }),
        Err(_) => HttpResponse::Ok().json(GetMcpKeyNumberResponse {
            Status: "False".to_string(),
            Number: None,
        }),
    }
}

// 删除 MCP Key
async fn admin_delete_mcp_key(
    data: web::Data<AppState>,
    req: HttpRequest,
    body: web::Json<DeleteMcpKeyRequest>,
) -> HttpResponse {
    let token = match get_token_from_request(&req) {
        Some(t) if t.len() == 20 => t,
        _ => {
            return HttpResponse::Ok().json(ApiResponse {
                Status: "False".to_string(),
                Token: None,
            });
        }
    };

    if is_session_expired(&data.db, &token).await {
        return HttpResponse::Ok().json(ApiResponse {
            Status: "False".to_string(),
            Token: None,
        });
    }

    let result = sqlx::query("DELETE FROM mcp WHERE `Key` = ?")
        .bind(&body.Key)
        .execute(&data.db)
        .await;

    let _ = update_last_activity(&data.db, &token).await;

    match result {
        Ok(_) => HttpResponse::Ok().json(ApiResponse {
            Status: "True".to_string(),
            Token: None,
        }),
        Err(_) => HttpResponse::Ok().json(ApiResponse {
            Status: "False".to_string(),
            Token: None,
        }),
    }
}

// 辅助函数：从请求中获取MCP Key
fn get_mcp_key_from_request(req: &HttpRequest) -> Option<String> {
    req.headers()
        .get("X-MCP-Key")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .or_else(|| get_token_from_request(req))
}

async fn validate_mcp_key(data: &web::Data<AppState>, mcp_key: &str) -> bool {
    if !mcp_key.starts_with("sk-") {
        return false;
    }
    let result = sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM mcp WHERE `Key` = ?")
        .bind(mcp_key)
        .fetch_optional(&data.db)
        .await;
    matches!(result, Ok(Some((1,))))
}

async fn check_mcp_remove_auth(data: &web::Data<AppState>, mcp_key: &str) -> bool {
    let result = sqlx::query_as::<_, (i32,)>("SELECT Auth_Remove FROM mcp WHERE `Key` = ?")
        .bind(mcp_key)
        .fetch_optional(&data.db)
        .await;
    matches!(result, Ok(Some((1,))))
}

async fn check_mcp_add_auth(data: &web::Data<AppState>, mcp_key: &str) -> bool {
    let result = sqlx::query_as::<_, (i32,)>("SELECT Auth_Add FROM mcp WHERE `Key` = ?")
        .bind(mcp_key)
        .fetch_optional(&data.db)
        .await;
    matches!(result, Ok(Some((1,))))
}

async fn check_mcp_change_auth(data: &web::Data<AppState>, mcp_key: &str) -> bool {
    let result = sqlx::query_as::<_, (i32,)>("SELECT Auth_Change FROM mcp WHERE `Key` = ?")
        .bind(mcp_key)
        .fetch_optional(&data.db)
        .await;
    matches!(result, Ok(Some((1,))))
}

// MCP 获取文章信息列表
async fn mcp_get_article_info_list(
    data: web::Data<AppState>,
    req: HttpRequest,
    body: web::Json<GetArticleInfoListRequest>,
) -> HttpResponse {
    let mcp_key = match get_mcp_key_from_request(&req) {
        Some(k) => k,
        _ => return HttpResponse::Ok().json(GetArticleInfoListResponse { Status: "False".to_string(), InfoList: None }),
    };

    if !validate_mcp_key(&data, &mcp_key).await {
        return HttpResponse::Ok().json(GetArticleInfoListResponse { Status: "False".to_string(), InfoList: None });
    }

    let number = if body.Number <= 0 { 10 } else { body.Number };

    let result = if let Some(ref keyword) = body.KeyWord {
        if keyword.is_empty() {
            sqlx::query_as::<_, (String, String, i32, sqlx::types::chrono::NaiveDateTime, String, String, Option<String>)>(
                "SELECT a.Id, a.Sender, a.Version, a.Date, a.Title, a.Content, a.File FROM article a
                 INNER JOIN (
                     SELECT Id, MAX(Version) as MaxVersion FROM article GROUP BY Id
                 ) b ON a.Id = b.Id AND a.Version = b.MaxVersion
                 ORDER BY a.Date DESC LIMIT ?"
            )
            .bind(number)
            .fetch_all(&data.db)
            .await
        } else {
            let pattern = format!("%{}%", keyword);
            sqlx::query_as::<_, (String, String, i32, sqlx::types::chrono::NaiveDateTime, String, String, Option<String>)>(
                "SELECT a.Id, a.Sender, a.Version, a.Date, a.Title, a.Content, a.File FROM article a
                 INNER JOIN (
                     SELECT Id, MAX(Version) as MaxVersion FROM article GROUP BY Id
                 ) b ON a.Id = b.Id AND a.Version = b.MaxVersion
                 WHERE a.Title LIKE ? OR a.Content LIKE ?
                 ORDER BY a.Date DESC LIMIT ?"
            )
            .bind(&pattern)
            .bind(&pattern)
            .bind(number)
            .fetch_all(&data.db)
            .await
        }
    } else {
        sqlx::query_as::<_, (String, String, i32, sqlx::types::chrono::NaiveDateTime, String, String, Option<String>)>(
            "SELECT a.Id, a.Sender, a.Version, a.Date, a.Title, a.Content, a.File FROM article a
             INNER JOIN (
                 SELECT Id, MAX(Version) as MaxVersion FROM article GROUP BY Id
             ) b ON a.Id = b.Id AND a.Version = b.MaxVersion
             ORDER BY a.Date DESC LIMIT ?"
        )
        .bind(number)
        .fetch_all(&data.db)
        .await
    };

    match result {
        Ok(rows) => {
            let info_list: Vec<ArticleInfoListItem> = rows.into_iter().map(|row| {
                let (id, sender, version, _date, title, content, file) = row;
                let truncated_content = if content.chars().count() > 5 {
                    format!("{}...", content.chars().take(5).collect::<String>())
                } else {
                    content
                };
                ArticleInfoListItem {
                    Id: id,
                    Sender: mask_mcp_key(&sender),
                    Title: title,
                    Content: truncated_content,
                    Version: version,
                    File: file.is_some(),
                }
            }).collect();

            HttpResponse::Ok().json(GetArticleInfoListResponse {
                Status: "True".to_string(),
                InfoList: Some(info_list),
            })
        }
        Err(_) => HttpResponse::Ok().json(GetArticleInfoListResponse { Status: "False".to_string(), InfoList: None }),
    }
}

// MCP 获取文章所有版本列表
async fn mcp_get_article_all_version_list(
    data: web::Data<AppState>,
    req: HttpRequest,
    body: web::Json<GetArticleAllVersionListRequest>,
) -> HttpResponse {
    let mcp_key = match get_mcp_key_from_request(&req) {
        Some(k) => k,
        _ => return HttpResponse::Ok().json(GetArticleAllVersionListResponse { Status: "False".to_string(), VersionList: None }),
    };

    if !validate_mcp_key(&data, &mcp_key).await {
        return HttpResponse::Ok().json(GetArticleAllVersionListResponse { Status: "False".to_string(), VersionList: None });
    }

    let result = sqlx::query_as::<_, (i32, String, String, String, Option<String>)>(
        "SELECT Version, Sender, Title, Content, File FROM article WHERE Id = ? ORDER BY Version DESC"
    )
    .bind(&body.Id)
    .fetch_all(&data.db)
    .await;

    match result {
        Ok(rows) => {
            let version_list: Vec<ArticleVersionListItem> = rows.into_iter().map(|row| {
                let (version, sender, title, content, file) = row;
                let truncated_content = if content.chars().count() > 5 {
                    format!("{}...", content.chars().take(5).collect::<String>())
                } else {
                    content
                };
                ArticleVersionListItem {
                    Version: version,
                    Sender: mask_mcp_key(&sender),
                    Title: title,
                    Content: truncated_content,
                    File: file.is_some(),
                }
            }).collect();

            HttpResponse::Ok().json(GetArticleAllVersionListResponse {
                Status: "True".to_string(),
                VersionList: Some(version_list),
            })
        }
        Err(_) => HttpResponse::Ok().json(GetArticleAllVersionListResponse { Status: "False".to_string(), VersionList: None }),
    }
}

// MCP 获取文章数量
async fn mcp_get_article_number(
    data: web::Data<AppState>,
    req: HttpRequest,
) -> HttpResponse {
    let mcp_key = match get_mcp_key_from_request(&req) {
        Some(k) => k,
        _ => return HttpResponse::Ok().json(ArticleNumberResponse { Status: "False".to_string(), Number: None }),
    };

    if !validate_mcp_key(&data, &mcp_key).await {
        return HttpResponse::Ok().json(ArticleNumberResponse { Status: "False".to_string(), Number: None });
    }

    let query_string = req.query_string();
    let keyword = query_string.split('&')
        .find(|pair| pair.starts_with("KeyWord="))
        .and_then(|pair| pair.split('=').nth(1))
        .map(urlencoding::decode)
        .and_then(|d| d.ok())
        .map(|s| s.to_string());

    let result = if let Some(ref kw) = keyword {
        if kw.is_empty() {
            sqlx::query_as::<_, (i64,)>(
                "SELECT COUNT(*) FROM article a WHERE Version = (SELECT MAX(Version) FROM article WHERE Id = a.Id)"
            )
            .fetch_one(&data.db)
            .await
        } else {
            let pattern = format!("%{}%", kw);
            sqlx::query_as::<_, (i64,)>(
                "SELECT COUNT(*) FROM article a WHERE Version = (SELECT MAX(Version) FROM article WHERE Id = a.Id) AND (Title LIKE ? OR Content LIKE ?)"
            )
            .bind(&pattern)
            .bind(&pattern)
            .fetch_one(&data.db)
            .await
        }
    } else {
        sqlx::query_as::<_, (i64,)>(
            "SELECT COUNT(*) FROM article a WHERE Version = (SELECT MAX(Version) FROM article WHERE Id = a.Id)"
        )
        .fetch_one(&data.db)
        .await
    };

    match result {
        Ok((count,)) => HttpResponse::Ok().json(ArticleNumberResponse {
            Status: "True".to_string(),
            Number: Some(count),
        }),
        Err(_) => HttpResponse::Ok().json(ArticleNumberResponse { Status: "False".to_string(), Number: None }),
    }
}

// MCP 获取文章完整内容
async fn mcp_get_article_full(
    data: web::Data<AppState>,
    req: HttpRequest,
    body: web::Json<McpGetArticleFullRequest>,
) -> HttpResponse {
    let mcp_key = match get_mcp_key_from_request(&req) {
        Some(k) => k,
        _ => return HttpResponse::Ok().json(McpGetArticleFullResponse { Status: "False".to_string(), Title: None, Content: None, FileList: None }),
    };

    if !validate_mcp_key(&data, &mcp_key).await {
        return HttpResponse::Ok().json(McpGetArticleFullResponse { Status: "False".to_string(), Title: None, Content: None, FileList: None });
    }

    let _ = sqlx::query("UPDATE article SET PageViews = PageViews + 1 WHERE Id = ? AND Version = ?")
        .bind(&body.Id)
        .bind(body.Version)
        .execute(&data.db)
        .await;

    let result = sqlx::query_as::<_, (String, String, Option<String>)>(
        "SELECT Title, Content, File FROM article WHERE Id = ? AND Version = ?"
    )
    .bind(&body.Id)
    .bind(body.Version)
    .fetch_optional(&data.db)
    .await;

    match result {
        Ok(Some((title, content, _file))) => {
            let mut file_list: Vec<String> = Vec::new();
            let article_dir = format!("{}/{}/{}", ARTICLE_FILE_DIR, body.Id, body.Version);
            if let Ok(entries) = std::fs::read_dir(&article_dir) {
                for entry in entries.flatten() {
                    if let Some(name) = entry.file_name().to_str() {
                        let file_url = format!("http://127.0.0.1:8080/Api/GetArticleFile?Id={}&Version={}&FileName={}", body.Id, body.Version, urlencoding::encode(name));
                        file_list.push(file_url);
                    }
                }
            }
            HttpResponse::Ok().json(McpGetArticleFullResponse {
                Status: "True".to_string(),
                Title: Some(title),
                Content: Some(content),
                FileList: Some(file_list),
            })
        }
        _ => HttpResponse::Ok().json(McpGetArticleFullResponse { Status: "False".to_string(), Title: None, Content: None, FileList: None }),
    }
}

// MCP 删除文章
async fn mcp_delete_article(
    data: web::Data<AppState>,
    req: HttpRequest,
    body: web::Json<McpDeleteArticleRequest>,
) -> HttpResponse {
    let mcp_key = match get_mcp_key_from_request(&req) {
        Some(k) => k,
        _ => return HttpResponse::Ok().json(McpDeleteArticleResponse { Status: "False".to_string(), Auth: false }),
    };

    if !validate_mcp_key(&data, &mcp_key).await {
        return HttpResponse::Ok().json(McpDeleteArticleResponse { Status: "False".to_string(), Auth: false });
    }

    if !check_mcp_remove_auth(&data, &mcp_key).await {
        return HttpResponse::Ok().json(McpDeleteArticleResponse { Status: "False".to_string(), Auth: false });
    }

    let result = sqlx::query("DELETE FROM article WHERE Id = ? AND Version = ?")
        .bind(&body.Id)
        .bind(body.Version)
        .execute(&data.db)
        .await;

    match result {
        Ok(_) => {
            write_log(&data.db, &mcp_key, "Remove", &body.Id).await;
            HttpResponse::Ok().json(McpDeleteArticleResponse { Status: "True".to_string(), Auth: true })},
        Err(_) => HttpResponse::Ok().json(McpDeleteArticleResponse { Status: "False".to_string(), Auth: true }),
    }
}

// MCP 添加文章
async fn mcp_add_article(
    data: web::Data<AppState>,
    req: HttpRequest,
    body: web::Json<McpAddArticleRequest>,
) -> HttpResponse {
    let mcp_key = match get_mcp_key_from_request(&req) {
        Some(k) => k,
        _ => return HttpResponse::Ok().json(McpAddArticleResponse { Status: "False".to_string(), Auth: false, ArticleId: None }),
    };

    if !validate_mcp_key(&data, &mcp_key).await {
        return HttpResponse::Ok().json(McpAddArticleResponse { Status: "False".to_string(), Auth: false, ArticleId: None });
    }

    if !check_mcp_add_auth(&data, &mcp_key).await {
        return HttpResponse::Ok().json(McpAddArticleResponse { Status: "False".to_string(), Auth: false, ArticleId: None });
    }

    let article_id = generate_unique_id(&data.db).await;

    let result = sqlx::query(
        "INSERT INTO article (Id, Sender, Version, Date, Title, Content, PageViews) VALUES (?, ?, 1, NOW(), ?, ?, 0)"
    )
    .bind(&article_id)
    .bind(&mcp_key)
    .bind(&body.Title)
    .bind(&body.Content)
    .execute(&data.db)
    .await;

    match result {
        Ok(_) => {
            write_log(&data.db, &mcp_key, "Add", &article_id).await;
            HttpResponse::Ok().json(McpAddArticleResponse {
            Status: "True".to_string(),
            Auth: true,
            ArticleId: Some(article_id),
        })},
        Err(_) => HttpResponse::Ok().json(McpAddArticleResponse { Status: "False".to_string(), Auth: true, ArticleId: None }),
    }
}

// MCP 上传文章文件
async fn mcp_upload_article_file(
    data: web::Data<AppState>,
    req: HttpRequest,
    mut payload: Multipart,
) -> HttpResponse {
    let mcp_key = match get_mcp_key_from_request(&req) {
        Some(k) => k,
        _ => return HttpResponse::Ok().json(McpFileUploadResponse { Status: "False".to_string(), Auth: false }),
    };

    if !validate_mcp_key(&data, &mcp_key).await {
        return HttpResponse::Ok().json(McpFileUploadResponse { Status: "False".to_string(), Auth: false });
    }

    if !check_mcp_add_auth(&data, &mcp_key).await {
        return HttpResponse::Ok().json(McpFileUploadResponse { Status: "False".to_string(), Auth: false });
    }

    let mut article_id: Option<String> = None;
    let mut version: Option<i32> = None;

    while let Some(item) = payload.next().await {
        let mut field = match item {
            Ok(f) => f,
            Err(_) => continue,
        };

        let content_disposition = match field.content_disposition() {
            Some(cd) => cd,
            None => continue,
        };

        let field_name = content_disposition
            .get_name()
            .unwrap_or_default()
            .to_string();

        if field_name == "Id" {
            let mut id_data = Vec::new();
            while let Some(chunk_result) = field.next().await {
                match chunk_result {
                    Ok(data) => id_data.extend_from_slice(&data),
                    Err(_) => break,
                }
            }
            if let Ok(id_str) = String::from_utf8(id_data) {
                article_id = Some(id_str.trim().to_string());
            }
            continue;
        }

        if field_name == "Version" {
            let mut version_data = Vec::new();
            while let Some(chunk_result) = field.next().await {
                match chunk_result {
                    Ok(data) => version_data.extend_from_slice(&data),
                    Err(_) => break,
                }
            }
            if let Ok(version_str) = String::from_utf8(version_data) {
                version = version_str.trim().parse::<i32>().ok();
            }
            continue;
        }

        if field_name == "file" {
            if article_id.is_none() || version.is_none() {
                continue;
            }

            let article_dir = format!("{}/{}/{}", ARTICLE_FILE_DIR, article_id.as_ref().unwrap(), version.unwrap());

            if let Err(_) = std::fs::create_dir_all(&article_dir) {
                continue;
            }

            let filename = content_disposition
                .get_filename()
                .unwrap_or_default()
                .to_string();

            if filename.is_empty() || filename.len() > 255 {
                continue;
            }

            let mut file_data: Vec<u8> = Vec::new();
            while let Some(chunk_result) = field.next().await {
                match chunk_result {
                    Ok(data) => {
                        file_data.extend_from_slice(&data);
                        if file_data.len() > MAX_FILE_SIZE {
                            return HttpResponse::Ok().json(McpFileUploadResponse { Status: "False".to_string(), Auth: true });
                        }
                    }
                    Err(_) => break,
                }
            }

            if file_data.is_empty() || file_data.len() > MAX_FILE_SIZE {
                continue;
            }

            let file_path = format!("{}/{}", article_dir, filename);
            match std::fs::File::create(&file_path) {
                Ok(mut file) => {
                    if file.write_all(&file_data).is_err() {
                        continue;
                    }
                }
                Err(_) => continue,
            }
        }
    }

    // Update article File field after upload
    if let (Some(id), Some(ver)) = (&article_id, &version) {
        let file_url = format!("/Api/GetArticleFile?Id={}&Version={}", id, ver);
        let _ = sqlx::query("UPDATE article SET File = ? WHERE Id = ? AND Version = ?")
            .bind(&file_url)
            .bind(id)
            .bind(ver)
            .execute(&data.db)
            .await;
    }

    HttpResponse::Ok().json(McpFileUploadResponse { Status: "True".to_string(), Auth: true })
}

// MCP 修改文章（创建新版本）
async fn mcp_change_article(
    data: web::Data<AppState>,
    req: HttpRequest,
    body: web::Json<McpChangeArticleRequest>,
) -> HttpResponse {
    let mcp_key = match get_mcp_key_from_request(&req) {
        Some(k) => k,
        _ => return HttpResponse::Ok().json(McpChangeArticleResponse { Status: "False".to_string(), Auth: false, Version: None }),
    };

    if !validate_mcp_key(&data, &mcp_key).await {
        return HttpResponse::Ok().json(McpChangeArticleResponse { Status: "False".to_string(), Auth: false, Version: None });
    }

    if !check_mcp_change_auth(&data, &mcp_key).await {
        return HttpResponse::Ok().json(McpChangeArticleResponse { Status: "False".to_string(), Auth: false, Version: None });
    }

    // 获取当前最大 Version
    let max_version_result = sqlx::query_as::<_, (Option<i32>,)>(
        "SELECT MAX(Version) FROM article WHERE Id = ?"
    )
    .bind(&body.Id)
    .fetch_optional(&data.db)
    .await;

    let new_version = match max_version_result {
        Ok(Some((Some(v),))) => v + 1,
        _ => 1,
    };

    let result = sqlx::query(
        "INSERT INTO article (Id, Sender, Version, Date, Title, Content, PageViews) VALUES (?, ?, ?, NOW(), ?, ?, 0)"
    )
    .bind(&body.Id)
    .bind(&mcp_key)
    .bind(new_version)
    .bind(&body.Title)
    .bind(&body.Content)
    .execute(&data.db)
    .await;

    match result {
        Ok(_) => {
            write_log(&data.db, &mcp_key, "Change", &body.Id).await;
            HttpResponse::Ok().json(McpChangeArticleResponse {
            Status: "True".to_string(),
            Auth: true,
            Version: Some(new_version),
        })},
        Err(_) => HttpResponse::Ok().json(McpChangeArticleResponse { Status: "False".to_string(), Auth: true, Version: None }),
    }
}

// MCP 修改文章文件
async fn mcp_change_upload_article_file(
    data: web::Data<AppState>,
    req: HttpRequest,
    mut payload: Multipart,
) -> HttpResponse {
    let mcp_key = match get_mcp_key_from_request(&req) {
        Some(k) => k,
        _ => return HttpResponse::Ok().json(McpFileUploadResponse { Status: "False".to_string(), Auth: false }),
    };

    if !validate_mcp_key(&data, &mcp_key).await {
        return HttpResponse::Ok().json(McpFileUploadResponse { Status: "False".to_string(), Auth: false });
    }

    if !check_mcp_change_auth(&data, &mcp_key).await {
        return HttpResponse::Ok().json(McpFileUploadResponse { Status: "False".to_string(), Auth: false });
    }

    let mut article_id: Option<String> = None;
    let mut version: Option<i32> = None;

    while let Some(item) = payload.next().await {
        let mut field = match item {
            Ok(f) => f,
            Err(_) => continue,
        };

        let content_disposition = match field.content_disposition() {
            Some(cd) => cd,
            None => continue,
        };

        let field_name = content_disposition
            .get_name()
            .unwrap_or_default()
            .to_string();

        if field_name == "Id" {
            let mut id_data = Vec::new();
            while let Some(chunk_result) = field.next().await {
                match chunk_result {
                    Ok(data) => id_data.extend_from_slice(&data),
                    Err(_) => break,
                }
            }
            if let Ok(id_str) = String::from_utf8(id_data) {
                article_id = Some(id_str.trim().to_string());
            }
            continue;
        }

        if field_name == "Version" {
            let mut version_data = Vec::new();
            while let Some(chunk_result) = field.next().await {
                match chunk_result {
                    Ok(data) => version_data.extend_from_slice(&data),
                    Err(_) => break,
                }
            }
            if let Ok(version_str) = String::from_utf8(version_data) {
                version = version_str.trim().parse::<i32>().ok();
            }
            continue;
        }

        if field_name == "file" {
            if article_id.is_none() || version.is_none() {
                continue;
            }

            let article_dir = format!("{}/{}/{}", ARTICLE_FILE_DIR, article_id.as_ref().unwrap(), version.unwrap());

            if let Err(_) = std::fs::create_dir_all(&article_dir) {
                continue;
            }

            let filename = content_disposition
                .get_filename()
                .unwrap_or_default()
                .to_string();

            if filename.is_empty() || filename.len() > 255 {
                continue;
            }

            let mut file_data: Vec<u8> = Vec::new();
            while let Some(chunk_result) = field.next().await {
                match chunk_result {
                    Ok(data) => {
                        file_data.extend_from_slice(&data);
                        if file_data.len() > MAX_FILE_SIZE {
                            return HttpResponse::Ok().json(McpFileUploadResponse { Status: "False".to_string(), Auth: true });
                        }
                    }
                    Err(_) => break,
                }
            }

            if file_data.is_empty() || file_data.len() > MAX_FILE_SIZE {
                continue;
            }

            let file_path = format!("{}/{}", article_dir, filename);
            match std::fs::File::create(&file_path) {
                Ok(mut file) => {
                    if file.write_all(&file_data).is_err() {
                        continue;
                    }
                }
                Err(_) => continue,
            }
        }
    }

    // Update article File field after upload
    if let (Some(id), Some(ver)) = (&article_id, &version) {
        let file_url = format!("/Api/GetArticleFile?Id={}&Version={}", id, ver);
        let _ = sqlx::query("UPDATE article SET File = ? WHERE Id = ? AND Version = ?")
            .bind(&file_url)
            .bind(id)
            .bind(ver)
            .execute(&data.db)
            .await;
    }

    HttpResponse::Ok().json(McpFileUploadResponse { Status: "True".to_string(), Auth: true })
}

// MCP Key 可用性检查
async fn get_mcp_key_auth_info(
    data: web::Data<AppState>,
    req: HttpRequest,
) -> HttpResponse {
    let mcp_key = match get_mcp_key_from_request(&req) {
        Some(k) => k,
        _ => {
            return HttpResponse::Ok().json(GetMcpKeyAuthInfoResponse {
                KeyStatus: "False".to_string(),
                ServerStatus: "True".to_string(),
                ReadAuth: None,
                AddAuth: None,
                RemoveAuth: None,
                ChangeAuth: None,
            });
        }
    };

    let result = sqlx::query_as::<_, (i32, i32, i32, i32)>(
        "SELECT Auth_Read, Auth_Add, Auth_Change, Auth_Remove FROM mcp WHERE `Key` = ?"
    )
    .bind(&mcp_key)
    .fetch_optional(&data.db)
    .await;

    match result {
        Ok(Some((auth_read, auth_add, auth_change, auth_remove))) => {
            HttpResponse::Ok().json(GetMcpKeyAuthInfoResponse {
                KeyStatus: "True".to_string(),
                ServerStatus: "True".to_string(),
                ReadAuth: Some(auth_read == 1),
                AddAuth: Some(auth_add == 1),
                RemoveAuth: Some(auth_remove == 1),
                ChangeAuth: Some(auth_change == 1),
            })
        }
        _ => HttpResponse::Ok().json(GetMcpKeyAuthInfoResponse {
            KeyStatus: "False".to_string(),
            ServerStatus: "True".to_string(),
            ReadAuth: None,
            AddAuth: None,
            RemoveAuth: None,
            ChangeAuth: None,
        }),
    }
}

// 更新文章文件路径
async fn update_article_file(
    data: web::Data<AppState>,
    req: HttpRequest,
    body: web::Json<UpdateArticleFileRequest>,
) -> HttpResponse {
    // 验证 Token
    let token = match get_token_from_request(&req) {
        Some(t) if t.len() == 20 => t,
        _ => {
            return HttpResponse::Ok().json(ApiResponse {
                Status: "False".to_string(),
                Token: None,
            });
        }
    };

    // 验证 session
    if is_session_expired(&data.db, &token).await {
        return HttpResponse::Ok().json(ApiResponse {
            Status: "False".to_string(),
            Token: None,
        });
    }

    // 更新文章文件路径
    let file_url = format!("/Api/GetArticleFile?Id={}", body.ArticleId);
    let result = sqlx::query(
        "UPDATE article SET File = ? WHERE Id = ?"
    )
    .bind(&file_url)
    .bind(&body.ArticleId)
    .execute(&data.db)
    .await;

    let _ = update_last_activity(&data.db, &token).await;

    match result {
        Ok(_) => HttpResponse::Ok().json(ApiResponse {
            Status: "True".to_string(),
            Token: None,
        }),
        Err(_) => HttpResponse::Ok().json(ApiResponse {
            Status: "False".to_string(),
            Token: None,
        }),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    // 创建上传目录
    let _ = std::fs::create_dir_all(UPLOAD_DIR);

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "mysql://iblog_user:iblog_user@localhost:3306/iblog_data".to_string());//修改成你的数据库名和数据库账号密码

    let pool = MySqlPool::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    println!("Database connected successfully");

    let db_pool = web::Data::new(AppState { db: pool });

    println!("Server start");
    println!("Upload directory: {}", UPLOAD_DIR);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .supports_credentials()
            .max_age(3600);

        App::new()
            .app_data(db_pool.clone())
            .wrap(cors)
            .route("/Api/Login", web::post().to(login))
            .route("/Api/LogOut", web::post().to(logout))
            .route("/Api/Heartbeat", web::post().to(heartbeat))
            .route("/Api/VerifyToken", web::get().to(verify_token))
            .route("/Api/GetUserInfo", web::get().to(get_user_info))
            .route("/Api/GetShowUserInfo", web::get().to(get_show_user_info))
            .route("/Api/SetUserInfo", web::post().to(set_user_info))
            .route("/Api/ChangePassWord", web::post().to(change_password))
            .route("/Api/UploadAvatar", web::post().to(upload_avatar))
            .route("/Api/GetAvatar", web::get().to(get_avatar))
            .route("/Api/GetAvatarUrlByUserName", web::post().to(get_avatar_url_by_username))
            .route("/Api/AdminAddArticle", web::post().to(admin_add_article))
            .route("/Api/GetArticleNumber", web::post().to(get_article_number))
            .route("/Api/GetTotalPageViews", web::get().to(get_total_page_views))
            .route("/Api/GetNewArticlesPageViewsByTime", web::post().to(get_new_articles_page_views_by_time))
            .route("/Api/GetNewArticlesNumberByTime", web::post().to(get_new_articles_number_by_time))
            .route("/Api/GetArticlesRankList", web::get().to(get_articles_rank_list))
            .route("/Api/GetSiteSetting", web::get().to(get_site_setting))
            .route("/Api/SetSiteSetting", web::post().to(set_site_setting))
            .route("/Api/GetArticleInfo", web::post().to(get_article_info))
            .route("/Api/UploadArticleFile", web::post().to(upload_article_file))
            .route("/Api/GetArticleFile", web::get().to(get_article_file))
            .route("/Api/UpdateArticleFile", web::post().to(update_article_file))
            .route("/Api/DeleteArticle", web::post().to(delete_article))
            .route("/Api/GetArticleFull", web::post().to(get_article_full))
            .route("/Api/GetArticleVersion", web::post().to(get_article_version))
            .route("/Api/AdminChangeArticle", web::post().to(change_article))
            .route("/Api/AdminAddMcpKey", web::post().to(admin_add_mcp_key))
            .route("/Api/AdminChangeMcpKey", web::post().to(admin_change_mcp_key))
            .route("/Api/AdminGetMcpKeys", web::post().to(admin_get_mcp_keys))
            .route("/Api/AdminGetMcpKeyNumber", web::get().to(admin_get_mcp_key_number))
            .route("/Api/AdminDeleteMcpKey", web::post().to(admin_delete_mcp_key))
            .route("/Api/Mcp/GetMcpKeyAuthInfo", web::get().to(get_mcp_key_auth_info))
            .route("/Api/Mcp/GetArticleInfoList", web::post().to(mcp_get_article_info_list))
            .route("/Api/Mcp/GetArticleAllVersionList", web::post().to(mcp_get_article_all_version_list))
            .route("/Api/Mcp/GetArticleFull", web::post().to(mcp_get_article_full))
            .route("/Api/Mcp/GetArticleNumber", web::get().to(mcp_get_article_number))
            .route("/Api/Mcp/DeleteArticle", web::post().to(mcp_delete_article))
            .route("/Api/Mcp/AddArticle", web::post().to(mcp_add_article))
            .route("/Api/Mcp/AddUploadArticleFile", web::post().to(mcp_upload_article_file))
            .route("/Api/Mcp/ChangeArticle", web::post().to(mcp_change_article))
            .route("/Api/Mcp/ChangeUpdateArticleFile", web::post().to(mcp_change_upload_article_file))
            .route("/Api/AdminDeleteLog", web::post().to(admin_delete_log))
            .route("/Api/GetLogNumber", web::get().to(get_log_number))
            .route("/Api/GetLogInfo", web::post().to(get_log_info))
    })
    .bind("127.0.0.1:8080")?// 部署时请将此处修改为实际的后端地址
    .run()
    .await
}