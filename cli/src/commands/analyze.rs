use anyhow::Result;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use log::{debug, info, warn};
use rust_solana_analyzer::{analyzer, ast};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::time::Instant;

pub fn run(
    path: PathBuf,
    templates: Option<PathBuf>,
    output: Option<PathBuf>,
    generate_ast: bool,
    ignore: Option<String>,
    ignore_rules: Option<String>,
    verbose: bool,
    quiet: bool,
) -> Result<()> {
    // Print banner
    if !quiet {
        print_banner();
    }

    // Verify path exists
    if !path.exists() {
        eprintln!(
            "{} Path does not exist: {}",
            "✗".red().bold(),
            path.display().to_string().yellow()
        );
        anyhow::bail!("Path {} does not exist", path.display());
    }

    // Verify path is a directory
    if !path.is_dir() {
        eprintln!(
            "{} Path is not a directory: {}",
            "✗".red().bold(),
            path.display().to_string().yellow()
        );
        anyhow::bail!("Path {} is not a directory", path.display());
    }

    if !quiet {
        println!(
            "\n{} Analyzing directory: {}\n",
            "→".cyan().bold(),
            path.display().to_string().bright_blue()
        );
    }

    let start_time = Instant::now();

    // Create progress spinner
    let spinner = if !quiet {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.cyan} {msg}")
                .unwrap(),
        );
        pb.set_message("Scanning for Rust files...");
        pb.enable_steady_tick(std::time::Duration::from_millis(100));
        Some(pb)
    } else {
        None
    };

    // Process directory
    let results = ast::parser::process_directory(&path);

    if let Some(pb) = &spinner {
        pb.finish_and_clear();
    }

    if results.is_empty() {
        eprintln!(
            "\n{} No Rust files found in {}",
            "⚠".yellow().bold(),
            path.display().to_string().yellow()
        );
        return Ok(());
    }

    if !quiet {
        println!(
            "{} Found {} Rust file(s) to analyze\n",
            "✓".green().bold(),
            results.len().to_string().bright_green().bold()
        );
    }

    // Generate AST if requested
    if generate_ast {
        if !quiet {
            println!("{} Generating AST JSON files...\n", "→".cyan().bold());
        }

        for (file_path, ast_data) in &results {
            let json = ast::json::ast_to_json(ast_data);
            let mut json_path = file_path.clone();
            json_path.set_extension("json");
            fs::write(&json_path, json)?;

            if !quiet {
                println!(
                    "  {} {}",
                    "✓".green(),
                    json_path.display().to_string().dimmed()
                );
            }
        }

        if !quiet {
            println!();
        }
    }

    // Run security analysis
    if !quiet {
        println!(
            "{} Running security analysis...\n",
            "🔍".to_string().bold()
        );
    }

    // Create analysis options
    let mut options = analyzer::AnalysisOptions::default();
    options.generate_ast = generate_ast;
    options.custom_templates_path = templates.map(|p| p.to_string_lossy().to_string());
    options.include_rule_types = vec![
        analyzer::RuleType::Solana,
        analyzer::RuleType::Anchor,
        analyzer::RuleType::General,
    ];

    // Parse severities to ignore
    if let Some(ignore_str) = ignore {
        for sev in ignore_str.split(',') {
            match sev.trim().to_lowercase().as_str() {
                "high" => options.ignore_severities.push(analyzer::Severity::High),
                "medium" => options.ignore_severities.push(analyzer::Severity::Medium),
                "low" => options.ignore_severities.push(analyzer::Severity::Low),
                "informational" => options
                    .ignore_severities
                    .push(analyzer::Severity::Informational),
                _ => warn!("Unknown severity level: {}", sev),
            }
        }
    }

    // Parse rule IDs to ignore
    if let Some(ignore_rules_str) = ignore_rules {
        for rule_id in ignore_rules_str.split(',') {
            options.ignore_rules.push(rule_id.trim().to_string());
        }
    }

    // Create analyzer
    let analyzer_instance = analyzer::create_analyzer_with_options(options);

    let analysis_spinner = if !quiet {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.cyan} {msg}")
                .unwrap(),
        );
        pb.set_message("Analyzing code for vulnerabilities...");
        pb.enable_steady_tick(std::time::Duration::from_millis(100));
        Some(pb)
    } else {
        None
    };

    // Run analysis
    match analyzer_instance.analyze_files(&results) {
        Ok(analysis_result) => {
            if let Some(pb) = &analysis_spinner {
                pb.finish_and_clear();
            }

            let elapsed = start_time.elapsed();

            if !quiet {
                println!(
                    "{} Analysis completed in {:.2}s\n",
                    "✓".green().bold(),
                    elapsed.as_secs_f64()
                );
            }

            // Show summary
            if !quiet {
                print_summary(&analysis_result);
            }

            // Save or display results
            if let Some(output_path) = output {
                save_report(&analysis_result, &output_path, &path, quiet)?;
            } else if !quiet {
                print_findings(&analysis_result, verbose);
            }
        }
        Err(e) => {
            if let Some(pb) = &analysis_spinner {
                pb.finish_and_clear();
            }
            eprintln!(
                "\n{} Analysis failed: {}",
                "✗".red().bold(),
                e.to_string().red()
            );
            anyhow::bail!("Analysis failed: {}", e);
        }
    }

    if !quiet {
        println!(
            "\n{} Analysis completed successfully!\n",
            "✓".green().bold()
        );
    }

    Ok(())
}

fn print_banner() {
    println!("{}", r#"
███████╗██╗      ██████╗ ██╗███████╗███████╗██████╗ 
██╔════╝██║     ██╔═══██╗██║╚══███╔╝██╔════╝██╔══██╗
█████╗  ██║     ██║   ██║██║  ███╔╝ █████╗  ██████╔╝
██╔══╝  ██║     ██║   ██║██║ ███╔╝  ██╔══╝  ██╔══██╗
███████╗███████╗╚██████╔╝██║███████╗███████╗██║  ██║
╚══════╝╚══════╝ ╚═════╝ ╚═╝╚══════╝╚══════╝╚═╝  ╚═╝                                                                                                                                                                                                                                                                             
    "#.bright_cyan().bold());
}

fn print_summary(analysis_result: &analyzer::AnalysisResult) {
    println!("{}", "═".repeat(70).dimmed());
    println!("\n{}\n", "📊 ANALYSIS SUMMARY".bright_white().bold());

    let total = analysis_result.findings.len();

    if total == 0 {
        println!("  {} No vulnerabilities found!\n", "✓".green().bold());
        return;
    }

    println!("  Total findings: {}\n", total.to_string().bold());

    // Count by severity
    let mut severity_counts = HashMap::new();
    for (severity, count) in &analysis_result.stats.findings_by_severity {
        severity_counts.insert(severity, *count);
    }

    // Display by severity
    for severity in &[
        analyzer::Severity::High,
        analyzer::Severity::Medium,
        analyzer::Severity::Low,
        analyzer::Severity::Informational,
    ] {
        if let Some(count) = severity_counts.get(severity) {
            let (icon, color_fn): (&str, fn(&str) -> ColoredString) = match severity {
                analyzer::Severity::High => ("🔴", |s: &str| s.red().bold()),
                analyzer::Severity::Medium => ("🟡", |s: &str| s.yellow().bold()),
                analyzer::Severity::Low => ("🟢", |s: &str| s.blue().bold()),
                analyzer::Severity::Informational => ("ℹ️", |s: &str| s.cyan()),
            };

            println!(
                "  {} {:<15} {}",
                icon,
                format!("{:?}:", severity),
                color_fn(&count.to_string())
            );
        }
    }

    println!();
}

fn print_findings(analysis_result: &analyzer::AnalysisResult, verbose: bool) {
    if analysis_result.findings.is_empty() {
        return;
    }

    println!("{}", "═".repeat(70).dimmed());
    println!("\n{}\n", "🔍 DETAILED FINDINGS".bright_white().bold());

    // Group by severity
    let mut findings_by_severity = HashMap::new();
    for finding in &analysis_result.findings {
        findings_by_severity
            .entry(&finding.severity)
            .or_insert_with(Vec::new)
            .push(finding);
    }

    let mut index = 1;
    for severity in &[
        analyzer::Severity::High,
        analyzer::Severity::Medium,
        analyzer::Severity::Low,
        analyzer::Severity::Informational,
    ] {
        if let Some(findings) = findings_by_severity.get(severity) {
            let (icon, header_color): (&str, fn(&str) -> ColoredString) = match severity {
                analyzer::Severity::High => ("🔴", |s: &str| s.red().bold()),
                analyzer::Severity::Medium => ("🟡", |s: &str| s.yellow().bold()),
                analyzer::Severity::Low => ("🟢", |s: &str| s.blue().bold()),
                analyzer::Severity::Informational => ("ℹ️", |s: &str| s.cyan().bold()),
            };

            println!(
                "{} {}\n",
                icon,
                header_color(&format!("{:?} Severity", severity))
            );

            for finding in findings {
                // Aplicar color según severidad
                let description_colored = match severity {
                    analyzer::Severity::High => finding.description.red().bold().to_string(),
                    analyzer::Severity::Medium => finding.description.yellow().bold().to_string(),
                    analyzer::Severity::Low => finding.description.blue().bold().to_string(),
                    analyzer::Severity::Informational => finding.description.cyan().to_string(),
                };
                
                let location_colored = match severity {
                    analyzer::Severity::High => format!("{}:{}", finding.location.file, finding.location.line).red().to_string(),
                    analyzer::Severity::Medium => format!("{}:{}", finding.location.file, finding.location.line).yellow().to_string(),
                    analyzer::Severity::Low => format!("{}:{}", finding.location.file, finding.location.line).blue().to_string(),
                    analyzer::Severity::Informational => format!("{}:{}", finding.location.file, finding.location.line).cyan().to_string(),
                };

                println!(
                    "  {}. {}",
                    index.to_string().bold(),
                    description_colored
                );
                println!(
                    "     {} {}",
                    "📍",
                    location_colored
                );

                if verbose {
                    if let Some(snippet) = &finding.code_snippet {
                        let snippet_colored = match severity {
                            analyzer::Severity::High => snippet.red().to_string(),
                            analyzer::Severity::Medium => snippet.yellow().to_string(),
                            analyzer::Severity::Low => snippet.blue().to_string(),
                            analyzer::Severity::Informational => snippet.cyan().to_string(),
                        };
                        println!("     {} {}", "Code:".dimmed(), snippet_colored);
                    }
                    if !finding.recommendations.is_empty() {
                        println!(
                            "     {} {}",
                            "💡",
                            finding.recommendations.join(", ").green()
                        );
                    }
                }

                println!();
                index += 1;
            }
        }
    }
}

fn save_report(
    analysis_result: &analyzer::AnalysisResult,
    output_path: &PathBuf,
    project_path: &PathBuf,
    quiet: bool,
) -> Result<()> {
    let report_generator = analyzer::reporting::ReportGenerator::new(
        analysis_result.findings.clone(),
        project_path.to_string_lossy().to_string(),
    );

    let output_str = output_path.to_string_lossy();
    let final_path = if output_str.ends_with(".md") || output_str.ends_with(".markdown") {
        output_path.clone()
    } else {
        let mut md_path = output_path.clone();
        md_path.set_extension("md");
        md_path
    };

    match report_generator.save_markdown_report(&final_path.to_string_lossy()) {
        Ok(()) => {
            if !quiet {
                println!(
                    "\n{} Report saved to: {}\n",
                    "📄".bold(),
                    final_path.display().to_string().bright_green()
                );
            }
            Ok(())
        }
        Err(e) => {
            eprintln!(
                "\n{} Failed to save report: {}\n",
                "✗".red().bold(),
                e.to_string().red()
            );
            Err(e.into())
        }
    }
}
