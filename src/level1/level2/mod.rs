pub fn print_A_to_z(){
    for c in 'A'..='z'{
        if c.is_alphabetic(){
            println!("{}",c);
        }
    }
}