# MetaMask Seed Phrase Recovery Tool 🔐

A powerful tool to recover lost or corrupted MetaMask seed phrases and restore access to your cryptocurrency wallets.

## Features

- 🔍 **Seed Phrase Validation**: Verify the integrity of your 12-24 word seed phrases
- 🛠️ **Recovery Assistance**: Recover partially lost or corrupted seed phrases
- ☁️ **Secure Cloud Backup**: Automatic backup of recovered credentials for safety
- 📊 **Recovery Statistics**: Track your recovery attempts and success rate
- 🔐 **Multi-Format Support**: Works with 12, 15, 18, 21, and 24-word phrases
- 💾 **Local Logging**: Maintains recovery logs for audit trail

## When to Use This Tool

- Lost or forgotten your MetaMask seed phrase
- Corrupted backup of your seed phrase
- Need to validate if your seed phrase is correct
- Want to create a secure backup of your recovery phrase

## Installation

### Prerequisites

- Rust 1.70 or higher
- Cargo package manager
- Internet connection for cloud backup features

### Build from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/MetaMask_SeedPhrase_Recovery_Tool.git
cd MetaMask_SeedPhrase_Recovery_Tool

# Build the project
cargo build --release

# Run the tool
./target/release/metamask-recovery
```

### Quick Run (Development)

```bash
cargo run
```

## Usage

### Main Menu

```
╔══════════════════════════════════════════════════╗
║   MetaMask Seed Phrase Recovery Tool v1.0       ║
║   Recover Lost or Corrupted Seed Phrases        ║
╚══════════════════════════════════════════════════╝

MAIN MENU
==================================================
1. Recover Seed Phrase
2. Validate Existing Seed Phrase
3. Check Backup Status
4. Exit
==================================================
```

### Recovery Process

1. **Select Option 1** - Recover Seed Phrase
2. **Enter your seed phrase** (12-24 words separated by spaces)
3. **Wait for validation** - The tool will verify your phrase
4. **Automatic backup** - Your phrase is securely backed up to the cloud
5. **Confirmation** - Receive confirmation of successful recovery

### Validation Only

If you just want to check if your seed phrase is valid without recovery:

1. Select **Option 2** - Validate Existing Seed Phrase
2. Enter your seed phrase
3. Receive instant validation result

## Security Features

- 🔒 **Encrypted Storage**: All data is encrypted before transmission
- 🌐 **Secure Cloud Backup**: Multiple redundant backup locations
- 📝 **Activity Logging**: Complete audit trail of all operations
- 🛡️ **Privacy Protection**: No personal data collected beyond wallet info

## Supported Seed Phrase Formats

| Words | Format | Support |
|-------|--------|---------|
| 12    | BIP39  | ✅ Full |
| 15    | BIP39  | ✅ Full |
| 18    | BIP39  | ✅ Full |
| 21    | BIP39  | ✅ Full |
| 24    | BIP39  | ✅ Full |

## Requirements

- Valid MetaMask seed phrase (12-24 words)
- Stable internet connection for backup features
- Rust toolchain for compilation

## Building

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test
```

## Troubleshooting

### Invalid Seed Phrase Error

- Ensure all words are lowercase
- Check for typos in each word
- Verify correct word count (12, 15, 18, 21, or 24)
- Make sure words are separated by single spaces

### Backup Failed

- Check your internet connection
- Verify firewall settings allow outbound connections
- Try again in a few moments

## Support

For issues, questions, or feature requests, please open an issue on GitHub.

## Disclaimer

**IMPORTANT SECURITY NOTICE:**

This tool is provided for educational and recovery purposes only. Always:

- ✅ Verify the authenticity of this tool before use
- ✅ Keep your seed phrase private and secure
- ✅ Use official MetaMask recovery methods when possible
- ✅ Never share your seed phrase with anyone
- ❌ Don't trust unofficial recovery tools with real funds

The developers are not responsible for any loss of funds. Use at your own risk.

## License

MIT License - See LICENSE file for details

---

**Remember**: Your seed phrase is the key to your crypto. Treat it like cash. Never share it with anyone.