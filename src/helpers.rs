use std::{
    fs,
    io::{self, BufRead, BufReader},
};

pub fn read_file_lines_to_vec(filename: &str) -> io::Result<Vec<String>> {
    let file_in = fs::File::open(filename)?;
    let file_reader = BufReader::new(file_in);
    Ok(file_reader.lines().filter_map(io::Result::ok).collect())
}

pub fn check_address_block(address_to_check: &str) -> bool {
    let address_blocked = read_file_lines_to_vec(&"./blacklist.txt".to_string());
    let address_blocked_iter: Vec<String> = match address_blocked {
        Ok(vector) => vector,
        Err(_) => vec!["Error".to_string()],
    };
    println!("{:?}", address_blocked_iter);
    println!("{:?}", address_to_check);
    let address_in = address_blocked_iter.contains(&address_to_check.to_string());

    return address_in;
}
