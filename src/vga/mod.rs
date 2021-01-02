#[macro_use]
pub mod macros;

pub fn unwrap_with_msg<R, E: core::fmt::Debug>(msg: &str, val: Result<R, E>) -> R {
    print!("{}...", msg);
    match val {
        Ok(o) => {
            println!("[OK]");
            o
        }
        Err(e) => {
            println!("[KO]");
            panic!("{:?}", e);
        }
    }
}

pub fn write_result_bool<T: Fn() -> bool>(msg: &str, func: T) -> bool {
    print!("{}...", msg);
    let rst = func();
    match rst {
        true => println!("[OK]"),
        false => println!("[KO]"),
    }
    rst
}
