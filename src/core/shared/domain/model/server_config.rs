use serde::{ Deserialize, Serialize };

#[derive(Clone, Deserialize, Serialize)]
pub struct ServerConfig {
    pub platform: String,
    pub domain: String,
    pub http_port: u16,
    pub https_port: u16,
    pub mode: String,
    pub admin_email: String,
    pub database: ServerDatabaseConfig,
}

impl ServerConfig {
    pub fn http_domain_and_port(&self) -> String {
        format!("{}:{}", self.domain, self.http_port)
    }

    pub fn https_domain_and_port(&self) -> String {
        format!("{}:{}", self.domain, self.https_port)
    }

    pub fn has_development_mode(&self) -> bool {
        "development" == &self.mode
    }

    pub fn has_production_mode(&self) -> bool {
        "production" == &self.mode
    }

    pub fn platform_name(&self) -> String {
        (
            match self.platform.as_str() {
                "arch" => "Arch Linux",
                "debian" => "Debian",
                "dragonflybsd" => "DragonFly BSD",
                "fedora" => "Fedora",
                "freebsd" => "FreeBSD",
                "netbsd" => "NetBSD",
                "openbsd" => "OpenBSD",
                "opensuse" => "OpenSUSE",
                "ubuntu" => "Ubuntu",
                _ => "Linux",
            }
        ).to_string()
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct ServerDatabaseConfig {
    pub host: String,
    pub name: String,
    pub password: String,
    pub user: String,
}

impl ServerDatabaseConfig {
    // TODO: Replace all the other database_url and use only this one
    pub fn get_database_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}/{}",
            self.user,
            self.password,
            self.host,
            self.name
        )
    }
}

pub const TUKOSMO_VERSION: &'static str = "0.1.0";
