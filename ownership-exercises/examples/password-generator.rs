use std::ops::Range;
use rand::Rng;
use rand::rngs::ThreadRng;
use std::env;

fn get_passwd_len() -> Option<u32> {
    let mut args = env::args();
    let _ = args.next();
    let passwd_str_opt = args.next();

    if let Some(passwd_str) = passwd_str_opt {
        passwd_str.parse::<u32>().ok()
    } else {
        None
    }
}

fn get_passwd(len: u32, char_range: &Range<u8>, rng: &mut ThreadRng) -> String {
    (0..len).map(|_| rng.gen_range(char_range.clone()))
        .map(|num| num as char)
        .collect::<String>()
}

fn main() {
    let passwd_len = get_passwd_len().unwrap();

    let first_char = '!' as u8;
    let last_char = '~' as u8;

    let char_range = first_char..(last_char+1);

    let mut rng = rand::thread_rng();

    let passwd = get_passwd(passwd_len, &char_range, &mut rng);

    println!("{}", passwd);
}
