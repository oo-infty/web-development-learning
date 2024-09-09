use std::sync::Arc;

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use snafu::ResultExt;

use crate::domain::application::Core;
use crate::domain::entity::answer::{
    CompletionAnswer, MultipleSelectionAnswer, SingleSelectionAnswer, StandardSource,
};
use crate::domain::entity::question::{
    CompletionQuestion, MultipleSelectionQuestion, SingleSelectionQuestion,
};
use crate::domain::entity::test::Test;
use crate::domain::entity::user::User;
use crate::inbound::error::{ApiError, DataInvalidSnafu, UnknownSnafu};

#[derive(Debug, Deserialize)]
pub struct StartRequest {
    login_id: usize,
}

#[derive(Debug, Serialize)]
pub struct StartResponse {
    test: Test,
}

pub async fn handle_start() -> Result<Json<StartResponse>, ApiError> {
    Ok(StartResponse {
        test: builtin_test(),
    }
    .into())
}

pub fn builtin_test() -> Test {
    Test::new(
        0.into(),
        vec![
            SingleSelectionQuestion::try_new(
                0.into(),
                "Which command is used to trace the system calls made by a process, and which options would you use to trace a specific process ID (PID) and output the results to a file?".into(),
                vec![
                    "<code>strace -p PID -o output.txt</code>".into(),
                    "<code>strace -c -p PID > output.txt</code>".into(),
                    "<code>strace -f -p PID | tee output.txt</code>".into(),
                    "<code>strace -t -p PID > output.txt</code>".into(),
                ],
                SingleSelectionAnswer::<StandardSource>::try_new(0).unwrap(),
            )
            .unwrap()
            .into(),
            MultipleSelectionQuestion::try_new(
                2.into(),
                "In Linux, how can you check the IP address of network interfaces?".into(),
                vec![
                    "<code>ifconfig</code>".into(),
                    "<code>ip addr show</code>".into(),
                    "<code>netstat</code>".into(),
                    "<code>ping</code>".into(),
                ],
                MultipleSelectionAnswer::<StandardSource>::try_new(vec![2, 3]).unwrap(),
            )
            .unwrap()
            .into(),
            CompletionQuestion::try_new(
                3.into(),
                "In Linux, which commands can be used to find files or directories?".into(),
                CompletionAnswer::<StandardSource>::try_new("answer").unwrap(),
            )
            .unwrap()
            .into(),
        ],
    )
}
