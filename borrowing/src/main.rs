//References and Borrowing in Rust
//Safety and Performance

//References: Enable you to borrow values without taking ownership.
// Imutable references allow you to read data without modifying it.
// Mutable references allow you to modify data, but only one mutable reference can exist at a time
// Create References by using the `&` operator




//fn main() {
    //let mut x: i32 = 5;
    //Y references the value of x without taking ownership
    //let y: &i32 = &x;

    // This line will cause a compile-time error because `y` is an immutable reference.
    // *y +=1; 

    //let z: &mut i32 = &mut x;
    //*z +=1; // This is allowed because `z` is a mutable reference to

    //println!("Value of x: {}", x);
    
//}


fn main(){
    let mut account = BankAccount{
        owner: "Alice".to_string(),
        balance: 150.55,
    };
    // Immutable borrow to check the balance
    account.check_balance();

    //mutable borrow to withdraw money
    account.withdraw(50.45);
}

struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn withdraw(&mut self, amount: f64){
        println!("Withdrawing {} from {:.2}", amount, self.owner);
        self.balance -= amount;
        self.check_balance();
    }

    fn check_balance(&self){
        println!("Account owned by {} has a balance of {:.2}", self.owner, self.balance);
    }
}