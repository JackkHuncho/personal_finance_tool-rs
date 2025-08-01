# Personal Finance Tool

A command-line personal finance management tool built in Rust for tracking transactions, managing budgets, and analyzing spending patterns.

## Features

- **Transaction Management**: Add and list financial transactions with categories
- **Category System**: Built-in categories for common expenses and income types
- **Custom Categories**: Support for custom income and expense categories
- **Data Persistence**: Automatic JSON-based data storage
- **Pretty Tables**: Clean, formatted display of transaction data
- **Date Validation**: Robust date parsing and validation
- **Extensible Architecture**: Modular design for future enhancements

## Installation

### Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)

### Build from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/personal_finance_tool.git
cd personal_finance_tool

# Build the project
cargo build --release

# Install globally (optional)
cargo install --path .
```

## Usage

### Adding Transactions

Add a new transaction with required fields:

```bash
personal_finance_tool add "12/25/2024" "100.50" "Entertainment" --note "Christmas gift"
```

**Parameters:**
- `date`: Transaction date in MM/DD/YYYY format
- `amount`: Transaction amount (positive for expenses, negative for income)
- `category`: Transaction category (see Categories section)
- `--note` (optional): Additional notes about the transaction

### Listing Transactions

View all stored transactions in a formatted table:

```bash
personal_finance_tool list
```

## Categories

### Built-in Categories

**Expenses:**
- Food
- Transport
- Entertainment
- Shopping
- Bills
- Healthcare
- Automotive

**Income:**
- Salary
- Investment
- Freelance

### Custom Categories

You can create custom categories using the format `type:name`:

```bash
# Custom income category
personal_finance_tool add "12/25/2024" "500.00" "income:Bonus" --note "Year-end bonus"

# Custom expense category
personal_finance_tool add "12/25/2024" "75.00" "expense:Pet Care" --note "Vet visit"
```

## Data Storage

Transaction data is automatically stored in JSON format in your system's data directory:
- **macOS**: `~/Library/Application Support/personal_finance_tool/`
- **Linux**: `~/.local/share/personal_finance_tool/`
- **Windows**: `%APPDATA%\personal_finance_tool\`

The data file is created automatically on first use.

## Project Structure

```
src/
├── main.rs              # Application entry point
├── cli/                 # Command-line interface
│   ├── commands.rs      # CLI command definitions
│   ├── display.rs       # Display formatting
│   └── mod.rs          # CLI module
├── models/              # Data models
│   ├── transaction.rs   # Transaction model and logic
│   ├── category.rs      # Category definitions
│   ├── budget.rs        # Budget management (planned)
│   └── mod.rs          # Models module
├── storage/             # Data persistence
│   ├── file_handler.rs  # File I/O operations
│   ├── backup.rs        # Backup functionality (planned)
│   ├── encryption.rs    # Data encryption (planned)
│   └── mod.rs          # Storage module
├── analysis/            # Financial analysis (planned)
│   ├── calculator.rs    # Financial calculations
│   ├── reports.rs       # Report generation
│   ├── trends.rs        # Trend analysis
│   └── mod.rs          # Analysis module
└── utils/               # Utility functions
    ├── date_utils.rs    # Date handling utilities
    ├── validation.rs    # Input validation
    └── mod.rs          # Utils module
```

## Development

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture
```

### Building for Development

```bash
# Debug build
cargo build

# Release build
cargo build --release
```

## Planned Features

- [ ] Budget management and tracking
- [ ] Financial reports and analytics
- [ ] Spending trend analysis
- [ ] Data backup and restore
- [ ] Data encryption for security
- [ ] CSV import/export functionality
- [ ] Monthly/yearly summaries
- [ ] Category-based spending limits

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Author

**Kendrix Henderson** - kendrixhenderson@gmail.com

## Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- CLI framework: [Clap](https://github.com/clap-rs/clap)
- Table formatting: [prettytable-rs](https://github.com/phsym/prettytable-rs)
- Date handling: [Chrono](https://github.com/chronotope/chrono) 