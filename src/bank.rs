
pub enum Asset {
    Stocks,
    Bonds,
    Funds,
    Cash
}

#[derive(Clone, Copy)]
enum Currency {
    USD,
    EUR,
    BRL,
}

struct Account {
    balance: f64,
    currency: Currency,
}

impl Account {
    fn new(balance: f64, currency: Currency) -> Account {
        Account {
            balance: balance,
            currency: currency
        }
    }

    fn deposit(&mut self, amount: f64) {
        self.balance += amount
    }

    fn withdraw(&mut self, amount: f64) {
        self.balance -= amount
    }

    fn capture_account(&self) -> Account{
        let acc = Account{balance: self.balance, currency: self.currency};
        acc
    }

    fn capture_balance(&self) -> f64 {
        self.balance
    }
}

impl Asset {
    
    pub fn price(&self) -> f64 {
        match self {
            Asset::Stocks => 10.0,
            Asset::Bonds => 20.0,
            Asset::Funds => 30.0,
            Asset::Cash => 40.0
        }
    }
}

pub fn calc_portifolio() {
    let portifolio = [Asset::Stocks, Asset::Bonds, Asset::Funds, Asset::Cash];

    let total: f64 = portifolio.iter().map(|asset| asset.price()).sum();
    
    println!("O valor total do portifólio é R$ {}", total);
}

pub fn create_account(){
    
    let mut acc = Account::new(20.0, Currency::USD);
    println!("{}", acc.capture_balance());

    acc.deposit(55.5);
    println!("{}", acc.capture_balance());

    acc.withdraw(5.5);
    println!(" você fez um saque, seu saldo agora é: {}", acc.capture_balance());

}