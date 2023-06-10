use std::fs;

fn main() {
    // read /etc/hosts file and print it
    let hosts = fs::read_to_string("/etc/hosts");
    let mut hosts_data: String;

    match hosts {
        Ok(content) => {
            println!("{}", content);
            hosts_data = content;
        },
        Err(e) => panic!("Error: {}", e),
    }

    // write the content to a file
    let write_result = fs::write("/tmp/hosts", hosts_data);

    match write_result {
        Ok(_) => println!("File written successfully"),
        Err(e) => panic!("Error: {}", e),
    }
}
