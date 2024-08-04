use std::io::{self, Write};

struct UserData {
    name: String,
    email: String,
    age: u8,
}

trait Display {
    fn display_info(&self);
}

trait Operations {
    fn add_user(&mut self);
    fn remove_user(&mut self);
    fn modify_user(&mut self);
    fn display_users(&self);
}

impl Display for UserData {
    fn display_info(&self) {
        println!("Name: {}, Email: {}, Age: {}", self.name, self.email, self.age);
    }
}

impl Operations for Vec<UserData> {
    fn add_user(&mut self) {
        let mut name = String::new();
        let mut email = String::new();
        let mut age_str = String::new();

        println!("Enter name:");
        io::stdin().read_line(&mut name).unwrap();
        let name = name.trim().to_string().to_lowercase();

        println!("Enter email:");
        io::stdin().read_line(&mut email).unwrap();
        let email = email.trim().to_string();

        println!("Enter age:");
        io::stdin().read_line(&mut age_str).unwrap();
        let age: u8 = age_str.trim().parse().expect("Please enter a valid age");

        self.push(UserData { name, email, age });
        println!("User added successfully!");
    }

    fn remove_user(&mut self) {
        let mut name = String::new();
        println!("Enter the name of the user to remove:");
        io::stdin().read_line(&mut name).unwrap();
        let name = name.trim().to_lowercase();

        if let Some(pos) = self.iter().position(|user| user.name == name) {
            self.remove(pos);
            println!("User removed successfully!");
        } else {
            println!("User not found.");
        }
    }

    fn modify_user(&mut self) {
        let mut name = String::new();
        println!("Enter the name of the user to modify:");
        io::stdin().read_line(&mut name).unwrap();
        let name = name.trim().to_lowercase();

        if let Some(user) = self.iter_mut().find(|user| user.name == name) {
            let mut new_name = String::new();
            let mut new_email = String::new();
            let mut new_age_str = String::new();

            println!("Enter new name (leave blank to keep current):");
            io::stdin().read_line(&mut new_name).unwrap();
            let new_name = new_name.trim().to_lowercase();
            if !new_name.is_empty() {
                user.name = new_name.to_string().to_lowercase();
            }

            println!("Enter new email (leave blank to keep current):");
            io::stdin().read_line(&mut new_email).unwrap();
            let new_email = new_email.trim();
            if !new_email.is_empty() {
                user.email = new_email.to_string();
            }

            println!("Enter new age (leave blank to keep current):");
            io::stdin().read_line(&mut new_age_str).unwrap();
            if let Ok(new_age) = new_age_str.trim().parse::<u8>() {
                user.age = new_age;
            }

            println!("User modified successfully!");
        } else {
            println!("User not found.");
        }
    }

    fn display_users(&self) {
        if self.is_empty() {
            println!("No users to display.");
        } else {
            for user in self {
                user.display_info();
            }
        }
    }
}

fn main() {
    let mut users: Vec<UserData> = Vec::new();
    loop {
        println!("\nMenu:");
        println!("1. Add User");
        println!("2. Remove User");
        println!("3. Modify User");
        println!("4. Display Users");
        println!("5. Exit");

        let mut choice = String::new();
        print!("Enter your choice: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut choice).unwrap();
        let choice: u32 = choice.trim().parse().expect("Please enter a valid number");

        match choice {
            1 => users.add_user(),
            2 => users.remove_user(),
            3 => users.modify_user(),
            4 => users.display_users(),
            5 => {
                println!("Exiting program.");
                break;
            },
            _ => println!("Invalid choice, please try again."),
        }
    }
}