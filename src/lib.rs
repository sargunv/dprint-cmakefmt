use cmakefmt::{CaseStyle, Config, ContinuationAlign, DangleAlign, LineEnding};
use dprint_core::configuration::{
    ConfigKeyMap, ConfigurationDiagnostic, GlobalConfiguration, NewLineKind, get_nullable_value,
    get_unknown_property_diagnostics,
};
use dprint_core::plugins::{
    CheckConfigUpdatesMessage, ConfigChange, FileMatchingInfo, FormatError, FormatResult,
    PluginInfo, PluginResolveConfigurationResult, SyncFormatRequest, SyncHostFormatRequest,
    SyncPluginHandler,
};
use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct CMakeFmtPlugin;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResolvedConfig {
    line_width: usize,
    indent_width: usize,
    use_tabs: bool,
    new_line_kind: NewLineKind,
    command_case: CaseStyle,
    keyword_case: CaseStyle,
    max_empty_lines: usize,
    max_lines_hwrap: usize,
    max_pargs_hwrap: usize,
    max_subgroups_hwrap: usize,
    max_rows_cmdline: usize,
    require_valid_layout: bool,
    wrap_after_first_arg: bool,
    continuation_align: ContinuationAlign,
    enable_sort: bool,
    autosort: bool,
    dangle_parens: bool,
    dangle_align: DangleAlign,
    enable_markup: bool,
    first_comment_is_literal: bool,
}

impl Default for ResolvedConfig {
    fn default() -> Self {
        let config = Config::default();
        Self {
            line_width: config.line_width,
            indent_width: config.tab_size,
            use_tabs: config.use_tabchars,
            new_line_kind: NewLineKind::LineFeed,
            command_case: config.command_case,
            keyword_case: config.keyword_case,
            max_empty_lines: config.max_empty_lines,
            max_lines_hwrap: config.max_lines_hwrap,
            max_pargs_hwrap: config.max_pargs_hwrap,
            max_subgroups_hwrap: config.max_subgroups_hwrap,
            max_rows_cmdline: config.max_rows_cmdline,
            require_valid_layout: config.require_valid_layout,
            wrap_after_first_arg: config.wrap_after_first_arg,
            continuation_align: config.continuation_align,
            enable_sort: config.enable_sort,
            autosort: config.autosort,
            dangle_parens: config.dangle_parens,
            dangle_align: config.dangle_align,
            enable_markup: config.enable_markup,
            first_comment_is_literal: config.first_comment_is_literal,
        }
    }
}

impl ResolvedConfig {
    fn to_cmakefmt_config(&self) -> Config {
        Config {
            line_width: self.line_width,
            tab_size: self.indent_width,
            use_tabchars: self.use_tabs,
            line_ending: match self.new_line_kind {
                NewLineKind::LineFeed => LineEnding::Unix,
                NewLineKind::CarriageReturnLineFeed => LineEnding::Windows,
                NewLineKind::Auto => LineEnding::Auto,
            },
            command_case: self.command_case,
            keyword_case: self.keyword_case,
            max_empty_lines: self.max_empty_lines,
            max_lines_hwrap: self.max_lines_hwrap,
            max_pargs_hwrap: self.max_pargs_hwrap,
            max_subgroups_hwrap: self.max_subgroups_hwrap,
            max_rows_cmdline: self.max_rows_cmdline,
            require_valid_layout: self.require_valid_layout,
            wrap_after_first_arg: self.wrap_after_first_arg,
            continuation_align: self.continuation_align,
            enable_sort: self.enable_sort,
            autosort: self.autosort,
            dangle_parens: self.dangle_parens,
            dangle_align: self.dangle_align,
            enable_markup: self.enable_markup,
            first_comment_is_literal: self.first_comment_is_literal,
            ..Config::default()
        }
    }
}

impl SyncPluginHandler<ResolvedConfig> for CMakeFmtPlugin {
    fn resolve_config(
        &mut self,
        mut config: ConfigKeyMap,
        global_config: &GlobalConfiguration,
    ) -> PluginResolveConfigurationResult<ResolvedConfig> {
        let mut diagnostics = Vec::new();
        let mut resolved_config = ResolvedConfig::default();

        if let Some(line_width) = global_config.line_width {
            resolved_config.line_width = line_width as usize;
        }
        if let Some(indent_width) = global_config.indent_width {
            resolved_config.indent_width = indent_width as usize;
        }
        if let Some(use_tabs) = global_config.use_tabs {
            resolved_config.use_tabs = use_tabs;
        }
        if let Some(new_line_kind) = global_config.new_line_kind {
            resolved_config.new_line_kind = new_line_kind;
        }

        apply_config_value(
            &mut config,
            "lineWidth",
            &mut resolved_config.line_width,
            &mut diagnostics,
        );
        apply_config_value(
            &mut config,
            "indentWidth",
            &mut resolved_config.indent_width,
            &mut diagnostics,
        );
        apply_config_value(
            &mut config,
            "useTabs",
            &mut resolved_config.use_tabs,
            &mut diagnostics,
        );
        apply_config_value(
            &mut config,
            "newLineKind",
            &mut resolved_config.new_line_kind,
            &mut diagnostics,
        );
        apply_config_enum_value(
            &mut config,
            "commandCase",
            &mut resolved_config.command_case,
            parse_case_style,
            &mut diagnostics,
        );
        apply_config_enum_value(
            &mut config,
            "keywordCase",
            &mut resolved_config.keyword_case,
            parse_case_style,
            &mut diagnostics,
        );
        apply_config_value(
            &mut config,
            "maxEmptyLines",
            &mut resolved_config.max_empty_lines,
            &mut diagnostics,
        );
        apply_config_value(
            &mut config,
            "maxLinesHwrap",
            &mut resolved_config.max_lines_hwrap,
            &mut diagnostics,
        );
        apply_config_value(
            &mut config,
            "maxHangingWrapPositionalArgs",
            &mut resolved_config.max_pargs_hwrap,
            &mut diagnostics,
        );
        apply_config_value(
            &mut config,
            "maxHangingWrapGroups",
            &mut resolved_config.max_subgroups_hwrap,
            &mut diagnostics,
        );
        apply_config_value(
            &mut config,
            "maxRowsCmdline",
            &mut resolved_config.max_rows_cmdline,
            &mut diagnostics,
        );
        apply_config_value(
            &mut config,
            "requireValidLayout",
            &mut resolved_config.require_valid_layout,
            &mut diagnostics,
        );
        apply_config_value(
            &mut config,
            "wrapAfterFirstArg",
            &mut resolved_config.wrap_after_first_arg,
            &mut diagnostics,
        );
        apply_config_enum_value(
            &mut config,
            "continuationAlign",
            &mut resolved_config.continuation_align,
            parse_continuation_align,
            &mut diagnostics,
        );
        apply_config_value(
            &mut config,
            "enableSort",
            &mut resolved_config.enable_sort,
            &mut diagnostics,
        );
        apply_config_value(
            &mut config,
            "autosort",
            &mut resolved_config.autosort,
            &mut diagnostics,
        );
        apply_config_value(
            &mut config,
            "dangleParens",
            &mut resolved_config.dangle_parens,
            &mut diagnostics,
        );
        apply_config_enum_value(
            &mut config,
            "dangleAlign",
            &mut resolved_config.dangle_align,
            parse_dangle_align,
            &mut diagnostics,
        );
        apply_config_value(
            &mut config,
            "enableMarkup",
            &mut resolved_config.enable_markup,
            &mut diagnostics,
        );
        apply_config_value(
            &mut config,
            "firstCommentIsLiteral",
            &mut resolved_config.first_comment_is_literal,
            &mut diagnostics,
        );

        diagnostics.extend(get_unknown_property_diagnostics(config));

        PluginResolveConfigurationResult {
            file_matching: FileMatchingInfo {
                file_extensions: vec!["cmake".to_string()],
                file_names: vec![
                    "CMakeLists.txt".to_string(),
                    "CMakeLists.txt.in".to_string(),
                ],
            },
            diagnostics,
            config: resolved_config,
        }
    }

    fn plugin_info(&mut self) -> PluginInfo {
        let version = env!("CARGO_PKG_VERSION");

        PluginInfo {
            name: env!("CARGO_PKG_NAME").to_string(),
            version: version.to_string(),
            config_key: "cmakefmt".to_string(),
            help_url: env!("CARGO_PKG_REPOSITORY").to_string(),
            config_schema_url: format!(
                "https://plugins.dprint.dev/sargunv/dprint-cmakefmt/{version}/schema.json"
            ),
            update_url: Some(
                "https://plugins.dprint.dev/sargunv/dprint-cmakefmt/latest.json".to_string(),
            ),
        }
    }

    fn license_text(&mut self) -> String {
        include_str!("../LICENSE").to_string()
    }

    fn check_config_updates(
        &self,
        _message: CheckConfigUpdatesMessage,
    ) -> Result<Vec<ConfigChange>, FormatError> {
        Ok(Vec::new())
    }

    fn format(
        &mut self,
        request: SyncFormatRequest<ResolvedConfig>,
        _format_with_host: impl FnMut(SyncHostFormatRequest) -> FormatResult,
    ) -> FormatResult {
        if request.token.is_cancelled() {
            return Err("Formatting cancelled.".into());
        }

        if request.range.is_some() {
            return Err("Range formatting is not supported by dprint-cmakefmt.".into());
        }

        let source = std::str::from_utf8(&request.file_bytes).map_err(|err| {
            FormatError::new(format!(
                "Could not format {} because it is not valid UTF-8: {err}",
                request.file_path.display()
            ))
        })?;

        let config = request.config.to_cmakefmt_config();
        let formatted = cmakefmt::format_source(source, &config).map_err(|err| {
            FormatError::new(format!(
                "Could not format {} with cmakefmt: {err}",
                request.file_path.display()
            ))
        })?;

        if request.token.is_cancelled() {
            return Err("Formatting cancelled.".into());
        }

        if formatted == source {
            Ok(None)
        } else {
            Ok(Some(formatted.into_bytes()))
        }
    }
}

fn apply_config_value<T>(
    config: &mut ConfigKeyMap,
    key: &str,
    target: &mut T,
    diagnostics: &mut Vec<ConfigurationDiagnostic>,
) where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Display,
{
    if let Some(value) = get_nullable_value(config, key, diagnostics) {
        *target = value;
    }
}

fn apply_config_enum_value<T>(
    config: &mut ConfigKeyMap,
    key: &str,
    target: &mut T,
    parse: impl FnOnce(&str) -> Result<T, String>,
    diagnostics: &mut Vec<ConfigurationDiagnostic>,
) {
    if let Some(value) = get_nullable_value::<String>(config, key, diagnostics) {
        match parse(&value) {
            Ok(value) => *target = value,
            Err(message) => diagnostics.push(ConfigurationDiagnostic {
                property_name: key.to_string(),
                message,
            }),
        }
    }
}

fn parse_case_style(value: &str) -> Result<CaseStyle, String> {
    match value {
        "lower" => Ok(CaseStyle::Lower),
        "upper" => Ok(CaseStyle::Upper),
        "unchanged" => Ok(CaseStyle::Unchanged),
        _ => Err(format!(
            "Expected one of \"lower\", \"upper\", or \"unchanged\", but found {value:?}."
        )),
    }
}

fn parse_continuation_align(value: &str) -> Result<ContinuationAlign, String> {
    match value {
        "same-indent" => Ok(ContinuationAlign::SameIndent),
        "under-first-value" => Ok(ContinuationAlign::UnderFirstValue),
        _ => Err(format!(
            "Expected one of \"same-indent\" or \"under-first-value\", but found {value:?}."
        )),
    }
}

fn parse_dangle_align(value: &str) -> Result<DangleAlign, String> {
    match value {
        "prefix" => Ok(DangleAlign::Prefix),
        "open" => Ok(DangleAlign::Open),
        "close" => Ok(DangleAlign::Close),
        _ => Err(format!(
            "Expected one of \"prefix\", \"open\", or \"close\", but found {value:?}."
        )),
    }
}

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
dprint_core::generate_plugin_code!(CMakeFmtPlugin, CMakeFmtPlugin, ResolvedConfig);

#[cfg(test)]
mod tests {
    use std::path::Path;

    use dprint_core::configuration::ConfigKeyValue;
    use dprint_core::plugins::{CancellationToken, NullCancellationToken};

    use super::*;

    #[test]
    fn plugin_info_uses_cmakefmt_config_key() {
        let mut plugin = CMakeFmtPlugin;
        let info = plugin.plugin_info();

        assert_eq!(info.name, "dprint-cmakefmt");
        assert_eq!(info.config_key, "cmakefmt");
        assert_eq!(info.version, env!("CARGO_PKG_VERSION"));
        assert_eq!(
            info.config_schema_url,
            format!(
                "https://plugins.dprint.dev/sargunv/dprint-cmakefmt/{}/schema.json",
                env!("CARGO_PKG_VERSION")
            )
        );
        assert_eq!(
            info.update_url.as_deref(),
            Some("https://plugins.dprint.dev/sargunv/dprint-cmakefmt/latest.json")
        );
    }

    #[test]
    fn license_text_is_bundled() {
        let mut plugin = CMakeFmtPlugin;

        assert_eq!(plugin.license_text(), include_str!("../LICENSE"));
    }

    #[test]
    fn resolves_cmake_file_matching() {
        let mut plugin = CMakeFmtPlugin;
        let result = plugin.resolve_config(ConfigKeyMap::new(), &GlobalConfiguration::default());

        assert_eq!(result.file_matching.file_extensions, vec!["cmake"]);
        assert_eq!(
            result.file_matching.file_names,
            vec!["CMakeLists.txt", "CMakeLists.txt.in"]
        );
    }

    #[test]
    fn resolves_global_configuration() {
        let mut plugin = CMakeFmtPlugin;
        let result = plugin.resolve_config(
            ConfigKeyMap::new(),
            &GlobalConfiguration {
                line_width: Some(100),
                indent_width: Some(4),
                use_tabs: Some(true),
                new_line_kind: Some(NewLineKind::CarriageReturnLineFeed),
            },
        );

        assert!(result.diagnostics.is_empty());
        assert_eq!(result.config.line_width, 100);
        assert_eq!(result.config.indent_width, 4);
        assert!(result.config.use_tabs);
        assert_eq!(
            result.config.new_line_kind,
            NewLineKind::CarriageReturnLineFeed
        );
    }

    #[test]
    fn diagnoses_unknown_config_keys() {
        let mut config = ConfigKeyMap::new();
        config.insert("unknown".to_string(), ConfigKeyValue::Bool(true));

        let mut plugin = CMakeFmtPlugin;
        let result = plugin.resolve_config(config, &GlobalConfiguration::default());

        assert_eq!(result.diagnostics.len(), 1);
        assert_eq!(result.diagnostics[0].property_name, "unknown");
    }

    #[test]
    fn formats_cmake_source() {
        let mut plugin = CMakeFmtPlugin;
        let config = ResolvedConfig::default();
        let result = plugin
            .format(
                SyncFormatRequest {
                    file_path: Path::new("CMakeLists.txt"),
                    file_bytes: b"CMAKE_MINIMUM_REQUIRED(VERSION 3.20)\n".to_vec(),
                    config_id: dprint_core::plugins::FormatConfigId::uninitialized(),
                    config: &config,
                    range: None,
                    token: &NullCancellationToken,
                },
                |_| Ok(None),
            )
            .unwrap();

        assert_eq!(
            result,
            Some(b"cmake_minimum_required(VERSION 3.20)\n".to_vec())
        );
    }

    #[test]
    fn reports_no_change_for_stable_source() {
        let mut plugin = CMakeFmtPlugin;
        let config = ResolvedConfig::default();
        let result = plugin
            .format(
                SyncFormatRequest {
                    file_path: Path::new("CMakeLists.txt"),
                    file_bytes: b"cmake_minimum_required(VERSION 3.20)\n".to_vec(),
                    config_id: dprint_core::plugins::FormatConfigId::uninitialized(),
                    config: &config,
                    range: None,
                    token: &NullCancellationToken,
                },
                |_| Ok(None),
            )
            .unwrap();

        assert_eq!(result, None);
    }

    #[test]
    fn check_config_updates_is_noop() {
        let plugin = CMakeFmtPlugin;
        let changes = plugin
            .check_config_updates(CheckConfigUpdatesMessage {
                old_version: None,
                config: ConfigKeyMap::new(),
            })
            .unwrap();

        assert!(changes.is_empty());
    }

    #[test]
    fn rejects_non_utf8_input() {
        let mut plugin = CMakeFmtPlugin;
        let config = ResolvedConfig::default();
        let err = plugin
            .format(
                SyncFormatRequest {
                    file_path: Path::new("CMakeLists.txt"),
                    file_bytes: vec![0xff],
                    config_id: dprint_core::plugins::FormatConfigId::uninitialized(),
                    config: &config,
                    range: None,
                    token: &NullCancellationToken,
                },
                |_| Ok(None),
            )
            .unwrap_err();

        assert!(err.to_string().contains("not valid UTF-8"));
    }

    #[test]
    fn reports_malformed_cmake_errors() {
        let mut plugin = CMakeFmtPlugin;
        let config = ResolvedConfig::default();
        let err = plugin
            .format(
                SyncFormatRequest {
                    file_path: Path::new("CMakeLists.txt"),
                    file_bytes: b"if(\n".to_vec(),
                    config_id: dprint_core::plugins::FormatConfigId::uninitialized(),
                    config: &config,
                    range: None,
                    token: &NullCancellationToken,
                },
                |_| Ok(None),
            )
            .unwrap_err();

        assert!(err.to_string().contains("cmakefmt"));
    }

    #[test]
    fn formats_large_input_without_panicking() {
        let mut plugin = CMakeFmtPlugin;
        let config = ResolvedConfig::default();
        let source = "CMAKE_MINIMUM_REQUIRED(VERSION 3.20)\n".repeat(1_000);
        let result = plugin
            .format(
                SyncFormatRequest {
                    file_path: Path::new("CMakeLists.txt"),
                    file_bytes: source.into_bytes(),
                    config_id: dprint_core::plugins::FormatConfigId::uninitialized(),
                    config: &config,
                    range: None,
                    token: &NullCancellationToken,
                },
                |_| Ok(None),
            )
            .unwrap();

        assert!(result.is_some());
    }

    #[test]
    fn repeated_format_calls_are_stable() {
        let mut plugin = CMakeFmtPlugin;
        let config = ResolvedConfig::default();

        for _ in 0..10 {
            let result = plugin
                .format(
                    SyncFormatRequest {
                        file_path: Path::new("CMakeLists.txt"),
                        file_bytes: b"cmake_minimum_required(VERSION 3.20)\n".to_vec(),
                        config_id: dprint_core::plugins::FormatConfigId::uninitialized(),
                        config: &config,
                        range: None,
                        token: &NullCancellationToken,
                    },
                    |_| Ok(None),
                )
                .unwrap();

            assert_eq!(result, None);
        }
    }

    #[test]
    fn rejects_range_formatting_explicitly() {
        let mut plugin = CMakeFmtPlugin;
        let config = ResolvedConfig::default();
        let err = plugin
            .format(
                SyncFormatRequest {
                    file_path: Path::new("CMakeLists.txt"),
                    file_bytes: b"cmake_minimum_required(VERSION 3.20)\n".to_vec(),
                    config_id: dprint_core::plugins::FormatConfigId::uninitialized(),
                    config: &config,
                    range: Some(0..10),
                    token: &NullCancellationToken,
                },
                |_| Ok(None),
            )
            .unwrap_err();

        assert!(
            err.to_string()
                .contains("Range formatting is not supported")
        );
    }

    #[test]
    fn respects_cancellation_before_formatting() {
        #[derive(Debug)]
        struct CancelledToken;

        impl CancellationToken for CancelledToken {
            fn is_cancelled(&self) -> bool {
                true
            }
        }

        let mut plugin = CMakeFmtPlugin;
        let config = ResolvedConfig::default();
        let err = plugin
            .format(
                SyncFormatRequest {
                    file_path: Path::new("CMakeLists.txt"),
                    file_bytes: b"cmake_minimum_required(VERSION 3.20)\n".to_vec(),
                    config_id: dprint_core::plugins::FormatConfigId::uninitialized(),
                    config: &config,
                    range: None,
                    token: &CancelledToken,
                },
                |_| Ok(None),
            )
            .unwrap_err();

        assert!(err.to_string().contains("cancelled"));
    }
}
