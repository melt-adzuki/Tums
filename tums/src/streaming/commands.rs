use anyhow::{ensure, Context, Result};
use regex::Regex;

use crate::{
    cat::generate_cat,
    entities::{NotedUser, User},
    init::SERVICE,
};

pub(crate) async fn command(text: &str, user: &NotedUser, reply_id: String) -> Result<()> {
    let context = text
        .split_whitespace()
        .filter(|s| {
            Regex::new(r"^@(?:[a-zA-Z]|\d|_)+(?:@([a-zA-Z]|\d|-)+\.[a-z]+)?$")
                .is_ok_and(|r| !r.is_match(s))
        })
        .collect::<Vec<_>>();

    let command = *context
        .first()
        .context("No command found on the context.")?;

    match command {
        "show" => SERVICE.show_uni().await?,
        "list" => SERVICE.list_uni(reply_id).await?,
        "add" => {
            let user = User::from(&user.id).await?;

            ensure!(!user.is_cat, generate_cat()?);
            ensure!(
                user.is_tums_mod().await,
                "You do not have permission to use this command."
            );

            let pos: i32 = context
                .get(1)
                .context("No position command found.")?
                .parse()?;
            let content = context
                .get(2..)
                .context("No Uni found on the command.")?
                .join(" ");

            SERVICE.add_uni(content, pos, reply_id).await?;
        }
        "remove" => {
            let user = User::from(&user.id).await?;

            ensure!(
                user.is_tums_mod().await,
                "You do not have permission to use this command."
            );

            let pos: i32 = context
                .get(1)
                .context("No position command found.")?
                .parse()?;

            SERVICE.remove_uni(pos, reply_id).await?;
        }
        _ => {}
    };

    Ok(())
}
