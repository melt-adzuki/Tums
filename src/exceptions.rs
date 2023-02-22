use rand::seq::SliceRandom;

pub(crate) enum Exception {
    DrivenByCatAccount,
    NotDrivenByBotAccount,
    CatAccount,
    NoPermission,
    CommandNotFound,
    PositionCommandNotFound,
    NoUniFoundOnThisPosition,
    NoUniFoundOnTheCommand,
    NoSuchCommand,
}

const CATS: [&str; 5] = [
    "ฅ(^•ω•^ฅ",
    "(=^・・^=)",
    "(ฅ >ω< ฅ)",
    "(=^･ω･^=)",
    "(=^･ｪ･^=)",
];

impl Exception {
    pub(crate) fn msg<'a>(&self) -> &'a str {
        use Exception::*;

        match self {
            DrivenByCatAccount => "This is a cat account!",
            NotDrivenByBotAccount => "This is not a bot account!",
            CatAccount => CATS.choose(&mut rand::thread_rng()).unwrap_or(&""),
            NoPermission => "You do not have permission to use this command.",
            CommandNotFound => "No command found on the context.",
            PositionCommandNotFound => "No position command found.",
            NoUniFoundOnThisPosition => "No Uni found on this position.",
            NoUniFoundOnTheCommand => "No Uni found on the command.",
            NoSuchCommand => "No such command.",
        }
    }
}
