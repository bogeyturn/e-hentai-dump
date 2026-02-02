use std::{collections::HashMap, sync::Arc};

use futures::lock::Mutex;
use reqwest::{
    ClientBuilder, Proxy,
    header::{HeaderMap, USER_AGENT},
};

use crate::{CallbackTrait, Session};

pub struct Cookie {
    /// User ID
    pub ipb_member_id: String,
    /// Password hash
    pub ipb_pass_hash: String,
    /// session
    pub sk: String,
    /// used for images
    pub igneous: Option<String>,
    pub hath_perks: Option<String>,
}

impl Cookie {
    pub fn new(
        ipb_member_id: &str,
        ipb_pass_hash: &str,
        sk: &str,
        igneous: Option<&String>,
        hath_perks: Option<&String>,
    ) -> Self {
        Cookie {
            ipb_member_id: ipb_member_id.to_string(),
            ipb_pass_hash: ipb_pass_hash.to_string(),
            sk: sk.to_string(),
            igneous: igneous.map(|v| v.to_string()),
            hath_perks: hath_perks.map(|v| v.to_string()),
        }
    }

    pub fn to_string(&self) -> String {
        let Self {
            ipb_member_id,
            ipb_pass_hash,
            sk,
            igneous,
            hath_perks,
        } = self;
        let igneous = igneous
            .as_ref()
            .map(|v| format!("; igneous={}", v))
            .unwrap_or_default();
        let hath_perks = hath_perks
            .as_ref()
            .map(|v| format!("; hath_perks={}", v))
            .unwrap_or_default();
        format!(
            "ipb_member_id={ipb_member_id}; ipb_pass_hash={ipb_pass_hash}; sk={sk}{igneous}{hath_perks}"
        )
    }
}

impl Session {
    pub fn cookie(
        ipb_member_id: &str,
        ipb_pass_hash: &str,
        sk: &str,
        igneous: Option<&String>,
        hath_perks: Option<&String>,
        connections: Option<u64>,
        url_rewrite: Option<String>,
        callback: Option<Box<dyn CallbackTrait>>,
    ) -> Self {
        let cookie = Cookie::new(ipb_member_id, ipb_pass_hash, sk, igneous, hath_perks);
        let mut hm = HeaderMap::new();
        hm.append(USER_AGENT, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/18.6 Safari/605.1.15".try_into().unwrap());
        let clients = (0..connections.unwrap_or(1))
            .map(|_| {
                ClientBuilder::new()
                    .default_headers(hm.clone())
                    .build()
                    .unwrap()
            })
            .collect::<Vec<_>>();
        Session {
            clients,
            url_rewrite,
            rr: Default::default(),
            cookie: Arc::new(Mutex::new(cookie)),
            callback: Arc::new(Mutex::new(callback)),
        }
    }

    pub fn set_proxies(&mut self, proxies: Vec<Proxy>) {
        let mut hm = HeaderMap::new();
        hm.append(USER_AGENT, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/18.6 Safari/605.1.15".try_into().unwrap());
        self.clients = proxies
            .into_iter()
            .map(|v| {
                ClientBuilder::new()
                    .default_headers(hm.clone())
                    .proxy(v)
                    .build()
                    .unwrap()
            })
            .collect::<Vec<_>>();
    }

    pub fn new(
        cookie: String,
        url_rewrite: Option<String>,
        callback: Option<Box<dyn CallbackTrait>>,
    ) -> Self {
        let cookies = cookie
            .split(';')
            .filter_map(|part| {
                let mut iter = part.trim().splitn(2, '=');
                let key = iter.next()?;
                let value = iter.next()?;
                Some((key.to_string(), value.to_string()))
            })
            .collect::<HashMap<_, _>>();
        Self::cookie(
            cookies.get("ipb_member_id").unwrap(),
            cookies.get("ipb_pass_hash").unwrap(),
            cookies.get("sk").unwrap(),
            cookies.get("igneous"),
            cookies.get("hath_perks"),
            None,
            url_rewrite,
            callback,
        )
    }
}
