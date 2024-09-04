use snafu::prelude::*;

use crate::domain::entity::answer::{
    Answer, CompletionAnswer, MultipleSelectionAnswer, SingleSelectionAnswer, StandardSource,
    SubmissionSource,
};
use crate::domain::entity::id::Id;

#[enum_dispatch::enum_dispatch]
pub trait AbstractQuestion {
    fn id(&self) -> Id;

    fn content(&self) -> &str;

    fn check(&self, submission: &Answer<SubmissionSource>) -> bool;
}

#[enum_dispatch::enum_dispatch(AbstractQuestion)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Question {
    SingleSelection(SingleSelectionQuestion),
    MultipleSelection(MultipleSelectionQuestion),
    Completion(CompletionQuestion),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SingleSelectionQuestion {
    id: Id,
    content: String,
    options: Vec<String>,
    answer: SingleSelectionAnswer<StandardSource>,
}

impl SingleSelectionQuestion {
    pub fn try_new(
        id: Id,
        content: String,
        options: Vec<String>,
        answer: SingleSelectionAnswer<StandardSource>,
    ) -> Result<Self, TryNewQuestionError> {
        ensure!(!content.is_empty(), ContentEmptySnafu);
        ensure!(!options.is_empty(), NoOptionSnafu);
        ensure!(options.iter().all(|o| !o.is_empty()), OptionEmptySnafu);

        Ok(Self {
            id,
            content,
            options,
            answer,
        })
    }

    pub fn options(&self) -> &Vec<String> {
        &self.options
    }
}

impl AbstractQuestion for SingleSelectionQuestion {
    fn id(&self) -> Id {
        self.id
    }

    fn content(&self) -> &str {
        &self.content
    }

    fn check(&self, submission: &Answer<SubmissionSource>) -> bool {
        self.answer.check(submission)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MultipleSelectionQuestion {
    id: Id,
    content: String,
    options: Vec<String>,
    answer: MultipleSelectionAnswer<StandardSource>,
}

impl MultipleSelectionQuestion {
    pub fn try_new(
        id: Id,
        content: String,
        options: Vec<String>,
        answer: MultipleSelectionAnswer<StandardSource>,
    ) -> Result<Self, TryNewQuestionError> {
        ensure!(!content.is_empty(), ContentEmptySnafu);
        ensure!(!options.is_empty(), NoOptionSnafu);
        ensure!(options.iter().all(|o| !o.is_empty()), OptionEmptySnafu);

        Ok(Self {
            id,
            content,
            options,
            answer,
        })
    }

    pub fn options(&self) -> &Vec<String> {
        &self.options
    }
}

impl AbstractQuestion for MultipleSelectionQuestion {
    fn id(&self) -> Id {
        self.id
    }

    fn content(&self) -> &str {
        &self.content
    }

    fn check(&self, submission: &Answer<SubmissionSource>) -> bool {
        self.answer.check(submission)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletionQuestion {
    id: Id,
    content: String,
    answer: CompletionAnswer<StandardSource>,
}

impl CompletionQuestion {
    pub fn try_new(
        id: Id,
        content: String,
        answer: CompletionAnswer<StandardSource>,
    ) -> Result<Self, TryNewQuestionError> {
        ensure!(!content.is_empty(), ContentEmptySnafu);

        Ok(Self {
            id,
            content,
            answer,
        })
    }
}

impl AbstractQuestion for CompletionQuestion {
    fn id(&self) -> Id {
        self.id
    }

    fn content(&self) -> &str {
        &self.content
    }

    fn check(&self, submission: &Answer<SubmissionSource>) -> bool {
        self.answer.check(submission)
    }
}

#[derive(Debug, Clone, Snafu, PartialEq, Eq)]
#[non_exhaustive]
pub enum TryNewQuestionError {
    #[snafu(display("Question must have non-empty content"))]
    ContentEmpty,
    #[snafu(display("Question must not have empty option"))]
    OptionEmpty,
    #[snafu(display("Question must have at least one option"))]
    NoOption,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn question_try_new_content_empty() {
        assert!(matches!(
            SingleSelectionQuestion::try_new(
                0.into(),
                "".into(),
                vec!["option-0".into()],
                SingleSelectionAnswer::<StandardSource>::try_new(0)
                    .unwrap()
                    .into()
            ),
            Err(TryNewQuestionError::ContentEmpty),
        ));
    }

    #[test]
    fn question_try_new_no_option() {
        assert!(matches!(
            MultipleSelectionQuestion::try_new(
                0.into(),
                "content".into(),
                vec![],
                MultipleSelectionAnswer::<StandardSource>::try_new(vec![0])
                    .unwrap()
                    .into()
            ),
            Err(TryNewQuestionError::NoOption),
        ));
    }

    #[test]
    fn question_try_new_option_empty() {
        assert!(matches!(
            SingleSelectionQuestion::try_new(
                0.into(),
                "content".into(),
                vec!["".into()],
                SingleSelectionAnswer::<StandardSource>::try_new(0)
                    .unwrap()
                    .into()
            ),
            Err(TryNewQuestionError::OptionEmpty),
        ));
    }

    #[test]
    fn question_check() {
        let question: Question = SingleSelectionQuestion::try_new(
            0.into(),
            "content".into(),
            vec!["option-0".into()],
            SingleSelectionAnswer::<StandardSource>::try_new(0)
                .unwrap()
                .into(),
        )
        .unwrap()
        .into();

        let correct = SingleSelectionAnswer::<SubmissionSource>::try_new(0).unwrap();
        assert!(question.check(&correct.into()));
        let wrong = SingleSelectionAnswer::<SubmissionSource>::try_new(1).unwrap();
        assert!(!question.check(&wrong.into()));
        let mismatched = CompletionAnswer::<SubmissionSource>::try_new("").unwrap();
        assert!(!question.check(&mismatched.into()));
    }
}
