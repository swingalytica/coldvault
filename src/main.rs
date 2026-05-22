fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.iter().any(|arg: &String| arg == "--help" || arg == "-h") {
        println!("Usage: coldvault [options]");
        println!("Options:");
        println!("  --help, -h    Show this help message");
        // Add more options as needed
    } else {
        println!("Running the program...");
        // Your program logic here
    }
}
