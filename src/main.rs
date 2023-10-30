use std::fs::create_dir;

fn main() {
    let path = dirs::home_dir().unwrap().join(".dawn");
    /*
    // Debug
    println!("{:}", path.display());
    */

    if path.exists() {
        println!("...Path checked");
    } else {
        create_dir(&path).expect("Fail");
        println!("...Path created");
        // TODO: Create sqlite .db file
    }
}
