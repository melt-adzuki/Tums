use anyhow::{Context, Result};
use rand::seq::SliceRandom;

const CATS: [&str; 5] = [
    "ฅ(^•ω•^ฅ",
    "(=^・・^=)",
    "(ฅ >ω< ฅ)",
    "(=^･ω･^=)",
    "(=^･ｪ･^=)",
];

pub(crate) fn generate_cat<'a>() -> Result<&'a str> {
    let cat = CATS
        .choose(&mut rand::thread_rng())
        .context("No serifs found.")?;

    Ok(cat)
}
