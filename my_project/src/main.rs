struct Wallet {
    balance: i32,
}

impl Wallet {
    fn deposit(&mut self, amount: i32) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: i32) {
        self.balance -= amount;
    }
}

fn main() {
    let mut w = Wallet { balance: 100 };
    w.deposit(50);
    w.withdraw(30);
    println!("Day 8: balance = {}", w.balance);
}