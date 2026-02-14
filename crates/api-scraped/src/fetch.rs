use std::sync::atomic::Ordering;

use anyhow::bail;
use reqwest::{
    Client, Response, Url,
    header::{COOKIE, SET_COOKIE},
};
use scraper::Html;
use serde::Serialize;

use crate::Session;

impl Session {
    async fn get_client(&self) -> &Client {
        let call = self.rr.fetch_add(1, Ordering::Relaxed);
        let idx = (call / 15) % self.clients.len();
        &self.clients[idx]
    }
    async fn update_cookie(&self, cookie: String) {
        let cookie = cookie
            .split_once("; ")
            .map(|v| v.0)
            .unwrap_or(cookie.as_str())
            .split_once("=");
        let c = self.cookie.lock().await.to_string();
        match cookie {
            Some(("ipb_member_id", value)) => {
                self.cookie.lock().await.ipb_member_id = value.to_string()
            }

            Some(("ipb_pass_hash", value)) => {
                self.cookie.lock().await.ipb_pass_hash = value.to_string()
            }
            Some(("sk", value)) => self.cookie.lock().await.sk = value.to_string(),
            Some(("igneous", value)) => self.cookie.lock().await.igneous = Some(value.to_string()),
            Some(("hath_perks", value)) => {
                self.cookie.lock().await.hath_perks = Some(value.to_string())
            }
            Some((key, value)) => log::info!("Unknown set-cookie: {}={}", key, value),
            None => todo!(),
        }
        if let Some(callback) = self.callback.lock().await.as_ref() {
            let msg = self.cookie.lock().await.to_string();
            if c != msg {
                callback.call("set-cookie", serde_json::Value::String(msg));
            }
        }
    }

    pub async fn local_api<T: Serialize>(&self, path: &str, json: &T) -> anyhow::Result<Response> {
        Ok(self
            .get_client()
            .await
            .post(self.local_api_host.join(path).unwrap())
            .json(&json)
            .send()
            .await?)
    }

    pub async fn api(&self, json: serde_json::Value) -> anyhow::Result<Response> {
        Ok(self
            .get_client()
            .await
            .post(self.url("https://s.exhentai.org/api.php").await)
            .header(COOKIE, self.cookie.lock().await.to_string())
            .json(&json)
            .send()
            .await?)
    }

    pub async fn get_text(&self, url: impl ToString) -> anyhow::Result<String> {
        let req = self
            .get_client()
            .await
            .get(self.url(url).await)
            .header(COOKIE, self.cookie.lock().await.to_string())
            .send()
            .await?
            .error_for_status()?;
        let headers = req
            .headers()
            .iter()
            .filter(|v| v.0.as_str().to_lowercase().as_str() == SET_COOKIE)
            .filter_map(|v| String::from_utf8(v.1.as_bytes().to_vec()).ok())
            .collect::<Vec<_>>();
        let text = req.text().await?;
        if text.contains(
            "This IP address has been temporarily banned due to an excessive request rate",
        ) {
            bail!("IP address has been temporarily banned")
        }
        for header in headers {
            self.update_cookie(header).await;
        }
        Ok(text)
    }

    pub async fn form<T: Serialize + ?Sized>(
        &self,
        url: impl ToString,
        form: &T,
    ) -> anyhow::Result<Response> {
        let req = self
            .get_client()
            .await
            .post(self.url(url).await)
            .header(COOKIE, self.cookie.lock().await.to_string())
            .form(form)
            .send()
            .await?
            .error_for_status()?;
        let headers = req
            .headers()
            .iter()
            .filter(|v| v.0.as_str().to_lowercase().as_str() == SET_COOKIE)
            .filter_map(|v| String::from_utf8(v.1.as_bytes().to_vec()).ok())
            .collect::<Vec<_>>();
        for header in headers {
            self.update_cookie(header).await;
        }
        Ok(req)
    }

    pub async fn get_html(&self, url: impl ToString) -> anyhow::Result<Html> {
        let req = self.get_text(url).await?;
        let html = Html::parse_document(&req);
        Ok(html)
    }

    async fn url(&self, url: impl ToString) -> String {
        match &self.url_rewrite {
            Some(str) => str
                .replace(
                    "{url}",
                    urlencoding::encode(url.to_string().as_str()).as_ref(),
                )
                .replace(
                    "{cookie}",
                    urlencoding::encode(&self.cookie.lock().await.to_string()).as_ref(),
                ),
            None => url.to_string(),
        }
    }
}
