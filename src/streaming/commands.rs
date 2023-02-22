use anyhow::{bail, ensure, Context, Result};
use regex::Regex;

use crate::{entities::User, exceptions::Exception::*, init::SERVICE};

pub(crate) async fn command(text: &str, user: &User, reply_id: String) -> Result<()> {
    let context = text
        .split_whitespace()
        .filter(|s| {
            Regex::new(r"^@(?:[a-zA-Z]|\d|_)+(?:@([a-zA-Z]|\d|-)+\.[a-z]+)?$")
                .is_ok_and(|r| !r.is_match(s))
        })
        .collect::<Vec<_>>();

    let command = *context.first().context(CommandNotFound.msg())?;

    match command {
        "show" => SERVICE.show_uni().await?,
        "list" => SERVICE.list_uni(reply_id).await?,
        "add" => {
            // We cannnot grab the full information of an user from note.
            let user = User::from(&user.id).await?;

            ensure!(!user.is_cat, CatAccount.msg());
            ensure!(user.is_tums_mod().await, NoPermission.msg());

            let pos: i32 = context
                .get(1)
                .context(PositionCommandNotFound.msg())?
                .parse()?;
            let content = context
                .get(2..)
                .context(NoUniFoundOnTheCommand.msg())?
                .join(" ");

            SERVICE.add_uni(content, pos, reply_id).await?;
        }
        "remove" => {
            let user = User::from(&user.id).await?;

            ensure!(user.is_tums_mod().await, NoPermission.msg());

            let pos: i32 = context
                .get(1)
                .context(PositionCommandNotFound.msg())?
                .parse()?;

            SERVICE.remove_uni(pos, reply_id).await?;
        }
        _ => bail!(NoSuchCommand.msg()),
    };

    Ok(())
}
