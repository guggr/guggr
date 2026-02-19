use urlencoding::encode;

use crate::basic::BasicConfigTrait;

pub(crate) trait ConnectionUrl {
    fn connection_url_builder(&self, protocol: String, path: String) -> String;
}

impl<T> ConnectionUrl for T
where
    T: BasicConfigTrait,
{
    fn connection_url_builder(&self, protocol: String, path: String) -> String {
        format!(
            "{}://{}:{}@{}:{}/{}",
            protocol,
            encode(&self.user()),
            encode(&self.password()),
            self.host(),
            self.port(),
            encode(&path)
        )
    }
}
