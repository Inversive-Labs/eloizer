use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::*;

mod commands;

#[derive(Parser)]
#[command(
    name = "eloizer",
    author = "inversive <contact@inversive.xyz>",
    version,
    about = "ELOIZER - Static Analyzer for Solana Smart Contracts",
    long_about = "A powerful static analysis tool for detecting security vulnerabilities and code quality issues in Solana/Anchor smart contracts written in Rust."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Disable colored output
    #[arg(long, global = true)]
    no_color: bool,

    /// Enable verbose output
    #[arg(short, long, global = true)]
    verbose: bool,

    /// Quiet mode (errors only)
    #[arg(short, long, global = true)]
    quiet: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Analyze Solana smart contracts for vulnerabilities
    Analyze {
        /// Path to Solana project directory or Rust file
        #[arg(short, long, value_name = "PATH")]
        path: std::path::PathBuf,

        /// Custom templates path
        #[arg(short, long, value_name = "DIR")]
        templates: Option<std::path::PathBuf>,

        /// Output report file path (supports .md format)
        #[arg(short, long, value_name = "FILE")]
        output: Option<std::path::PathBuf>,

        /// Generate AST JSON files
        #[arg(long)]
        ast: bool,

        /// Severities to ignore (comma-separated: low,medium,high,informational)
        #[arg(short, long, value_name = "SEVERITIES")]
        ignore: Option<String>,

        /// Specific rule IDs to ignore (comma-separated)
        #[arg(long, value_name = "RULE_IDS")]
        ignore_rules: Option<String>,
    },

    /// List all available detection rules
    ListRules {
        /// Filter by severity (high, medium, low, informational)
        #[arg(short, long)]
        severity: Option<String>,

        /// Show detailed information
        #[arg(short, long)]
        detailed: bool,
    },

    /// Show information about a specific rule
    RuleInfo {
        /// Rule ID to show information for
        rule_id: String,
    },

    /// Initialize a new analysis configuration file
    Init {
        /// Output path for config file
        #[arg(short, long, default_value = "eloizer.toml")]
        output: std::path::PathBuf,
    },

    /// Run analysis with a configuration file
    Config {
        /// Path to configuration file
        #[arg(short, long, default_value = "eloizer.toml")]
        config: std::path::PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize logger based on verbosity
    let log_level = if cli.verbose {
        "debug"
    } else if cli.quiet {
        "error"
    } else {
        "warn"  // Changed from "info" to hide INFO logs by default
    };

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(log_level))
        .format_timestamp(None)
        .init();

    // Disable colors if requested
    if cli.no_color {
        colored::control::set_override(false);
    }

    // Execute command
    match cli.command {
        Commands::Analyze {
            path,
            templates,
            output,
            ast,
            ignore,
            ignore_rules,
        } => commands::analyze::run(path, templates, output, ast, ignore, ignore_rules, cli.verbose, cli.quiet),

        Commands::ListRules { severity, detailed } => {
            commands::list_rules::run(severity, detailed)
        }

        Commands::RuleInfo { rule_id } => commands::rule_info::run(rule_id),

        Commands::Init { output } => commands::init::run(output),

        Commands::Config { config } => commands::config::run(config, cli.verbose, cli.quiet),
    }
}
