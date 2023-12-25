use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub enum BrowserEnum {
    Brave,
    Chrome,
    Edge,
    Firefox,
    IceCat,
    Midori,
    NintendoBrowser,
    Opera,
    Safari,
    Unknown,
}

#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub struct UserBrowser(BrowserEnum);

impl UserBrowser {
    pub fn from(value: BrowserEnum) -> Self {
        Self(value)
    }

    // TODO: Improve user agent parser
    pub fn from_user_agent(user_agent_request_header: &str) -> Self {
        let priority_ordered_keywords = [
            ("IceCat", BrowserEnum::IceCat),
            ("NintendoBrowser", BrowserEnum::NintendoBrowser),
            ("Firefox", BrowserEnum::Firefox),
            ("Brave", BrowserEnum::Brave),
            ("Midori", BrowserEnum::Midori),
            ("Edge", BrowserEnum::Edge),
            ("Opera", BrowserEnum::Opera),
            ("Chrome", BrowserEnum::Chrome),
            ("Safari", BrowserEnum::Safari),
        ];

        for (keyword, enum_value) in priority_ordered_keywords {
            if user_agent_request_header.contains(keyword) {
                return Self(enum_value);
            }
        }

        Self(BrowserEnum::Unknown)
    }

    pub fn value(&self) -> &BrowserEnum {
        &self.0
    }
}
