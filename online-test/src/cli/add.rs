use std::io;
use std::sync::Arc;

use snafu::{prelude::*, Whatever};

use crate::domain::repository::question::QuestionRepository;

pub async fn run_add(repo: Arc<dyn QuestionRepository>) -> Result<(), Whatever> {
    let mut kind = String::new();

    println!("What kind of question would you like to add?");
    println!("([s]ingle-selection, [m]ultiple-selection, [c]ompletion)");

    loop {
        io::stdin()
            .read_line(&mut kind)
            .whatever_context("Could not read input")?;

        match kind.trim().chars().next() {
            Some('s') | Some('S') => return run_add_single_selection(repo).await,
            Some('m') | Some('M') => return run_add_multiple_selection(repo).await,
            Some('c') | Some('C') => return run_add_completion(repo).await,
            _ => println!("Invalid input: Expected 's', 'm' or 'c'"),
        }

        kind.clear();
        println!();
    }
}

async fn run_add_single_selection(repo: Arc<dyn QuestionRepository>) -> Result<(), Whatever> {
    println!("");
    let content = read_content()?;
    let options = (1..=4)
        .map(|i| {
            println!("");
            read_option(i)
        })
        .collect::<Result<Vec<_>, _>>()?;

    let answer = loop {
        println!("");
        println!("What's your question's answer");
        println!("(Input one integer within [1, 4])");
        let mut answer = String::with_capacity(8);
        io::stdin()
            .read_line(&mut answer)
            .whatever_context("Could not read input")?;
        match answer.trim().parse() {
            Ok(answer) if (1..=4).contains(&answer) => break answer,
            _ => println!("Invalid input"),
        }
    };

    repo.insert_single_selection(content, options, answer)
        .await
        .whatever_context("Could not add question")?;

    Ok(())
}

async fn run_add_multiple_selection(repo: Arc<dyn QuestionRepository>) -> Result<(), Whatever> {
    println!("");
    let content = read_content()?;
    let options = (1..=4)
        .map(|i| {
            println!("");
            read_option(i)
        })
        .collect::<Result<Vec<_>, _>>()?;

    let answer = loop {
        println!("");
        println!("What's your question's answer");
        println!("(Input integers within [1, 4] separated by space)");
        let mut answer = String::with_capacity(8);
        io::stdin()
            .read_line(&mut answer)
            .whatever_context("Could not read input")?;
        let answer = answer
            .trim()
            .split_whitespace()
            .map(|x| x.parse())
            .collect::<Result<Vec<_>, _>>();
        match answer {
            Ok(answer) if answer.iter().all(|x| (1..=4).contains(x)) => break answer,
            _ => println!("Invalid input"),
        }
    };

    if !(2..=3).contains(&answer.len()) {
        println!("Warning: Expect the number of correct options to be 2 or 3");
    }

    repo.insert_multiple_selection(content, options, answer)
        .await
        .whatever_context("Could not add question")?;

    Ok(())
}

async fn run_add_completion(repo: Arc<dyn QuestionRepository>) -> Result<(), Whatever> {
    println!("");
    let content = read_content()?;

    println!("");
    println!("What's your question's answer");
    let mut answer = String::with_capacity(64);
    io::stdin()
        .read_line(&mut answer)
        .whatever_context("Could not read input")?;

    repo.insert_completion(content, answer.trim().to_owned())
        .await
        .whatever_context("Could not add question")?;

    Ok(())
}

fn read_content() -> Result<String, Whatever> {
    println!("What's your question's content?");
    let mut content = String::with_capacity(256);
    io::stdin()
        .read_line(&mut content)
        .whatever_context("Could not read input")?;
    Ok(content.trim().to_owned())
}

fn read_option(index: usize) -> Result<String, Whatever> {
    println!("What's your question's option #{index}");
    let mut option = String::with_capacity(64);
    io::stdin()
        .read_line(&mut option)
        .whatever_context("Could not read input")?;
    Ok(option.trim().to_owned())
}
