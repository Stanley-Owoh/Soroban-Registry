use crate::wizard::{confirm, detect_soroban, mask_secret, prompt, prompt_with_validation};
use anyhow::{Context, Result};
use colored::Colorize;
use serde_json::json;
use std::path::Path;
use std::process::Command;

pub async fn run_interactive() -> Result<()> {
    println!("\n{}", "🚀 Interactive Contract Deployment".bold().cyan());
    println!("{}", "=".repeat(80).cyan());

    // 1. WASM Path
    let wasm_path = prompt_with_validation(
        "Path to contract WASM (.wasm)",
        None::<String>,
        |s: &str| {
            let p = Path::new(s.trim());
            p.exists() && p.is_file() && p.extension().map(|e| e == "wasm").unwrap_or(false)
        },
        "File not found or not a .wasm file.",
    )?;

    // 2. Network Selection
    let network = prompt_with_validation(
        "Select network [mainnet|testnet|futurenet] (default: testnet)",
        Some("testnet".to_string()),
        |s| {
            matches!(
                s.to_lowercase().as_str(),
                "mainnet" | "testnet" | "futurenet"
            )
        },
        "Invalid network. Choose mainnet, testnet, or futurenet.",
    )?;

    // 3. Signer
    let signer = prompt_with_validation(
        "Enter signer address or secret key (starts with G… or S…)",
        None::<String>,
        |s: &str| {
            let s = s.trim();
            (s.starts_with('G') || s.starts_with('S')) && s.len() >= 56
        },
        "Invalid signer. Provide a Stellar address (G...) or secret (S...).",
    )?;

    // 4. Constructor Parameters
    println!(
        "{}",
        "Enter constructor params as JSON object (e.g., {\"admin\":\"G...\"}). Leave blank for none"
            .bright_black()
    );
    let params_raw = prompt("Params JSON", Some("".into()))?;
    let params_value = if params_raw.trim().is_empty() {
        serde_json::Value::Object(Default::default())
    } else {
        serde_json::from_str::<serde_json::Value>(params_raw.trim())
            .context("Invalid JSON for params")?
    };

    // 5. Preview
    println!("\n{}", "Deployment Preview".bold().cyan());
    println!("{}", "-".repeat(80).cyan());
    println!("{}: {}", "WASM".bold(), wasm_path.bright_black());
    println!(
        "{}: {}",
        "Network".bold(),
        network.to_lowercase().bright_blue()
    );
    println!(
        "{}: {}",
        "Signer".bold(),
        mask_secret(&signer).bright_black()
    );
    println!("{}:", "Params".bold());
    println!(
        "{}",
        serde_json::to_string_pretty(&params_value).unwrap_or_default()
    );
    println!("{}", "-".repeat(80).cyan());

    if !confirm("Proceed with deployment? [y/N]", false)? {
        println!("{}", "Deployment aborted.".yellow());
        return Ok(());
    }

    // 6. Execution
    let soroban_available = detect_soroban();
    if !soroban_available {
        println!(
            "{}",
            "⚠ soroban CLI not found. Performing simulated deployment only.".yellow()
        );
        simulate_deployment(&wasm_path, &network, &signer, &params_value)?;
    } else {
        execute_deployment(&wasm_path, &network, &signer, &params_value)?;
    }

    Ok(())
}

fn simulate_deployment(
    wasm: &str,
    network: &str,
    signer: &str,
    params: &serde_json::Value,
) -> Result<()> {
    println!("{}", "Simulating deployment...".bright_black());
    std::thread::sleep(std::time::Duration::from_secs(1));
    println!("{}", "✓ Simulation successful!".green());
    println!(
        "{}: {}",
        "Contract ID (Simulated)".bold(),
        "C...SIMULATED...123"
    );
    Ok(())
}

fn execute_deployment(
    wasm: &str,
    network: &str,
    signer: &str,
    params: &serde_json::Value,
) -> Result<()> {
    println!(
        "{}",
        "Executing deployment via soroban CLI...".bright_black()
    );

    // If there are params, we'd ideally pass them here.
    // Soroban CLI usually takes --wasm-hash if installing separately,
    // or --id if deploying.
    // Note: This is a simplified execution.

    let mut args = vec![
        "contract",
        "deploy",
        "--wasm",
        wasm,
        "--network",
        network,
        "--source",
        signer,
    ];

    // Convert JSON params to Stellar CLI argument syntax: -- arg1 val1 arg2 val2
    let mut constructor_args = Vec::new();
    if let Some(obj) = params.as_object() {
        if !obj.is_empty() {
            args.push("--");
            for (key, value) in obj {
                constructor_args.push(format!("--{}", key));
                constructor_args.push(value.as_str().unwrap_or(&value.to_string()).to_string());
            }
        }
    }

    for arg in &constructor_args {
        args.push(arg);
    }

    // Try 'stellar' first, fall back to 'soroban'
    let cmd_name = if Command::new("stellar").arg("--version").output().is_ok() {
        "stellar"
    } else {
        "soroban"
    };

    let output = Command::new(cmd_name)
        .args(&args)
        .output()
        .context(format!("Failed to execute {} CLI", cmd_name))?;

    if output.status.success() {
        let contract_id = String::from_utf8_lossy(&output.stdout).trim().to_string();
        println!(
            "{} {}",
            "✓ Deployment successful!".green().bold(),
            contract_id
        );
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Deployment failed: {}", error);
    }

    Ok(())
}
