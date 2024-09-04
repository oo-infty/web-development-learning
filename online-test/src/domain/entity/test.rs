use std::collections::HashMap;

use crate::domain::entity::answer::{Answer, SubmissionSource};
use crate::domain::entity::id::Id;
use crate::domain::entity::question::{AbstractQuestion, Question};
use crate::domain::entity::score::Score;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Test {
    id: Id,
    questions: Vec<Question>,
}

impl Test {
    pub fn new(id: Id, questions: Vec<Question>) -> Self {
        Self { id, questions }
    }

    pub fn id(&self) -> Id {
        self.id
    }

    pub fn grade(&self, submission: &Submission) -> Score {
        if self.questions.is_empty() {
            return Score::try_new(100f32)
                .unwrap_or_else(|_| unreachable!("`100f32` should be converted to `Score`"));
        }

        let correct = self
            .questions
            .iter()
            .filter_map(|q| submission.answers.get(&q.id()).map(|a| (q, a)))
            .filter(|(q, a)| q.check(a))
            .count();

        Score::try_new(100f32 * correct as f32 / self.questions.len() as f32)
            .unwrap_or_else(|_| unreachable!("`f32` variable should be converted to `Score`"))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Submission {
    answers: HashMap<Id, Answer<SubmissionSource>>,
}

impl Submission {
    pub fn new(answers: HashMap<Id, Answer<SubmissionSource>>) -> Self {
        Self { answers }
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::entity::{
        answer::{
            CompletionAnswer, MultipleSelectionAnswer, SingleSelectionAnswer, StandardSource,
        },
        question::{CompletionQuestion, MultipleSelectionQuestion, SingleSelectionQuestion},
    };

    use super::*;

    #[test]
    fn test_grade() {
        let test = Test::new(
            0.into(),
            vec![
                SingleSelectionQuestion::try_new(
                    0.into(),
                    "0. Single-selection".into(),
                    vec![
                        "option a".into(),
                        "option b".into(),
                        "option c".into(),
                        "option d".into(),
                    ],
                    SingleSelectionAnswer::<StandardSource>::try_new(0).unwrap(),
                )
                .unwrap()
                .into(),
                SingleSelectionQuestion::try_new(
                    1.into(),
                    "1. Single-selection".into(),
                    vec![
                        "option a".into(),
                        "option b".into(),
                        "option c".into(),
                        "option d".into(),
                    ],
                    SingleSelectionAnswer::<StandardSource>::try_new(1).unwrap(),
                )
                .unwrap()
                .into(),
                MultipleSelectionQuestion::try_new(
                    2.into(),
                    "2. Multiple-selection".into(),
                    vec![
                        "option a".into(),
                        "option b".into(),
                        "option c".into(),
                        "option d".into(),
                    ],
                    MultipleSelectionAnswer::<StandardSource>::try_new(vec![2, 3]).unwrap(),
                )
                .unwrap()
                .into(),
                CompletionQuestion::try_new(
                    3.into(),
                    "3. Completion".into(),
                    CompletionAnswer::<StandardSource>::try_new("answer").unwrap(),
                )
                .unwrap()
                .into(),
            ],
        );

        let submission = Submission::new(
            vec![
                SingleSelectionAnswer::<SubmissionSource>::try_new(0)
                    .unwrap()
                    .into(),
                SingleSelectionAnswer::<SubmissionSource>::try_new(0)
                    .unwrap()
                    .into(),
                MultipleSelectionAnswer::<SubmissionSource>::try_new(vec![2, 3])
                    .unwrap()
                    .into(),
                CompletionAnswer::<SubmissionSource>::try_new("answer")
                    .unwrap()
                    .into(),
            ]
            .into_iter()
            .enumerate()
            .map(|(i, a)| (i.into(), a))
            .collect(),
        );

        assert_eq!(test.grade(&submission), Score::try_new(75f32).unwrap());
    }
}
