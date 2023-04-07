use crate::config::MjAccount;
use crate::mj_domain::{TemplateContentResponse, TemplateResponse};
use reqwest::blocking::{Client, RequestBuilder};
use reqwest::Method;
use reqwest::Result;

const BASE_REST_URL: &str = "https://api.mailjet.com/v3/REST";

pub struct MjClient<'a> {
    account: &'a MjAccount,
    reqwest: Client,
}

impl<'a> MjClient<'a> {
    pub fn new(account: &'a MjAccount) -> Self {
        Self {
            account,
            reqwest: Client::new(),
        }
    }

    fn new_request(&self, method: Method, endpoint: &str) -> RequestBuilder {
        self.reqwest
            .request(method, format!("{}{}", BASE_REST_URL, endpoint))
            .basic_auth(self.account.public(), Some(self.account.private()))
    }

    pub fn get_templates(&self) -> Result<TemplateResponse> {
        self.new_request(Method::GET, "/template")
            .send()?
            .json::<TemplateResponse>()
    }

    pub fn get_template(&self, id: u32) -> Result<TemplateResponse> {
        self.new_request(Method::GET, format!("/template/{}", id).as_str())
            .send()?
            .json::<TemplateResponse>()
    }

    pub fn get_template_content(&self, id: u32) -> Result<TemplateContentResponse> {
        self.new_request(
            Method::GET,
            format!("/template/{}/detailcontent", id).as_str(),
        )
        .send()?
        .json::<TemplateContentResponse>()
    }
}
