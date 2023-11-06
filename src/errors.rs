// This function is used to log the erros on the terminal and exit the program
pub fn create_end(message: &str) { 
    println!("[cobalt] uh-oh! {message}"); // log error message on the terminal
    std::process::exit(0); // KILL
}