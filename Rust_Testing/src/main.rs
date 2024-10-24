mod bank_account;

use bank_account::BankAccount;

fn main(){
    let mut account = BankAccount::new(60.0);

    println!("You created and account with a balance of: {}",account.balance());
    
    account.deposit(20.0);
    println!("Your new balance after deposit is: {}",account.balance());

    account.withdraw(10.0);
    println!("Your balance after withdraw is: {}", account.balance());
}
