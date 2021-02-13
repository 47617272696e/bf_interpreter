struct Mem {
    neg: Vec<i128>,
    pos: Vec<i128>,
    ptr: i128,
}

impl Mem {
    fn new() -> Mem {
        Mem {
            neg: Vec::new(),
            pos: vec![0],
            ptr: 0,
        }
    }

    fn ptr_plus(&mut self, delta: i128) {
        self.ptr += delta;
        if self.ptr < 0 {
            if -self.ptr > self.neg.len() as i128 {
                self.neg.push(0);
            }
        } else if self.ptr + 1 > self.pos.len() as i128 {
            self.pos.push(0);
        }
    }

    fn val(&mut self) -> &mut i128 {
        if self.ptr < 0 {
            &mut self.neg[-self.ptr as usize - 1]
        } else {
            &mut self.pos[self.ptr as usize]
        }
    }
}

pub fn main() {
    loop {
        cls();
        let code = input("Enter code: ");
        parse(code, &mut Mem::new());
        let _ = input("Press enter to continue...");
    }
}

fn parse(code: String, mem: &mut Mem) {
    let mut current: usize = 0;
    let mut skip = false;
    let mut last_bracket = Vec::new();
    loop {
        let ch = code.chars().nth(current);
        let ch = if ch == None {
            break;
        } else {
            ch.unwrap()
        };
        if ch == '[' {
            last_bracket.push(current);
            if *mem.val() == 0 {
                skip = true;
            }
        } else if ch == ']' {
            if skip {
                skip = false;
                last_bracket.pop();
            } else {
                current = last_bracket[last_bracket.len() - 1];
                continue;
            }
        } else if !skip {
            match ch {
                '>' => mem.ptr_plus(1),
                '<' => mem.ptr_plus(-1),
                '+' => *mem.val() += 1,
                '-' => *mem.val() -= 1,
                '.' => println!(" {}", mem.val()),
                ',' => *mem.val() = user_in(),
                _ => {}
            }
        }
        current += 1;
    }
}

fn user_in() -> i128 {
    let input = input("   >");
    let ret = input.parse::<i128>();
    ret.expect("Couldn't parse input")
}

fn cls() {
    print!("\x1B[2J\x1B[1;1H");
}

fn input(msg: &str) -> String {
    use std::io::{stdin, stdout, Write};

    let mut s = String::new();
    print!("{}", msg);
    let _ = stdout().flush();
    stdin().read_line(&mut s).expect("How?");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
    s
}
