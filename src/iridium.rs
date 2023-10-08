pub mod http_client {
    use std::sync::Mutex;

    use const_format::formatcp;
    use once_cell::sync::Lazy;
    use regex::Regex;
    use reqwest::{Response, StatusCode};
    use serde::Deserialize;
    use tracing::{error, info, warn};
    use warp::http;

    const PROXY_URL_BASE: &str = formatcp!("http://{}/", crate::commons::PROXY_HOST);
    static HTTP_CLIENT: Lazy<reqwest::Client> = Lazy::new(http_client);
    static SESSION_COOKIE: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::default()));
    static SET_COOKIE_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"(ir-session-id=[^;]+)").unwrap());

    fn http_client() -> reqwest::Client {
        reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .expect("Default reqwest client couldn't build")
    }


    async fn relogin() -> bool {
        let client = http_client();
        let remote_request = client
            .request(reqwest::Method::POST, PROXY_URL_BASE)
            .header(reqwest::header::CONTENT_TYPE, reqwest::header::HeaderValue::from_str("application/x-www-form-urlencoded").unwrap())
            .body("Password=2007&name=authform&Login=admin")
            .build()
            .unwrap();

        match client.execute(remote_request).await {
            Ok(remote_response) => {
                if let Some(cookie) = remote_response.headers().get(reqwest::header::SET_COOKIE) {
                    if let Ok(new_cookie) = cookie.to_str() {
                        if let Some(captures) = SET_COOKIE_PATTERN.captures(new_cookie) {
                            if let Some(new_session_cookie)  = captures.get(1) {
                                let mut session_cookie = SESSION_COOKIE.lock().unwrap();
                                session_cookie.clear();
                                session_cookie.push_str(new_session_cookie.as_str());
                                return true;
                            }
                        }
                    }
                }
            }
            Err(err) => {
                error!("Login failed: {err}");
            }
        }
        false
    }

    async fn invoke_remote(path: &str, query: &[(&str, &str)]) -> Result<Response, &'static str> {
        let login_attempts = 3;
        'attempts: for _i in 1..login_attempts {
            let mut remote_uri: String = PROXY_URL_BASE.to_string();
            remote_uri.push_str(path);

            let mut remote_request_builder = HTTP_CLIENT.request(reqwest::Method::GET, remote_uri);
            if !query.is_empty() {
                remote_request_builder = remote_request_builder.query(query);
            }
            {
                let session_cookie = SESSION_COOKIE.lock().unwrap().as_str().to_owned();
                remote_request_builder = remote_request_builder.header(http::header::COOKIE, session_cookie);
            }
            let remote_request = remote_request_builder.build().unwrap();
            if let Ok(remote_response) = HTTP_CLIENT.execute(remote_request).await {
                info!("invoke_remote status: {}", remote_response.status());
                if remote_response.status() == StatusCode::OK {
                    return Ok(remote_response);
                } else if remote_response.status() == StatusCode::NOT_ACCEPTABLE {
                    if relogin().await {
                        warn!("Login successful");
                        continue 'attempts;
                    } else {
                        warn!("Login failed");
                        break;
                    }
                }
            } else {
                break;
            }
        }
        Err("Illegal state")
    }



    #[derive(Deserialize)]
    pub(crate) struct TagsBody {
        #[allow(dead_code)]
        tags_count: String,
        pub tags: Vec<TagRow>
    }

    #[derive(Deserialize, Debug)]
    pub(crate) struct TagRow {
        pub id: u32,
        pub name: String,
        #[allow(dead_code)]
        suffix: String,
        pub value: String,
        #[allow(dead_code)]
        data_type: String,
        #[allow(dead_code)]
        zero: String
    }

    pub(crate) async fn read_all_tags() -> Result<TagsBody, String> {
        match invoke_remote("json/tags/server/tags/get?from=0&count=999&what=tags", &[]).await {
            Ok(response) => {
                match response.json::<TagsBody>().await {
                    Ok(tags_body) => {
                        Ok(tags_body)
                    }
                    Err(err) => {
                        Err(err.to_string())
                    }
                }
            }
            Err(err) => {
                Err(err.to_string())
            }
        }
    }

    pub(crate) async fn send_set(channel_name: &str, value: &str) -> bool {
        match invoke_remote("json/ok/server/channel/set",
         &[("name", channel_name), ("value", value)]
        ).await {
            Ok(response) => {
                response.status() == StatusCode::OK
            }
            Err(_err) => {
                false
            }
        }
    }
}
