use serde::Serialize;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Configuration {
  // add configuration properties here...
  line_width: u32, // for example
}
