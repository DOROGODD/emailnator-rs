use reqwest::{
    cookie::{Cookie, Jar},
    header, Client, Proxy, Url,
};
use serde_json::{json, Value};
use std::{collections::HashMap, sync::Arc};
use tokio;
use urlencoding::decode;

pub struct CreateClient {
    client: Client,
}

impl CreateClient {
    pub fn new(proxy: Option<String>) -> Self {
        let mut default_headers = header::HeaderMap::new();
        default_headers.insert(
            "Accept",
            "application/json, text/plain, */*".parse().unwrap(),
        );
        default_headers.insert(
            "Accept-Language",
            "ru,en;q=0.9,vi;q=0.8,es;q=0.7,cy;q=0.6".parse().unwrap(),
        );
        default_headers.insert("Content-Type", "application/json".parse().unwrap());

        let mut client = Client::builder()
            .default_headers(default_headers)
            .cookie_store(true);

        if let Some(proxy) = proxy {
            client = client.proxy(Proxy::all(proxy).unwrap());
        }

        CreateClient {
            client: client.build().unwrap(),
        }
    }

    // async fn get(&self, url: &str) -> Result<reqwest::Response, reqwest::Error> {
    //     let client = Client::builder()
    //         .proxy(reqwest::Proxy::all(&self.proxy.clone().unwrap()).unwrap())
    //         .build()?;

    //     let res = client.get(url).send().await?;
    //     Ok(res)
    // }

    // async fn post(
    //     &self,
    //     url: &str,
    //     json: Option<&serde_json::Value>,
    // ) -> Result<reqwest::Response, reqwest::Error> {
    //     let cookies = self.get_request_cookies().await?;
    //     let csrf_token = decode(cookies.get("XSRF-TOKEN").unwrap()).unwrap();

    //     let client = Client::builder()
    //         .proxy(reqwest::Proxy::all(&self.proxy.clone().unwrap()).unwrap())
    //         .build()?;

    //     let res = client.post(url).json(&json).send().await?;
    //     Ok(res)
    // }

    async fn get_request_cookies<'a>(&'a self) -> Result<HashMap<String, String>, reqwest::Error> {
        let url = "https://www.emailnator.com/generate-email";
        let response = self.client.get(url).send().await?;
        Ok(response
            .cookies()
            .into_iter()
            .map(|cookie| (cookie.name().to_string(), cookie.value().to_string()))
            .collect())
    }

    pub async fn get_email(
        &self,
        use_plus_gmail: bool,
        use_dot_gmail: bool,
        use_google_mail: bool,
        use_domain: bool,
    ) -> Result<String, reqwest::Error> {
        let mut email_list = Vec::new();

        let csrf_token =
            decode(self.get_request_cookies().await?.get("XSRF-TOKEN").unwrap()).unwrap();

        if use_plus_gmail {
            email_list.push("plusGmail");
        }
        if use_dot_gmail {
            email_list.push("dotGmail");
        }
        if use_google_mail {
            email_list.push("googleMail");
        }
        if use_domain {
            email_list.push("domain");
        }

        let json_data = json!({ "email": email_list });

        let response = self
            .client
            .post("https://www.emailnator.com/generate-email")
            .header("x-xsrf-token", csrf_token)
            .json(&json_data)
            .send()
            .await?;
        let json: Value = response.json().await?;
        Ok(json["email"][0].as_str().unwrap().to_string())
    }

    pub async fn get_message(
        &self,
        email: &str,
        message_id: &str,
    ) -> Result<String, reqwest::Error> {
        let cookies = self.get_request_cookies().await?;
        let csrf_token = decode(cookies.get("XSRF-TOKEN").unwrap()).unwrap();

        let json_data = json!({ "email": email, "messageID": message_id });

        let response = self
            .client
            .post("https://www.emailnator.com/message-list")
            .header("x-xsrf-token", csrf_token)
            .json(&json_data)
            .send()
            .await?;
        Ok(response.text().await?)
    }
}

#[cfg(test)]
mod tests {
    use crate::CreateClient;

    #[tokio::test]
    async fn test_get_email() {
        let response = CreateClient::new(None)
            .get_email(true, true, true, true)
            .await;

        assert!(response.is_ok());
        println!("{}", response.unwrap());
    }

    #[tokio::test]
    async fn test_get_message() {
        // MThjOTdkYzNhYjVjN2ZlZA==

        let response = CreateClient::new(None)
            .get_message("dueltmp+gltts@gmail.com", "MThjOTdkYzNhYjVjN2ZlZA==")
            .await;

        assert!(response.is_ok());
        println!("{}", response.unwrap());
    }
}
