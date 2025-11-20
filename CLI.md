# CLI Documentation

## Installation

### From Repository

Install the CLI tool globally from the repository:

```bash
cargo install --path cli
```

This will install the `eloizer` command globally, making it available from anywhere in your system.

### Verify Installation

```bash
eloizer --version
```

### Uninstall

```bash
cargo uninstall eloizer
```

## Usage

### Basic Commands

#### Analyze a Project

```bash
eloizer analyze --path <PATH>
```

Example:
```bash
eloizer analyze --path test-securty-solana/programs/test-securty-solana/src
```

#### Generate Report

```bash
eloizer analyze --path <PATH> --output report.md
```

#### List Available Rules

```bash
eloizer list-rules
```

Filter by severity:
```bash
eloizer list-rules --severity high
```

Show detailed information:
```bash
eloizer list-rules --severity high --detailed
```

#### Get Rule Information

```bash
eloizer rule-info <RULE_ID>
```

Example:
```bash
eloizer rule-info pda-sharing-cwe-345
```

#### Initialize Configuration File

```bash
eloizer init
```

This creates a `eloizer.toml` configuration file in the current directory.

#### Run with Configuration

```bash
eloizer config
```

Or specify a custom configuration file:
```bash
eloizer config --config my-config.toml
```

### Command Options

#### analyze

```
eloizer analyze [OPTIONS] --path <PATH>

Options:
  -p, --path <PATH>              Path to Solana project directory or Rust file
  -t, --templates <DIR>          Custom templates path
  -o, --output <FILE>            Output report file path (supports .md format)
      --ast                      Generate AST JSON files
  -i, --ignore <SEVERITIES>      Severities to ignore (comma-separated: low,medium,high,informational)
      --ignore-rules <RULE_IDS>  Specific rule IDs to ignore (comma-separated)
      --no-color                 Disable colored output
  -v, --verbose                  Enable verbose output
  -q, --quiet                    Quiet mode (errors only)
  -h, --help                     Print help
```

#### list-rules

```
eloizer list-rules [OPTIONS]

Options:
  -s, --severity <SEVERITY>  Filter by severity (high, medium, low, informational)
  -d, --detailed             Show detailed information
  -h, --help                 Print help
```

#### rule-info

```
eloizer rule-info <RULE_ID>

Arguments:
  <RULE_ID>  Rule ID to show information for

Options:
  -h, --help  Print help
```

#### init

```
eloizer init [OPTIONS]

Options:
  -o, --output <FILE>  Output path for config file [default: solana-analyzer.toml]
  -h, --help           Print help
```

#### config

```
eloizer config [OPTIONS]

Options:
  -c, --config <FILE>  Path to configuration file [default: solana-analyzer.toml]
  -h, --help           Print help
```

### Global Options

These options work with all commands:

```
  --no-color   Disable colored output
  -v, --verbose  Enable verbose output
  -q, --quiet    Quiet mode (errors only)
  -h, --help     Print help
  -V, --version  Print version
```

## Examples

### Basic Analysis

Analyze a Solana project and display results in the terminal:

```bash
eloizer analyze --path src/
```

### Generate Markdown Report

Analyze and save results to a Markdown file:

```bash
eloizer analyze --path src/ --output security-report.md
```

### Ignore Low Severity Issues

Analyze but ignore low and informational severity findings:

```bash
eloizer analyze --path src/ --ignore low,informational
```

### Ignore Specific Rules

Analyze but ignore specific rules by their IDs:

```bash
eloizer analyze --path src/ --ignore-rules unsafe-code,missing-error-handling
```

### Generate AST Files

Generate AST JSON files along with the analysis:

```bash
eloizer analyze --path src/ --ast
```

### Verbose Output

Show detailed debug information during analysis:

```bash
eloizer analyze --path src/ --verbose
```

### Quiet Mode

Show only errors:

```bash
eloizer analyze --path src/ --quiet
```

### View High Severity Rules

List all high severity detection rules with details:

```bash
eloizer list-rules --severity high --detailed
```

### Configuration File Workflow

Create and use a configuration file for consistent analysis:

```bash
# Create configuration file
eloizer init --output .eloizer.toml

# Edit the configuration file as needed
# Then run analysis using the configuration
eloizer config --config .eloizer.toml
```

## Configuration File Format

The configuration file uses TOML format:

```toml
[analysis]
path = "src/"
generate_ast = false

[output]
report_file = "security-report.md"

[rules]
ignore_severities = ["low"]
ignore_rules = []
include_rule_types = ["solana", "anchor", "general"]

[display]
verbose = false
quiet = false
no_color = false
```

## Output Format

### Terminal Output

The CLI provides colored, formatted output with:

- Banner with tool name and version
- Progress indicators during analysis
- Summary of findings by severity
- Detailed findings with file locations
- Color-coded severity levels (High: red, Medium: yellow, Low: blue, Informational: cyan)

### Markdown Report

Generated reports include:

- Executive summary
- Files analyzed
- Findings grouped by severity
- Detailed descriptions and locations
- Code snippets
- Recommendations

## Exit Codes

- `0` - Success
- `1` - Error during execution

## Environment Variables

The CLI respects the following environment variables:

- `NO_COLOR` - Disable colored output (set to any value)
- `RUST_LOG` - Set log level (only with --verbose flag)

## Troubleshooting

### Command Not Found

If `eloizer` is not found after installation, ensure that `~/.cargo/bin` is in your PATH:

```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

Add this line to your shell configuration file (`~/.bashrc`, `~/.zshrc`, etc.) to make it permanent.

### Permission Denied

If you encounter permission errors during installation:

```bash
cargo install --path cli --force
```

### Outdated Installation

To update to the latest version:

```bash
cd /path/to/rust-solana-analyzer
git pull
cargo install --path cli --force
```
