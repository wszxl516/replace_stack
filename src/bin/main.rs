use stack_replace::ReplaceStack;
use std::time::Duration;

fn main() {
    let args = std::env::args();
    let args_str = args.collect::<Vec<String>>();
    let st = ReplaceStack::new().unwrap();
    let argv_addr = st.find_string_addr(&args_str[0]).unwrap();
    println!("argv: {:?}", argv_addr);
    for addr in argv_addr {
        ReplaceStack::replace_string(addr, "[test_name]");
    }
    loop {
        let args = std::env::args();
        let args_str = args.collect::<Vec<String>>();
        println!("{}", args_str[0]);
        std::thread::sleep(Duration::from_secs(1))
    }
}
