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

    fn deposit(&mut self, amount: i32) -> i32 {
        self.balance += amount;
        self.balance
    }

    fn withdraw(&mut self, amount: i32) -> i32 {
        if self.balance >= amount {
            self.balance -= amount;
            self.balance
        } else {
            println!("Cannot withdraw due to insufficient funds");
            self.balance
        }
    }

    fn summary(&self) -> String {
        format!(
            "Account {}: Holder: {}, Balance: {}",
            self.id, self.holder, self.balance
        )
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

    fn add_account(&mut self, account: Account) {
        self.account.push(account);
    }

    fn total_balance(&self) -> i32 {
        self.account.iter().map(|acc| acc.balance).sum()
    }

    fn summary(&self) -> Vec<String> {
        self.account.iter().map(|acc| acc.summary()).collect()
    }
}

fn main() {
    let mut bank = Bank::new();
    let mut account1 = Account::new(1, String::from("Alice"));
    let mut account2 = Account::new(2, String::from("Bob"));

    account1.deposit(1000);
    account1.withdraw(400);
    account2.deposit(2000);
    account2.withdraw(250);

    let account1summary = account1.summary();
    println!("{}", account1summary);

    bank.add_account(account1);
    bank.add_account(account2);
    let total = bank.total_balance();
    let summaries = bank.summary();

    println!("{:#?}", bank);
    println!("Total balance in bank: {}", total);
    println!("Account summaries: {:#?}", summaries);
}
