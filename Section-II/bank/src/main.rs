#[derive(Debug)]

struct Account {
    id: u32,
    balance: i32,
    holder: String,
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {
            id,
            balance: 0,
            holder,
        }
    }
}

#[derive(Debug)]
struct Bank {
    account: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Bank { account: vec![] }
    }
}

//helper function
fn print_account(account: Account) {
    println!("{:#?}", account)
}

fn main() {
    let bank = Bank::new();
    let account = Account::new(1, String::from("Sharath"));
    println!("{:#?}", bank);
    println!("{:#?}", account);
}
