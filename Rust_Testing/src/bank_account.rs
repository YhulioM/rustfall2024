#[derive(Debug)]
pub struct BankAccount {
    balance: f64,
}

impl BankAccount {
    pub fn new(initial_balance: f64) -> BankAccount {
        BankAccount { balance:initial_balance }
        //self.balance = initial_balance;
    }

    pub fn deposit(&mut self, amount: f64) {
        if amount > 0.0 {
            self.balance += amount;
        }
    }

    pub fn withdraw(&mut self, amount: f64) {
        if amount < self.balance && amount > 0.0 {
            self.balance -= amount;
        }
    }

    pub fn balance(&self) -> f64 {
        return self.balance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_account() {
        // Write a test for creating a new account
        let mut account = BankAccount::new(60.0);
        assert_eq!(account.balance, 60.0);
    }

    #[test]
    fn test_deposit() {
        // Write a test for depositing money
        let mut account = BankAccount::new(60.0);
        account.deposit(20.0);
        assert_eq!(account.balance, 80.0)
    }

    #[test]
    fn test_withdraw() {
        // Write a test for withdrawing money
        let mut account = BankAccount::new(60.0);
        account.withdraw(20.0);
        assert_eq!(account.balance, 40.0)
    }

    #[test]
    fn check_balance() {
        // Write a test for checking the balance
        let mut account = BankAccount::new(80.0);
        assert_eq!(account.balance, 80.0);
    }

    #[test]
    fn negative_amounts() {
        // Write a test for depositing/withdrawing negative amounts
        let mut account = BankAccount::new(60.0);
        account.withdraw(-10.0);
        assert_eq!(account.balance, 60.0);

        account.deposit(-20.0);
        assert_eq!(account.balance, 60.0)
    }

    #[test]
    fn overdraft() {
        // Write a test for withdrawing more than the balance
        let mut account = BankAccount::new(60.0);
        account.withdraw(100.0);
        assert_eq!(account.balance, 60.0);
    }
    
}