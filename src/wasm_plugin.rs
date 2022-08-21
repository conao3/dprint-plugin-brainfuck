use anyhow::Result;
use dprint_core::configuration::get_unknown_property_diagnostics;
use dprint_core::configuration::get_value;
use dprint_core::configuration::ConfigKeyMap;
use dprint_core::configuration::ConfigurationDiagnostic;
use dprint_core::configuration::GlobalConfiguration;
use dprint_core::configuration::ResolveConfigurationResult;
use dprint_core::generate_plugin_code;
use dprint_core::plugins::PluginHandler;
use dprint_core::plugins::PluginInfo;
use std::path::Path;

use crate::configuration::Configuration; // import the Configuration from above

pub struct MyPluginHandler {}

impl MyPluginHandler {
  fn new() -> Self {
    MyPluginHandler {}
  }
}

impl PluginHandler<Configuration> for MyPluginHandler {
  fn get_plugin_info(&mut self) -> PluginInfo {
    PluginInfo {
      name: env!("CARGO_PKG_NAME").to_string(),
      version: env!("CARGO_PKG_VERSION").to_string(),
      config_key: "keyGoesHere".to_string(),
      file_extensions: vec!["txt_ps".to_string()],
      file_names: vec![],
      help_url: "".to_string(),          // fill this in
      config_schema_url: "".to_string(), // leave this empty for now
    }
  }

  fn get_license_text(&mut self) -> String {
    "License text goes here.".to_string()
  }

  fn resolve_config(&mut self, config: ConfigKeyMap, global_config: &GlobalConfiguration) -> ResolveConfigurationResult<Configuration> {
    // implement this... for example
    let mut config = config;
    let mut diagnostics = Vec::new();
    let line_width = get_value(&mut config, "line_width", global_config.line_width.unwrap_or(120), &mut diagnostics);

    diagnostics.extend(get_unknown_property_diagnostics(config));

    ResolveConfigurationResult {
      config: Configuration { line_width },
      diagnostics,
    }
  }

  fn format_text(
    &mut self,
    file_path: &Path,
    file_text: &str,
    config: &Configuration,
    mut format_with_host: impl FnMut(&Path, String, &ConfigKeyMap) -> Result<String>,
  ) -> Result<String> {
    // format here
  }
}

// specify the plugin struct name and then an expression to create it
generate_plugin_code!(MyPluginHandler, MyPluginHandler::new());
