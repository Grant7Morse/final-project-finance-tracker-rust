mod storage;
mod summary;
mod transaction;

use clap::{Parser, Subcommand};
use storage::{load_transactions, save_transactions};
use summary::{balance, filter_by_category, total_expenses, total_income};
use transaction::{Transaction, TransactionKind};

const FILE_PATH: &str = "transactions.json";

#[derive(Parser)]
#[command(name = "finance-tracker")]
#[command(about = "Track income and expenses")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        #[arg(short, long)]
        kind: String,

        #[arg(short, long)]
        amount: f64,

        #[arg(short, long)]
        category: String,

        #[arg(short, long, default_value = "")]
        note: String,
    },

    List,

    Summary,

    Category {
        category: String,
    },
}

fn main() {
    let cli = Cli::parse();

    let mut transactions = load_transactions(FILE_PATH).unwrap_or_else(|_| {
        println!("Could not load transactions. Starting fresh.");
        Vec::new()
    });

    match cli.command {
        Commands::Add {
            kind,
            amount,
            category,
            note,
        } => {
            let kind_enum = match kind.to_lowercase().as_str() {
                "income" => TransactionKind::Income,
                "expense" => TransactionKind::Expense,
                _ => {
                    println!("Invalid kind. Use 'income' or 'expense'");
                    return;
                }
            };

            match Transaction::new(kind_enum, amount, category, note) {
                Ok(tx) => {
                    transactions.push(tx);
                    save_transactions(FILE_PATH, &transactions).unwrap();
                    println!("Transaction added.");
                }
                Err(e) => println!("Error: {}", e),
            }
        }

        Commands::List => {
            for t in &transactions {
                println!("{:?}", t);
            }
        }

        Commands::Summary => {
            let income = total_income(&transactions);
            let expenses = total_expenses(&transactions);
            let bal = balance(&transactions);

            println!("Income: ${:.2}", income);
            println!("Expenses: ${:.2}", expenses);
            println!("Balance: ${:.2}", bal);
        }

        Commands::Category { category } => {
            let filtered = filter_by_category(&transactions, &category);

            for t in filtered {
                println!("{:?}", t);
            }
        }
    }
}
