use std::io;

fn main() {

    println!("=======> GORUSTER-LITE <=======");
    println!("Enter target URL ( https:// must be included. ): ");

    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read URL...");
    let input = input.trim();

    let wordlist = [
    "admin", "administrator", "login", "signin", "secret", "secrets", 
    "dashboard", "panel", "wp-admin", "cpanel", "config", "configuration", 
    "setup", "install", "db", "database", "backup", "backups", "dump", 
    "sql", "api", "api/v1", "api/v2", "dev", "developer", "test", 
    "testing", "staging", "prod", "production", "uploads", "images", 
    "assets", "files", "download", "downloads", "hidden", "private", 
    "secure", "auth", "user", "users", "server-status", "status", 
    "info", "phpinfo", "logs", "log", "git", "github"
    ];


    let client = reqwest::blocking::Client::new();

    let mut success = 0;
    let mut failed = 0;

    let mut success_words: Vec<&str> = Vec::new();


    for &word in &wordlist {

        let full = format!("{}/{}", input, word);

        if let Ok(response) = client.get(&full).send() {

            if response.status().is_success() {
            
                println!("{} 203 SUCCESS!", word);
                success += 1;
                success_words.push(word);

            }

            else {

                println!("{} 404 NOT FOUND", word);
                failed += 1;
            }

        }

    }

    println!("203 Success: {}", success);
    println!("404 Not Found: {}", failed);

    println!("");

    let mut question: String = String::new();
    println!("Do you want to see the successful words? (y/n): ");
    io::stdin().read_line(&mut question).expect("Failed to read input...");

    let question = question.trim();

    if question == "Y" || question == "y" {

        println!("Successful words: {:?}", success_words);

    }
}