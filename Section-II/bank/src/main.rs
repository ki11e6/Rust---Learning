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
fn print_account(account: &Account) {
    println!("{:#?}", account);
}

fn change_account(account: &mut Account) {
    account.balance += 100;
}
fn main() {
    let mut account = Account::new(1, String::from("Sharath"));

    change_account(&mut account);

    println!("{:#?}", account);
}
