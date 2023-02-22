#[macro_export]
macro_rules! log {
    ($l:literal -> $e:expr) => {{
        use colored::Colorize;
        println!("{} {}", format!("[{:^4}]", $l).dimmed(), $e)
    }};

    ($l:literal | $($e:expr),*) => {{
        use colored::Colorize;
        println!("{} {}", format!("[{:^4}]", $l).dimmed(), format!($($e),*))
    }};
}
