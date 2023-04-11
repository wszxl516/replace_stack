use std::convert::TryInto;
use std::ffi::CString;
use std::fs::File;
use std::io;
use std::io::BufRead;

pub struct ReplaceStack {
    stack: (usize, usize),
}

#[derive(Debug)]
pub struct StrBlock(usize, usize);
impl StrBlock{
    fn new(start: usize, len: usize)-> Self{
        Self{0: start, 1: len}
    }
}


impl ReplaceStack {
    pub fn new() -> Result<Self, ()> {
        match Self::find_stack() {
            Ok(stack) => Ok(Self { stack }),
            Err(_) => Err(()),
        }
    }
    fn find_stack() -> Result<(usize, usize), ()> {
        let map_file = File::open("/proc/self/maps").unwrap();
        let stack = io::BufReader::new(map_file)
            .lines()
            .map(|line| line.unwrap_or("".to_string()))
            .filter(|line| line.contains("[stack]"))
            .collect::<Vec<String>>();
        if stack.is_empty() {
            return Err(());
        }
        let stack_range: [&str; 2] = stack[0].split(" ").collect::<Vec<&str>>()[0]
            .splitn(2, "-")
            .collect::<Vec<&str>>()
            .try_into()
            .unwrap();
        let start = usize::from_str_radix(stack_range[0], 16).unwrap_or(0);
        let end = usize::from_str_radix(stack_range[1], 16).unwrap_or(0);
        if start != 0 && end != 0 {
            return Ok((start, end));
        }
        Err(())
    }
    pub fn find_string_addr(&self, name: &String) -> Result<Vec<StrBlock>, ()> {
        let mut tmp = Vec::<u8>::new();
        let mut argv_addr = Vec::<StrBlock>::new();
        let (start, end) = self.stack;
        for i in start..end {
            tmp.clear();
            for x in 0..name.len() {
                if i + x >= end {
                    break;
                }
                tmp.push(unsafe { std::ptr::read((i + x) as *const u8) })
            }
            let p_name = unsafe { CString::from_vec_unchecked(tmp.clone()) }
                .to_str()
                .unwrap_or("")
                .to_string();
            if p_name.contains(name) {
                let mut len = 0usize;
                loop {
                    if (unsafe { std::ptr::read((i + len) as *const u8) }).eq(&0){
                        break
                    }
                    len += 1;
                }
                argv_addr.push(StrBlock::new(i, len))
            }
        }
        if argv_addr.is_empty() {
            return Err(());
        }
        Ok(argv_addr)
    }
    pub fn replace_string(block: StrBlock, name: &str) {
        let name_len = name.len();
        let str_ptr: &mut [u8] =
            unsafe { std::slice::from_raw_parts_mut(block.0 as *mut u8, block.1) };
        str_ptr.fill(0);
        for c in 0..name_len {
            str_ptr[c] = name.as_bytes()[c];
        }
    }
}
