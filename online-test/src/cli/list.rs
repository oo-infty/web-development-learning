use std::sync::Arc;

use comfy_table::Table;
use snafu::{prelude::*, Whatever};

use crate::domain::repository::question::QuestionRepository;

pub async fn run_list(repo: Arc<dyn QuestionRepository>) -> Result<(), Whatever> {
    let mut table = Table::new();
    table.set_header(vec!["ID", "Kind", "Content"]);

    repo.list_questions(20)
        .await
        .whatever_context("Could not list questions")?
        .into_iter()
        .for_each(|q| {
            table.add_row(q);
        });

    println!("{table}");
    Ok(())
}
