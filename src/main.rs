fn main() {
    print!("{}", process(String::from("hello MATMEX")));
}

fn process(_text: String) -> String {
    let first_msg: String = _text.split_whitespace().next().unwrap_or("").to_string();

    let mut message = _text;
    message.push_str(" rust");

    let first_mut: &mut String = &mut message;
    first_mut.push_str("");
    let sec_mut: &String = &message;

    format!("{first_msg}\n{message}\n{sec_mut}\n",)
}

#[cfg(test)]
#[path = "../tests/unit/process.rs"]
mod process_tests;
