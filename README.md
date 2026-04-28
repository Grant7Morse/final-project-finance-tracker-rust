What it does

I created a personal finance tracker that logs income and expenses. As you enter in incomes or expenses you can put it in a category and add a note to each entry.All of the data is stored on a JSON file. The tool is made to be an easy to use finance tracker that even someone not used to using it can use.

Installation

1. For installation, first be sure to have rust installed: website to download https://www.rust-lang.org/tools/install

2. Clone the repositiory https://github.com/Grant7Morse/final-project-finance-tracker-rust.git

3. Go into the project folder and using the terminal type cd final-project-finance-tracker-rust

4. Build the project by typing: cargo build. To verify installation run: cargo test. Then run the program by typing cargo run -- --help.


Useage

This will all be used using cargo and typing in the command line.

To add an income input:  cargo run -- add --kind income --amount 500 --category work --note "paycheck". This will add 500 in the work category noted paycheck. The notes and amounts can be edited based on what is wanted to be input. The expected output: Transaction added.

To add an expense input: cargo run -- add --kind expense --amount 20 --category food --note "lunch". This will subtract 20 from the total in the category food noted lunch. The expected output: Transaction added.

To list all transactions: cargo run -- list.
Expected output of this is:
Transaction { kind: Income, amount: 500.0, category: "work", note: "paycheck" }
Transaction { kind: Expense, amount: 20.0, category: "food", note: "lunch" }

To view a summary: cargo run -- summary
Expected output of this: 

Income: $500.00
Expenses: $20.00
Balance: $480.00

Examples

cargo run -- add --kind income --amount 1000 --category salary
cargo run -- add --kind expense --amount 50 --category groceries
cargo run -- add --kind expense --amount 25 --category food
cargo run -- summary

Expected output 

Income: $1000.00
Expenses: $75.00
Balance: $925.00

Known Limitations

The biggest limitation I can think of right now is the inability to edit transactions. 
