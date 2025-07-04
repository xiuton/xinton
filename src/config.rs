use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub app: AppSettings,
    pub server: ServerSettings,
    pub database: DatabaseSettings,
    pub features: FeatureSettings,
    pub ui: UISettings,
    pub logging: LoggingSettings,
    pub security: SecuritySettings,
    pub email: EmailSettings,
    pub storage: StorageSettings,
    pub analytics: AnalyticsSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub name: String,
    pub version: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerSettings {
    pub host: String,
    pub port: u16,
    pub debug: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseSettings {
    pub url: String,
    pub pool_size: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureSettings {
    pub auth: bool,
    pub api: bool,
    pub websocket: bool,
    pub caching: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UISettings {
    pub theme: String,
    pub language: String,
    pub timezone: String,
}

impl fmt::Display for UISettings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "主题: {}, 语言: {}, 时区: {}", 
            self.theme, self.language, self.timezone)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingSettings {
    pub level: String,
    pub file: String,
    pub max_size: String,
    pub max_files: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecuritySettings {
    pub session_timeout: u32,
    pub max_login_attempts: u32,
    pub password_min_length: u32,
}

impl fmt::Display for AppSettings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "名称: {}, 版本: {}, 描述: {}", 
            self.name, self.version, self.description)
    }
}

impl fmt::Display for ServerSettings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "主机: {}, 端口: {}, 调试: {}", 
            self.host, self.port, if self.debug { "启用" } else { "禁用" })
    }
}

impl fmt::Display for FeatureSettings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "认证: {}, API: {}, WebSocket: {}, 缓存: {}", 
            if self.auth { "启用" } else { "禁用" },
            if self.api { "启用" } else { "禁用" },
            if self.websocket { "启用" } else { "禁用" },
            if self.caching { "启用" } else { "禁用" })
    }
}

impl fmt::Display for SecuritySettings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "会话超时: {}秒, 最大登录尝试: {}, 密码最小长度: {}", 
            self.session_timeout, self.max_login_attempts, self.password_min_length)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailSettings {
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    pub smtp_password: String,
    pub from_address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageSettings {
    pub r#type: String,
    pub path: String,
    pub max_file_size: String,
    pub allowed_types: Vec<String>,
}

impl fmt::Display for StorageSettings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "类型: {}, 路径: {}, 最大文件大小: {}, 允许类型: {}", 
            self.r#type, self.path, self.max_file_size, self.allowed_types.join(", "))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsSettings {
    pub enabled: bool,
    pub provider: String,
    pub tracking_id: String,
}

impl AppConfig {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = Path::new("config.toml");
        
        if !config_path.exists() {
            // 如果配置文件不存在，复制示例文件
            if Path::new("config.example.toml").exists() {
                fs::copy("config.example.toml", "config.toml")?;
            } else {
                return Err("配置文件不存在且示例文件也不存在".into());
            }
        }
        
        let content = fs::read_to_string(config_path)?;
        let config: AppConfig = toml::from_str(&content)?;
        Ok(config)
    }
    
    pub fn get_default() -> Self {
        Self {
            app: AppSettings {
                name: "Liwin".to_string(),
                version: "0.1.0".to_string(),
                description: "A modern web application template".to_string(),
            },
            server: ServerSettings {
                host: "127.0.0.1".to_string(),
                port: 8080,
                debug: true,
            },
            database: DatabaseSettings {
                url: "sqlite:./data/app.db".to_string(),
                pool_size: 10,
            },
            features: FeatureSettings {
                auth: true,
                api: true,
                websocket: false,
                caching: true,
            },
            ui: UISettings {
                theme: "light".to_string(),
                language: "zh-CN".to_string(),
                timezone: "Asia/Shanghai".to_string(),
            },
            logging: LoggingSettings {
                level: "info".to_string(),
                file: "./logs/app.log".to_string(),
                max_size: "10MB".to_string(),
                max_files: 5,
            },
            security: SecuritySettings {
                session_timeout: 3600,
                max_login_attempts: 5,
                password_min_length: 8,
            },
            email: EmailSettings {
                smtp_host: "smtp.example.com".to_string(),
                smtp_port: 587,
                smtp_username: "your-email@example.com".to_string(),
                smtp_password: "your-password".to_string(),
                from_address: "noreply@example.com".to_string(),
            },
            storage: StorageSettings {
                r#type: "local".to_string(),
                path: "./uploads".to_string(),
                max_file_size: "10MB".to_string(),
                allowed_types: vec!["jpg".to_string(), "png".to_string(), "pdf".to_string(), "doc".to_string()],
            },
            analytics: AnalyticsSettings {
                enabled: false,
                provider: "google".to_string(),
                tracking_id: "".to_string(),
            },
        }
    }
} 