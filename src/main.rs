use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;
use std::io::{Bytes, Read, Stdin, Write, stdin, stdout};
use std::path::Path;

fn main() {
    let args = get_args();

    if args.len() != 1 {
        eprintln!("You must pass exactly one filepath as an argument.");
        return;
    }

    if !Path::new(&args[0]).is_file() {
        eprintln!("File {} does not exist.", args[0]);
        return;
    }

    let prog = match read_to_string(&args[0]) {
        Ok(x) => { x }
        _     => {
            eprintln!("Error occurred reading from file {}.", args[0]);
            eprintln!("This user might not have permission to read the file.");
            eprintln!("The file might not be encoded as valid utf8.");
            return;
        }
    };

    let mut count: i128 = 0;

    for c in prog.chars() {
        match c {
            '[' => { count += 1; }
            ']' => {
                count -= 1;
                if count < 0 {
                    eprintln!("Mismatched brackets in source code.");
                    return;
                }
            }
            _ => {}
        }
    }

    if count != 0 {
        eprintln!("Mismatched brackets in source code.");
        return;
    }

    Program::from_source(&prog).exec();
}

fn get_args() -> Vec<String> {
    let mut args = env::args();
    args.next(); // drop executable path
    args.collect()
}

struct Program {
    dp: isize,
    ip: usize,
    len: usize,
    dtape: HashMap<isize, u8>,
    itape: Vec<Token>,
    input: Bytes<Stdin>,
}

impl Program {
    fn from_source(source: &str) -> Self {
        use Token::*;

        let     dp: isize;
        let     ip: usize;
        let     len: usize;
        let mut dtape: HashMap<isize, u8>;
        let mut itape: Vec<Token>;
        let     input: Bytes<Stdin>;

        dp = 0;
        ip = 0;

        dtape = HashMap::new();
        dtape.insert(0, 0);

        itape = Vec::new();
        for c in source.chars() {
            match c {
                '>' => { itape.push(INCP); }
                '<' => { itape.push(DECP); }
                '+' => { itape.push(INCC); }
                '-' => { itape.push(DECC); }
                '.' => { itape.push(PCHR); }
                ',' => { itape.push(GCHR); }
                '[' => { itape.push(LBRK); }
                ']' => { itape.push(RBRK); }
                 _  => {                   }
            }
        }

        #[cfg(debug_assertions)]
        eprintln!(
            "itape:\n{:#?}",
            &itape,
        );

        len = itape.len();

        input = stdin().bytes();

        #[cfg(debug_assertions)]
        eprintln!(
            "input:\n{:#?}",
            &input,
        );

        Program {
            dp,
            ip,
            len,
            dtape,
            itape,
            input,
        }
    }

    fn exec(&mut self) {
        use Token::*;

        while !self.is_halted() {
            match self.itape[self.ip] {
                INCP => { self.incp(); }
                DECP => { self.decp(); }
                INCC => { self.incc(); }
                DECC => { self.decc(); }
                PCHR => { self.pchr(); }
                GCHR => { self.gchr(); }
                LBRK => { self.lbrk(); }
                RBRK => { self.rbrk(); }
            }
        }
    }

    fn is_halted(&self) -> bool {
        self.ip == self.len
    }

    fn incp(&mut self) {
        #[cfg(debug_assertions)]
        eprintln!(
            "Called INCP | IP: {:5} | DP: {:5} ({:3})",
            self.ip,
            self.dp,
            self.dtape[&self.dp],
        );

        self.dp += 1;

        if !self.dtape.contains_key(&self.dp) {
            self.dtape.insert(self.dp, 0);
        }

        self.ip += 1;
    }

    fn decp(&mut self) {
        #[cfg(debug_assertions)]
        eprintln!(
            "Called DECP | IP: {:5} | DP: {:5} ({:3})",
            self.ip,
            self.dp,
            self.dtape[&self.dp],
        );

        self.dp -= 1;

        if !self.dtape.contains_key(&self.dp) {
            self.dtape.insert(self.dp, 0);
        }

        self.ip += 1;
    }

    fn incc(&mut self) {
        #[cfg(debug_assertions)]
        eprintln!(
            "Called INCC | IP: {:5} | DP: {:5} ({:3})",
            self.ip,
            self.dp,
            self.dtape[&self.dp],
        );

        self.dtape.insert(
            self.dp,
            self.dtape[&self.dp].wrapping_add(1),
        );

        self.ip += 1;
    }

    fn decc(&mut self) {
        #[cfg(debug_assertions)]
        eprintln!(
            "Called DECC | IP: {:5} | DP: {:5} ({:3})",
            self.ip,
            self.dp,
            self.dtape[&self.dp],
        );

        self.dtape.insert(
            self.dp,
            self.dtape[&self.dp].wrapping_sub(1),
        );

        self.ip += 1;
    }

    fn pchr(&mut self) {
        #[cfg(debug_assertions)]
        eprintln!(
            "Called PCHR | IP: {:5} | DP: {:5} ({:3})",
            self.ip,
            self.dp,
            self.dtape[&self.dp],
        );

        print!(
            "{}",
            self.dtape[&self.dp] as char
        );
        stdout().flush().unwrap();

        #[cfg(debug_assertions)]
        eprintln!();

        self.ip += 1;
    }

    fn gchr(&mut self) {
        #[cfg(debug_assertions)]
        eprintln!(
            "Called GCHR | IP: {:5} | DP: {:5} ({:3})",
            self.ip,
            self.dp,
            self.dtape[&self.dp],
        );

        let chr = self.input.next().unwrap_or(Ok(0)).unwrap();

        #[cfg(debug_assertions)]
        eprintln!("Called GHCR | Got: {:3} ({})", chr, chr as char);

        self.dtape.insert(self.dp, chr);

        self.ip += 1;
    }

    fn lbrk(&mut self) {
        use Token::*;

        #[cfg(debug_assertions)]
        eprintln!(
            "Called LBRK | IP: {:5} | DP: {:5} ({:3})",
            self.ip,
            self.dp,
            self.dtape[&self.dp],
        );

        self.ip += 1;

        if self.dtape[&self.dp] != 0 {
            return;
        }

        let mut count = 1;

        while count != 0 {
            match self.itape[self.ip] {
                LBRK => { count += 1; }
                RBRK => { count -= 1; }
                _    => {             }
            }

            self.ip += 1;
        }
    }

    fn rbrk(&mut self) {
        use Token::*;

        #[cfg(debug_assertions)]
        eprintln!(
            "Called RBRK | IP: {:5} | DP: {:5} ({:3})",
            self.ip,
            self.dp,
            self.dtape[&self.dp],
        );

        if self.dtape[&self.dp] == 0 {
            self.ip += 1;
            return;
        }

        self.ip -= 1;

        let mut count = 1;

        while count != 0 {
            match self.itape[self.ip] {
                RBRK => { count += 1; }
                LBRK => { count -= 1; }
                _    => {             }
            }

            self.ip -= 1;
        }

        self.ip += 2;
    }
}

#[derive(Debug)]
enum Token {
    INCP, // >
    DECP, // <
    INCC, // +
    DECC, // -
    PCHR, // .
    GCHR, // ,
    LBRK, // [
    RBRK, // ]
}
