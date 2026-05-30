#![allow(dead_code)]

use anyhow::{Context, Result};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

const NOTIF_FILE: &str = "notifications.json";

// ── Types ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AlertType {
    Updates,
    Audits,
    Security,
    Deployments,
}

impl std::fmt::Display for AlertType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AlertType::Updates => write!(f, "updates"),
            AlertType::Audits => write!(f, "audits"),
            AlertType::Security => write!(f, "security"),
            AlertType::Deployments => write!(f, "deployments"),
        }
    }
}

impl std::str::FromStr for AlertType {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "updates" => Ok(AlertType::Updates),
            "audits" => Ok(AlertType::Audits),
            "security" => Ok(AlertType::Security),
            "deployments" => Ok(AlertType::Deployments),
            _ => anyhow::bail!(
                "Unknown alert type '{}'. Valid: updates, audits, security, deployments",
                s
            ),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Channel {
    Email,
    Webhook,
    Cli,
}

impl std::fmt::Display for Channel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Channel::Email => write!(f, "email"),
            Channel::Webhook => write!(f, "webhook"),
            Channel::Cli => write!(f, "cli"),
        }
    }
}

impl std::str::FromStr for Channel {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "email" => Ok(Channel::Email),
            "webhook" => Ok(Channel::Webhook),
            "cli" => Ok(Channel::Cli),
            _ => anyhow::bail!(
                "Unknown channel '{}'. Valid: email, webhook, cli",
                s
            ),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Frequency {
    Instant,
    Daily,
    Weekly,
}

impl std::fmt::Display for Frequency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Frequency::Instant => write!(f, "instant"),
            Frequency::Daily => write!(f, "daily"),
            Frequency::Weekly => write!(f, "weekly"),
        }
    }
}

impl std::str::FromStr for Frequency {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "instant" => Ok(Frequency::Instant),
            "daily" => Ok(Frequency::Daily),
            "weekly" => Ok(Frequency::Weekly),
            _ => anyhow::bail!(
                "Unknown frequency '{}'. Valid: instant, daily, weekly",
                s
            ),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationRule {
    pub id: String,
    pub address: String,
    pub alert_types: Vec<AlertType>,
    pub channels: Vec<Channel>,
    pub frequency: Frequency,
    pub networks: Vec<String>,
    pub categories: Vec<String>,
    pub channel_target: Option<String>, // email address or webhook URL
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NotificationStore {
    pub rules: Vec<NotificationRule>,
}

// ── Persistence ───────────────────────────────────────────────────────────────

fn store_path() -> Option<PathBuf> {
    dirs::home_dir().map(|h| h.join(".soroban-registry").join(NOTIF_FILE))
}

fn load_store() -> Result<NotificationStore> {
    let Some(path) = store_path() else {
        return Ok(NotificationStore::default());
    };
    if !path.exists() {
        return Ok(NotificationStore::default());
    }
    let raw = fs::read_to_string(&path)
        .with_context(|| format!("Failed to read {}", path.display()))?;
    serde_json::from_str(&raw)
        .with_context(|| format!("Failed to parse {}", path.display()))
}

fn save_store(store: &NotificationStore) -> Result<()> {
    let Some(path) = store_path() else {
        anyhow::bail!("Could not resolve home directory");
    };
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create dir {}", parent.display()))?;
    }
    let json = serde_json::to_string_pretty(store)?;
    fs::write(&path, json)
        .with_context(|| format!("Failed to write {}", path.display()))
}

// ── Handlers ──────────────────────────────────────────────────────────────────

pub fn subscribe(
    address: &str,
    alert_types: Vec<String>,
    channels: Vec<String>,
    frequency: &str,
    networks: Vec<String>,
    categories: Vec<String>,
    channel_target: Option<String>,
) -> Result<()> {
    let parsed_alerts: Vec<AlertType> = alert_types
        .iter()
        .map(|s| s.parse())
        .collect::<Result<_>>()?;
    let parsed_channels: Vec<Channel> = channels
        .iter()
        .map(|s| s.parse())
        .collect::<Result<_>>()?;
    let freq: Frequency = frequency.parse()?;

    let mut store = load_store()?;

    // Prevent duplicate subscription for same address
    if store.rules.iter().any(|r| r.address == address) {
        println!(
            "{} Already subscribed to {}. Use {} to update.",
            "!".yellow(),
            address.bold(),
            "contract notification configure".cyan()
        );
        return Ok(());
    }

    let rule = NotificationRule {
        id: uuid::Uuid::new_v4().to_string(),
        address: address.to_string(),
        alert_types: parsed_alerts,
        channels: parsed_channels,
        frequency: freq,
        networks,
        categories,
        channel_target,
        created_at: chrono::Utc::now().to_rfc3339(),
    };

    println!("{} Subscribed to notifications for {}", "✓".green(), address.bold());
    println!("  Rule ID : {}", rule.id.dimmed());
    println!(
        "  Alerts  : {}",
        rule.alert_types.iter().map(|a| a.to_string()).collect::<Vec<_>>().join(", ")
    );
    println!(
        "  Channels: {}",
        rule.channels.iter().map(|c| c.to_string()).collect::<Vec<_>>().join(", ")
    );
    println!("  Frequency: {}", rule.frequency);

    store.rules.push(rule);
    save_store(&store)
}

pub fn unsubscribe(address: &str) -> Result<()> {
    let mut store = load_store()?;
    let before = store.rules.len();
    store.rules.retain(|r| r.address != address);

    if store.rules.len() == before {
        println!("{} No subscription found for {}", "!".yellow(), address.bold());
    } else {
        save_store(&store)?;
        println!("{} Unsubscribed from notifications for {}", "✓".green(), address.bold());
    }
    Ok(())
}

pub fn list(address: Option<&str>, json: bool) -> Result<()> {
    let store = load_store()?;
    let rules: Vec<&NotificationRule> = match address {
        Some(addr) => store.rules.iter().filter(|r| r.address == addr).collect(),
        None => store.rules.iter().collect(),
    };

    if json {
        println!("{}", serde_json::to_string_pretty(&rules)?);
        return Ok(());
    }

    if rules.is_empty() {
        println!("{}", "No notification rules configured.".dimmed());
        return Ok(());
    }

    println!("{}", "Notification Rules".bold().underline());
    for rule in &rules {
        println!();
        println!("  {} {}", "Address :".dimmed(), rule.address.bold());
        println!("  {} {}", "Rule ID :".dimmed(), rule.id.dimmed());
        println!(
            "  {} {}",
            "Alerts  :".dimmed(),
            rule.alert_types.iter().map(|a| a.to_string()).collect::<Vec<_>>().join(", ")
        );
        println!(
            "  {} {}",
            "Channels:".dimmed(),
            rule.channels.iter().map(|c| c.to_string()).collect::<Vec<_>>().join(", ")
        );
        println!("  {} {}", "Frequency:".dimmed(), rule.frequency);
        if !rule.networks.is_empty() {
            println!("  {} {}", "Networks:".dimmed(), rule.networks.join(", "));
        }
        if !rule.categories.is_empty() {
            println!("  {} {}", "Categories:".dimmed(), rule.categories.join(", "));
        }
        if let Some(target) = &rule.channel_target {
            println!("  {} {}", "Target  :".dimmed(), target);
        }
    }
    Ok(())
}

pub fn configure(
    address: &str,
    alert_types: Option<Vec<String>>,
    channels: Option<Vec<String>>,
    frequency: Option<String>,
    networks: Option<Vec<String>>,
    categories: Option<Vec<String>>,
    channel_target: Option<String>,
) -> Result<()> {
    let mut store = load_store()?;
    let rule = store
        .rules
        .iter_mut()
        .find(|r| r.address == address)
        .ok_or_else(|| {
            anyhow::anyhow!(
                "No subscription found for {}. Use 'subscribe' first.",
                address
            )
        })?;

    if let Some(types) = alert_types {
        rule.alert_types = types.iter().map(|s| s.parse()).collect::<Result<_>>()?;
    }
    if let Some(ch) = channels {
        rule.channels = ch.iter().map(|s| s.parse()).collect::<Result<_>>()?;
    }
    if let Some(freq) = frequency {
        rule.frequency = freq.parse()?;
    }
    if let Some(nets) = networks {
        rule.networks = nets;
    }
    if let Some(cats) = categories {
        rule.categories = cats;
    }
    if channel_target.is_some() {
        rule.channel_target = channel_target;
    }

    save_store(&store)?;
    println!("{} Updated notification rule for {}", "✓".green(), address.bold());
    Ok(())
}

pub fn test_notification(address: &str) -> Result<()> {
    let store = load_store()?;
    let rule = store
        .rules
        .iter()
        .find(|r| r.address == address)
        .ok_or_else(|| {
            anyhow::anyhow!(
                "No subscription found for {}. Use 'subscribe' first.",
                address
            )
        })?;

    println!("{} Sending test notification for {}", "→".cyan(), address.bold());

    for channel in &rule.channels {
        match channel {
            Channel::Cli => {
                println!(
                    "  {} [CLI] Test alert: contract {} has a new {} event.",
                    "✓".green(),
                    address,
                    rule.alert_types.first().map(|a| a.to_string()).unwrap_or_else(|| "update".into())
                );
            }
            Channel::Email => {
                let target = rule.channel_target.as_deref().unwrap_or("<no email configured>");
                println!(
                    "  {} [Email] Would send test alert to {}",
                    "✓".green(),
                    target
                );
            }
            Channel::Webhook => {
                let target = rule.channel_target.as_deref().unwrap_or("<no webhook URL configured>");
                println!(
                    "  {} [Webhook] Would POST test payload to {}",
                    "✓".green(),
                    target
                );
            }
        }
    }

    println!("{} Test notification dispatched.", "✓".green());
    Ok(())
}
