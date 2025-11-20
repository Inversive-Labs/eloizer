use anyhow::Result;
use colored::*;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
struct Config {
    analysis: AnalysisConfig,
    output: OutputConfig,
    rules: RulesConfig,
    #[serde(default)]
    display: DisplayConfig,
}

#[derive(Debug, Deserialize)]
struct AnalysisConfig {
    path: String,
    #[serde(default)]
    generate_ast: bool,
}

#[derive(Debug, Deserialize)]
struct OutputConfig {
    report_file: String,
}

#[derive(Debug, Deserialize)]
struct RulesConfig {
    #[serde(default)]
    ignore_severities: Vec<String>,
    #[serde(default)]
    ignore_rules: Vec<String>,
    #[serde(default)]
    include_rule_types: Vec<String>,
}

#[derive(Debug, Deserialize, Default)]
struct DisplayConfig {
    #[serde(default)]
    verbose: bool,
    #[serde(default)]
    quiet: bool,
    #[serde(default)]
    no_color: bool,
}

pub fn run(config_path: PathBuf, cli_verbose: bool, cli_quiet: bool) -> Result<()> {
    if !config_path.exists() {
        eprintln!(
            "{} Configuration file not found: {}",
            "✗".red().bold(),
            config_path.display().to_string().yellow()
        );
        eprintln!(
            "\nCreate one with: {}\n",
            "eloizer init".cyan().bold()
        );
        anyhow::bail!("Configuration file not found");
    }

    // Read and parse config
    let config_content = fs::read_to_string(&config_path)?;
    let config: Config = toml::from_str(&config_content).map_err(|e| {
        eprintln!(
            "{} Failed to parse configuration file: {}",
            "✗".red().bold(),
            e.to_string().red()
        );
        e
    })?;

    println!(
        "\n{} Using configuration: {}\n",
        "⚙".cyan().bold(),
        config_path.display().to_string().bright_blue()
    );

    // Prepare parameters for analyze command
    let path = PathBuf::from(&config.analysis.path);
    let templates = None; // TODO: Add templates to config file
    let output = Some(PathBuf::from(&config.output.report_file));
    let ast = config.analysis.generate_ast;

    let ignore = if config.rules.ignore_severities.is_empty() {
        None
    } else {
        Some(config.rules.ignore_severities.join(","))
    };

    let ignore_rules = if config.rules.ignore_rules.is_empty() {
        None
    } else {
        Some(config.rules.ignore_rules.join(","))
    };

    // CLI flags override config
    let verbose = cli_verbose || config.display.verbose;
    let quiet = cli_quiet || config.display.quiet;

    // Run analysis
    super::analyze::run(path, templates, output, ast, ignore, ignore_rules, verbose, quiet)
}
