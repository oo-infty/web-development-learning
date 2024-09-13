use std::io;
use std::sync::Arc;

use snafu::{prelude::*, Whatever};

use crate::domain::repository::question::QuestionRepository;

pub async fn run_remove(repo: Arc<dyn QuestionRepository>) -> Result<(), Whatever> {
    let id: usize = loop {
        println!("What's the question's ID");
        let mut id = String::with_capacity(8);
        io::stdin()
            .read_line(&mut id)
            .whatever_context("Could not read input")?;
        match id.trim().parse() {
            Ok(id) => break id,
            _ => println!("Invalid input"),
        }
        println!("");
    };

    repo.remove_question(id.into())
        .await
        .whatever_context("Could not remove question")?;

    Ok(())
}
