pub struct Config {
    pub rest_api_endpoint: String,
    pub web_socket_endpoint: String,
    pub web_socket_subscribe_endpoint: String,
    pub web_socket_unsubscribe_endpoint: String,
}

impl Config {
    pub fn default() -> Self {
        Config {
            rest_api_endpoint: String::from("https://serum-api.bonfida.com"),
            web_socket_endpoint: String::from("wss://serum-ws.bonfida.com"),
            web_socket_subscribe_endpoint: "https://serum-ws.bonfida.com".into(),
            web_socket_unsubscribe_endpoint: "https://serum-ws.bonfida.com".into(),
        }
    }

    pub fn set_rest_end_point<T: Into<String>>(mut self, rest_api_endpoint: T) -> Self {
        self.rest_api_endpoint = rest_api_endpoint.into();
        self
    }

    pub fn set_web_socket_end_point<T: Into<String>>(mut self, web_socket_endpoint: T) -> Self {
        self.web_socket_endpoint = web_socket_endpoint.into();
        self
    }

    pub fn set_web_socket_subscribe_endpoint<T: Into<String>>(
        mut self,
        web_socket_subscribe_endpoint: T,
    ) -> Self {
        self.web_socket_subscribe_endpoint = web_socket_subscribe_endpoint.into();
        self
    }

    pub fn set_web_socket_unsubscribe_endpoint<T: Into<String>>(
        mut self,
        web_socket_unsubscribe_endpoint: T,
    ) -> Self {
        self.web_socket_unsubscribe_endpoint = web_socket_unsubscribe_endpoint.into();
        self
    }
}
