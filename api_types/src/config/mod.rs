use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub latest_message: String,
}

impl Config {
    pub fn to_string(&self) -> Result<String, ron::Error> {
        ron::ser::to_string_pretty(
            self,
            ron::ser::PrettyConfig::new().struct_names(true).to_owned(),
        )
    }
}

#[test]
fn can_parse_config() {
    let json = r#"{"latestMessage":"Hello, world!"}"#;
    let config: Config = serde_json::from_str(json).expect("Test data is validated");
    assert_eq!(config.latest_message, "Hello, world!");
}
