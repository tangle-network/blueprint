//! Minimal testing utilities using wiremock for HTTP mocking

/// Simple mock provider for non-AWS providers using HTTP mocking
#[cfg(test)]
pub mod http_mock {
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    pub async fn mock_digitalocean_server() -> MockServer {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v2/droplets"))
            .respond_with(
                ResponseTemplate::new(201)
                    .set_body_string(include_str!("../test_data/do_create_droplet.json")),
            )
            .mount(&server)
            .await;

        server
    }
}
