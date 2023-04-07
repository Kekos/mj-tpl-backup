use serde_derive::Deserialize;
use serde_json::Result;

#[derive(Deserialize, Debug, PartialEq)]
pub struct Template {
    #[serde(rename = "ID")]
    pub id: u32,
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct TemplateResponse {
    #[serde(rename = "Count")]
    pub count: u16,
    #[serde(rename = "Data")]
    pub data: Vec<Template>,
    #[serde(rename = "Total")]
    pub total: u16,
}

#[cfg(test)]
impl TemplateResponse {
    fn from_json(json: &str) -> Result<Self> {
        serde_json::from_str::<Self>(json)
    }
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct TemplateContentHeaders {
    #[serde(rename = "Subject")]
    pub subject: String,
    #[serde(rename = "SenderName")]
    pub sender_name: String,
    #[serde(rename = "SenderEmail")]
    pub sender_email: String,
    #[serde(rename = "ReplyEmail")]
    pub reply_email: String,
    #[serde(rename = "X-MJ-TemplateLanguage")]
    pub x_mj_template_language: String,
    #[serde(rename = "From")]
    pub from: String,
    #[serde(rename = "Reply-To")]
    pub reply_to: String,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct TemplateContent {
    #[serde(rename = "MJMLContent")]
    pub mjmlcontent: Option<String>,
    #[serde(rename = "Html-part")]
    pub html_part: Option<String>,
    #[serde(rename = "Text-part")]
    pub text_part: Option<String>,
    #[serde(rename = "Headers")]
    pub headers: TemplateContentHeaders,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct TemplateContentResponse {
    #[serde(rename = "Count")]
    pub count: u16,
    #[serde(rename = "Data")]
    pub data: Vec<TemplateContent>,
    #[serde(rename = "Total")]
    pub total: u16,
}

#[cfg(test)]
impl TemplateContentResponse {
    fn from_json(json: &str) -> Result<Self> {
        serde_json::from_str::<Self>(json)
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_domain::{
        Template, TemplateContent, TemplateContentHeaders, TemplateContentResponse,
        TemplateResponse,
    };

    #[test]
    fn test_parse_template_response() {
        let data = r#"
            {
                "Count": 1,
                "Data": [
                    {
                        "ID": 12345,
                        "Name": "TEST NAME",
                        "Author": "AUTHOR",
                        "OwnerId": 67890,
                        "CreatedAt": "2021-10-21T18:33:46Z",
                        "LastUpdatedAt": "2021-11-11T17:09:48Z"
                    }
                ],
                "Total": 1
            }"#;

        assert_eq!(
            TemplateResponse {
                count: 1,
                data: vec![Template {
                    id: 12345,
                    name: String::from("TEST NAME"),
                },],
                total: 1,
            },
            TemplateResponse::from_json(data).unwrap()
        )
    }

    #[test]
    fn test_parse_template_content_response() {
        let data = r#"
            {
                "Count": 1,
                "Data": [
                    {
                        "MJMLContent": null,
                        "Html-part": "<!DOCTYPE html>\n",
                        "Text-part": "TEXT PART\n",
                        "Headers": {
                            "Subject": "MY SUBJECT",
                            "SenderName": "SENDER NAME",
                            "SenderEmail": "test@example.com",
                            "ReplyEmail": "",
                            "X-MJ-TemplateLanguage": "0",
                            "From": "FROM NAME <test@example.com>",
                            "Reply-To": ""
                        }
                    }
                ],
                "Total": 1
            }"#;

        assert_eq!(
            TemplateContentResponse {
                count: 1,
                data: vec![TemplateContent {
                    mjmlcontent: None,
                    html_part: Some(String::from("<!DOCTYPE html>\n")),
                    text_part: Some(String::from("TEXT PART\n")),
                    headers: TemplateContentHeaders {
                        subject: String::from("MY SUBJECT"),
                        sender_name: String::from("SENDER NAME"),
                        sender_email: String::from("test@example.com"),
                        reply_email: String::from(""),
                        x_mj_template_language: String::from("0"),
                        from: String::from("FROM NAME <test@example.com>"),
                        reply_to: String::from(""),
                    },
                },],
                total: 1,
            },
            TemplateContentResponse::from_json(data).unwrap()
        )
    }
}
