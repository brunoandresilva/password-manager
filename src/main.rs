use aes::Aes128;
use aes::cipher::{
    BlockCipher, BlockEncrypt, BlockDecrypt, KeyInit,
    generic_array::GenericArray,
};
use std::io::{stdin, stdout, Read, Write};

#[derive(Debug)]
struct PasswordEntry{
    username: String,
    password: String,
    website: String,
}

// Implement methods for the PasswordEntry struct
impl PasswordEntry {
    // Constructor method to create a new PasswordEntry instance
    fn new(username: &str, password: &str, website: &str) -> Self {
        Self {
            username: username.to_string(),
            password: password.to_string(),
            website: website.to_string(),
        }
    }

    // Method to display the details of the password entry
    fn display_details(&self) {
        println!("Username: {}", self.username);
        println!("Password: {}", self.password);
        println!("Website URL: {}", self.website);
    }
}

fn login() -> bool{
    let mut username = String::new(); 
    let mut password: String = String::new();
    println!("Login");
    print!("Username: ");
    let _= stdout().flush();
    stdin().read_line(&mut username).expect("Did not enter a correct string");
    let username = username.trim().to_string();
    print!("Password: ");
    let _= stdout().flush();
    stdin().read_line(&mut password).expect("Did not enter a correct string");
    let password = password.trim().to_string();
    let mut login_succeed: bool = false;
    if username == "bikes" && password == "pass"{
        login_succeed = true;
    }
    return login_succeed;
}

fn main() {
    println!("Password Manager");
    println!("--------------------");
    let logged_in: bool = login();
    println!("{}", logged_in);
    if logged_in{
        // run code
        
        loop{
            let mut option = String::new();
            println!("1- Add new password");
            println!("2- Print my passwords");
            println!("3- Search password");
            println!("4- Exit");
            print!("Option: ");
            let _ = stdout().flush();
            stdin().read_line(&mut option).expect("Error readin option");
            let option: i32 = option.trim().parse().expect("Input is not i32");
            println!("Option chosen: {}", option);
            match option{
                1=> {println!("Add new password menu");
                    let mut test: PasswordEntry = PasswordEntry::new("user_default", "password_default", "deefault@test.com");
                    let mut passwords: Vec<PasswordEntry> = Vec::new();
                    passwords.push(test);
                    println!("Passwords vector: {:?}", passwords);},
                2=> println!("Print passwords menu"),
                3=> println!("Search password mennu"),
                4=> {println!("exit");
                        std::process::exit(0)},
                _=> println!("Not a valid option"),
            }
        }
    }
    else{
        std::process::exit(0);
    }
    
    
}
