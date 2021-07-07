use colored::Colorize;

#[macro_export]
macro_rules! cdbg  {
    () => {
        eprintln!("{}", format!("[{}:{}]", file!(), line!()).green());
    };
    ($val:expr $(,)?) => {
        match $val {
            tmp => {
                eprintln!("{} {} = {}",
                    format!("[{}:{}]", file!(), line!()).green(),
                    stringify!($val).red(),
                    format!("{:#?}", &tmp).blue(),
                );
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($(colored_dbg!($val)),+,)
    };
}