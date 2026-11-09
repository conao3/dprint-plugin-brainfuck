use dprint_core::configuration::get_unknown_property_diagnostics;
use dprint_core::configuration::get_value;
use dprint_core::configuration::ConfigKeyMap;
use dprint_core::configuration::GlobalConfiguration;
use dprint_core::plugins::CheckConfigUpdatesMessage;
use dprint_core::plugins::ConfigChange;
use dprint_core::plugins::FileMatchingInfo;
use dprint_core::plugins::FormatError;
use dprint_core::plugins::FormatResult;
use dprint_core::plugins::PluginInfo;
use dprint_core::plugins::PluginResolveConfigurationResult;
use dprint_core::plugins::SyncFormatRequest;
use dprint_core::plugins::SyncHostFormatRequest;
use dprint_core::plugins::SyncPluginHandler;

use crate::configuration::Configuration; // import the Configuration from above

pub struct MyPluginHandler {}

impl MyPluginHandler {
    const fn new() -> Self {
        MyPluginHandler {}
    }
}

impl SyncPluginHandler<Configuration> for MyPluginHandler {
    fn plugin_info(&mut self) -> PluginInfo {
        PluginInfo {
            name: env!("CARGO_PKG_NAME").to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            config_key: "keyGoesHere".to_string(),
            help_url: "".to_string(),          // fill this in
            config_schema_url: "".to_string(), // leave this empty for now
            update_url: Some("".to_string()),  // leave this empty for now
            supports_range_formatting: Some(false),
        }
    }

    fn license_text(&mut self) -> String {
        "License text goes here.".to_string()
    }

    fn resolve_config(
        &mut self,
        config: ConfigKeyMap,
        global_config: &GlobalConfiguration,
    ) -> PluginResolveConfigurationResult<Configuration> {
        // implement this... for example
        let mut config = config;
        let mut diagnostics = Vec::new();
        let line_width = get_value(
            &mut config,
            "line_width",
            global_config.line_width.unwrap_or(120),
            &mut diagnostics,
        );

        diagnostics.extend(get_unknown_property_diagnostics(config));

        PluginResolveConfigurationResult {
            file_matching: FileMatchingInfo {
                file_extensions: vec!["txt_ps".to_string()],
                file_names: vec![],
                additive: false,
            },
            config: Configuration { line_width },
            diagnostics,
        }
    }

    fn check_config_updates(
        &self,
        _message: CheckConfigUpdatesMessage,
    ) -> Result<Vec<ConfigChange>, FormatError> {
        Ok(Vec::new())
    }

    fn format(
        &mut self,
        _request: SyncFormatRequest<Configuration>,
        _format_with_host: impl FnMut(SyncHostFormatRequest) -> FormatResult,
    ) -> FormatResult {
        // format here
        Ok(None)
    }
}

// specify the plugin struct name and then an expression to create it
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
dprint_core::generate_plugin_code!(MyPluginHandler, MyPluginHandler::new());
