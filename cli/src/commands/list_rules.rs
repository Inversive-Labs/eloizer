use anyhow::Result;
use colored::*;
use rust_solana_analyzer::analyzer;

pub fn run(severity_filter: Option<String>, detailed: bool) -> Result<()> {
    println!("\n{}\n", "📋 Available Detection Rules".bright_cyan().bold());

    // Create analyzer to get rules
    let analyzer_instance = analyzer::create_analyzer();
    let rules = analyzer_instance.rules();

    // Filter by severity if specified
    let filtered_rules: Vec<_> = if let Some(sev_str) = severity_filter {
        let target_severity = match sev_str.to_lowercase().as_str() {
            "high" => analyzer::Severity::High,
            "medium" => analyzer::Severity::Medium,
            "low" => analyzer::Severity::Low,
            "informational" => analyzer::Severity::Informational,
            _ => {
                eprintln!("{} Unknown severity: {}", "✗".red().bold(), sev_str);
                anyhow::bail!("Unknown severity: {}", sev_str);
            }
        };
        rules
            .iter()
            .filter(|r| r.severity() == target_severity)
            .collect()
    } else {
        rules.iter().collect()
    };

    if filtered_rules.is_empty() {
        println!("  {} No rules found", "⚠".yellow());
        return Ok(());
    }

    // Group by severity
    for severity in &[
        analyzer::Severity::High,
        analyzer::Severity::Medium,
        analyzer::Severity::Low,
        analyzer::Severity::Informational,
    ] {
        let severity_rules: Vec<_> = filtered_rules
            .iter()
            .filter(|r| r.severity() == *severity)
            .collect();

        if severity_rules.is_empty() {
            continue;
        }

        let (icon, color_fn): (&str, fn(&str) -> ColoredString) = match severity {
            analyzer::Severity::High => ("🔴", |s: &str| s.red().bold()),
            analyzer::Severity::Medium => ("🟡", |s: &str| s.yellow().bold()),
            analyzer::Severity::Low => ("🟢", |s: &str| s.blue().bold()),
            analyzer::Severity::Informational => ("ℹ️", |s: &str| s.cyan()),
        };

        println!(
            "{} {} ({} rules)\n",
            icon,
            color_fn(&format!("{:?} Severity", severity)),
            severity_rules.len()
        );

        for rule in severity_rules {
            println!("  • {} - {}", rule.id().bold(), rule.title());

            if detailed {
                println!("    {}", rule.description().dimmed());
                println!();
            }
        }

        println!();
    }

    println!(
        "Total: {} rules\n",
        filtered_rules.len().to_string().bold()
    );

    Ok(())
}
