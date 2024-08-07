#[cfg(feature = "deserialize_bundler_options")]
use schemars::JsonSchema;
#[cfg(feature = "deserialize_bundler_options")]
use serde::Deserialize;

#[cfg_attr(
  feature = "deserialize_bundler_options",
  derive(Deserialize, JsonSchema),
  serde(rename_all = "camelCase", deny_unknown_fields)
)]
#[derive(Debug, Clone)]
pub enum ModuleType {
  Js,
  Jsx,
  Ts,
  Tsx,
  Json,
  Text,
  Base64,
  Dataurl,
  Binary,
  Empty,
  Custom(String),
}

impl ModuleType {
  pub fn from_known_str(s: &str) -> anyhow::Result<Self> {
    match s {
      "js" => Ok(Self::Js),
      "jsx" => Ok(Self::Jsx),
      "ts" => Ok(Self::Ts),
      "tsx" => Ok(Self::Tsx),
      "json" => Ok(Self::Json),
      "text" => Ok(Self::Text),
      "base64" => Ok(Self::Base64),
      "dataurl" => Ok(Self::Dataurl),
      "binary" => Ok(Self::Binary),
      "empty" => Ok(Self::Empty),
      _ => Err(anyhow::format_err!("Unknown module type: {s}")),
    }
  }
}
