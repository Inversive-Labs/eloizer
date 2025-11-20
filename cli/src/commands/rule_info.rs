use anyhow::Result;
use colored::*;
use rust_solana_analyzer::analyzer;

pub fn run(rule_id: String) -> Result<()> {
    let analyzer_instance = analyzer::create_analyzer();
    let rules = analyzer_instance.rules();

    // Find the rule
    let rule = rules
        .iter()
        .find(|r| r.id().to_lowercase() == rule_id.to_lowercase());

    match rule {
        Some(r) => {
            println!("\n{}\n", "📖 Rule Information".bright_cyan().bold());
            println!("  {} {}", "ID:".bold(), r.id());
            println!("  {} {}", "Title:".bold(), r.title());

            let (icon, color_fn): (&str, fn(&str) -> ColoredString) = match r.severity() {
                analyzer::Severity::High => ("🔴", |s: &str| s.red().bold()),
                analyzer::Severity::Medium => ("🟡", |s: &str| s.yellow().bold()),
                analyzer::Severity::Low => ("🟢", |s: &str| s.blue().bold()),
                analyzer::Severity::Informational => ("ℹ️", |s: &str| s.cyan()),
            };

            println!(
                "  {} {} {}\n",
                "Severity:".bold(),
                icon,
                color_fn(&format!("{:?}", r.severity()))
            );

            println!("  {}", "Description:".bold());
            println!("  {}\n", r.description());

            Ok(())
        }
        None => {
            eprintln!(
                "{} Rule not found: {}",
                "✗".red().bold(),
                rule_id.yellow()
            );
            eprintln!("\nUse {} to see all available rules\n", "eloizer list-rules".cyan());
            anyhow::bail!("Rule not found: {}", rule_id);
        }
    }
}
