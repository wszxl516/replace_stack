# example replace argv[0] in stack

```c
    int main(int argc, char *argv[]){
        argv[0] = "[new_name]";
        return 0;
    }
```

```rust
    use stack_replace::ReplaceStack;
    fn main() {
        let args = std::env::args();
        let args_str = args.collect::<Vec<String>>();
        let st = ReplaceStack::new().unwrap();
        let argv_addr = st.find_string_addr(&args_str[0]).unwrap();
        println!("argv: {:?}", argv_addr);
        for addr in argv_addr {
            ReplaceStack::replace_string(addr, "[new_name]");
        }
    }
```

