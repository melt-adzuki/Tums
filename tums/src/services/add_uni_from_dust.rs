use std::sync::Arc;

use anyhow::{ensure, Result};
use futures::{lock::Mutex, stream, StreamExt};
use log::info;
use similar::{ChangeTag, TextDiff};

use crate::{
    domain::{interactor::Interactor, uni::UniRepository},
    validation::IsUni,
};

use super::service::Service;

impl<T, U> Service<T, U>
where
    T: UniRepository,
    U: Interactor,
{
    /// タイムラインから自動的に新しい思慮深いウニを検出し、データベースに追加します。
    /// その際、該当の投稿に対して追加された文字列を返信します。
    pub(crate) async fn add_uni_from_dust(&self, dust: String, reply_id: String) -> Result<()> {
        let new = dust.split('\n').map(|s| s.trim()).collect::<Vec<_>>();
        if !new.is_uni() {
            info!("The content cannot be a part of Uni.");
            return Ok(());
        }

        let new = {
            let len = new.len();
            &new[2..len - 1].to_vec()
        };

        let current = self.uni_repo.list().await?;
        let current = current
            .iter()
            .map(|u| u.content.as_str())
            .collect::<Vec<_>>();

        let diff = TextDiff::from_slices(current.as_slice(), new.as_slice());

        let lost_line_counter = Arc::new(Mutex::new(0));

        let new_lines = stream::iter(diff.iter_all_changes())
            .filter(|c| {
                let c = *c;
                let counter = Arc::clone(&lost_line_counter);
                async move {
                    if c.tag() == ChangeTag::Delete {
                        *counter.lock().await += 1;
                    }
                    c.tag() == ChangeTag::Insert && c.to_string_lossy().trim().len() >= 4
                }
            })
            .filter_map(|c| {
                let c = c;
                let counter = Arc::clone(&lost_line_counter);
                async move {
                    let count = *counter.lock().await;
                    c.new_index()
                        .map(|index| (c.to_string_lossy().to_string(), index + 1 + count))
                }
            })
            .collect::<Vec<_>>()
            .await;

        for line in new_lines.iter() {
            let content = line.0.to_string();
            let pos = line.1 as i32;

            // let counter = Arc::clone(&lost_line_counter);
            // let count = counter.try_lock().unwrap().to_owned();
            // println!("{} + {} = {} --- {}", pos - (count as i32), count, pos, content);
            self.uni_repo.add(content, pos).await?;
        }

        let lines_added = new_lines
            .iter()
            .map(|line| line.0.to_string())
            .collect::<Vec<_>>()
            .join("\n");

        let content = format!("以下を思慮深いウニに追加しました:\n\n{}", lines_added);
        self.interactor.reply(content, reply_id).await?;

        Ok(())
    }
}
