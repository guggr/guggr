#[derive(Debug, PartialEq, Eq)]
pub struct BasicConfig {
    user: String,
    password: String,
    host: String,
    port: String,
}

impl BasicConfig {
    pub const fn new(user: String, password: String, host: String, port: String) -> Self {
        Self {
            user,
            password,
            host,
            port,
        }
    }
}

pub trait BasicConfigTrait {
    fn user(&self) -> String;
    fn password(&self) -> String;
    fn host(&self) -> String;
    fn port(&self) -> String;
}

impl BasicConfigTrait for BasicConfig {
    fn user(&self) -> String {
        self.user.clone()
    }

    fn password(&self) -> String {
        self.password.clone()
    }

    fn host(&self) -> String {
        self.host.clone()
    }

    fn port(&self) -> String {
        self.port.clone()
    }
}
