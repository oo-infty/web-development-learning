use snafu::prelude::*;

pub trait AnswerSourceMarker: Clone + Copy + PartialEq + Eq + Default {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnswerSource {
    Standard(StandardSource),
    Submission(SubmissionSource),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct StandardSource;

impl From<StandardSource> for AnswerSource {
    fn from(value: StandardSource) -> Self {
        Self::Standard(value)
    }
}

impl AnswerSourceMarker for StandardSource {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SubmissionSource;

impl From<SubmissionSource> for AnswerSource {
    fn from(value: SubmissionSource) -> Self {
        Self::Submission(value)
    }
}

impl AnswerSourceMarker for SubmissionSource {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Answer<Src: AnswerSourceMarker> {
    SingleSelection(SingleSelectionAnswer<Src>),
    MultipleSelection(MultipleSelectionAnswer<Src>),
    Completion(CompletionAnswer<Src>),
}

impl Answer<StandardSource> {
    pub fn source(&self) -> AnswerSource {
        match self {
            Self::SingleSelection(s) => s.source(),
            Self::MultipleSelection(s) => s.source(),
            Self::Completion(s) => s.source(),
        }
    }

    pub fn check(&self, submission: &Answer<SubmissionSource>) -> bool {
        match self {
            Self::SingleSelection(s) => s.check(submission),
            Self::MultipleSelection(s) => s.check(submission),
            Self::Completion(s) => s.check(submission),
        }
    }
}

impl Answer<SubmissionSource> {
    pub fn source(&self) -> AnswerSource {
        match self {
            Self::SingleSelection(s) => s.source(),
            Self::MultipleSelection(s) => s.source(),
            Self::Completion(s) => s.source(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SingleSelectionAnswer<Src: AnswerSourceMarker> {
    value: u32,
    source: Src,
}

impl<Src: AnswerSourceMarker> SingleSelectionAnswer<Src> {
    pub fn try_new(value: u32) -> Result<Self, TryNewAnswerError> {
        Ok(SingleSelectionAnswer {
            value,
            source: Default::default(),
        })
    }
}

impl<Src: AnswerSourceMarker> From<SingleSelectionAnswer<Src>> for Answer<Src> {
    fn from(value: SingleSelectionAnswer<Src>) -> Self {
        Answer::SingleSelection(value)
    }
}

impl SingleSelectionAnswer<StandardSource> {
    pub fn source(&self) -> AnswerSource {
        self.source.into()
    }

    pub fn check(&self, submission: &Answer<SubmissionSource>) -> bool {
        match submission {
            Answer::SingleSelection(submission) => self.value == submission.value,
            _ => false,
        }
    }
}

impl SingleSelectionAnswer<SubmissionSource> {
    pub fn source(&self) -> AnswerSource {
        self.source.into()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MultipleSelectionAnswer<Src: AnswerSourceMarker> {
    value: Vec<u32>,
    source: Src,
}

impl<Src: AnswerSourceMarker> From<MultipleSelectionAnswer<Src>> for Answer<Src> {
    fn from(value: MultipleSelectionAnswer<Src>) -> Self {
        Answer::MultipleSelection(value)
    }
}

impl MultipleSelectionAnswer<StandardSource> {
    pub fn try_new(mut value: Vec<u32>) -> Result<Self, TryNewAnswerError> {
        ensure!(!value.is_empty(), OptionEmptySnafu);
        value.sort();
        ensure!(
            value.iter().zip(value.iter().skip(1)).all(|(x, y)| x != y),
            OptionDuplicatedSnafu
        );
        Ok(MultipleSelectionAnswer {
            value,
            source: Default::default(),
        })
    }

    pub fn source(&self) -> AnswerSource {
        self.source.into()
    }

    pub fn check(&self, submission: &Answer<SubmissionSource>) -> bool {
        match submission {
            Answer::MultipleSelection(submission) => self.value == submission.value,
            _ => false,
        }
    }
}

impl MultipleSelectionAnswer<SubmissionSource> {
    pub fn try_new(mut value: Vec<u32>) -> Result<Self, TryNewAnswerError> {
        value.sort();
        ensure!(
            value.iter().zip(value.iter().skip(1)).all(|(x, y)| x != y),
            OptionDuplicatedSnafu
        );
        Ok(MultipleSelectionAnswer {
            value,
            source: Default::default(),
        })
    }

    pub fn source(&self) -> AnswerSource {
        self.source.into()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletionAnswer<Src: AnswerSourceMarker> {
    value: String,
    source: Src,
}

impl<Src: AnswerSourceMarker> From<CompletionAnswer<Src>> for Answer<Src> {
    fn from(value: CompletionAnswer<Src>) -> Self {
        Answer::Completion(value)
    }
}

impl CompletionAnswer<StandardSource> {
    pub fn try_new<S: AsRef<str>>(value: S) -> Result<Self, TryNewAnswerError> {
        ensure!(!value.as_ref().is_empty(), ContentEmptySnafu);
        Ok(CompletionAnswer {
            value: value.as_ref().into(),
            source: Default::default(),
        })
    }

    pub fn source(&self) -> AnswerSource {
        self.source.into()
    }

    pub fn check(&self, submission: &Answer<SubmissionSource>) -> bool {
        match submission {
            Answer::Completion(submission) => self.value == submission.value,
            _ => false,
        }
    }
}

impl CompletionAnswer<SubmissionSource> {
    pub fn try_new<S: AsRef<str>>(value: S) -> Result<Self, TryNewAnswerError> {
        Ok(CompletionAnswer {
            value: value.as_ref().into(),
            source: Default::default(),
        })
    }

    pub fn source(&self) -> AnswerSource {
        self.source.into()
    }
}

#[derive(Debug, Clone, Snafu, PartialEq, Eq)]
pub enum TryNewAnswerError {
    #[snafu(display("Multiple selection have duplicated options"))]
    OptionDuplicated,
    #[snafu(display("Multiple selection must have at least one correct option"))]
    OptionEmpty,
    #[snafu(display("Completion must have non-empty content as answer"))]
    ContentEmpty,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn answer_try_new_option_duplicated() {
        assert!(matches!(
            MultipleSelectionAnswer::<StandardSource>::try_new(vec![0, 0, 1]),
            Err(TryNewAnswerError::OptionDuplicated),
        ));
        assert!(matches!(
            MultipleSelectionAnswer::<SubmissionSource>::try_new(vec![0, 0, 1]),
            Err(TryNewAnswerError::OptionDuplicated),
        ));
    }

    #[test]
    fn answer_try_new_option_empty() {
        assert!(matches!(
            MultipleSelectionAnswer::<StandardSource>::try_new(vec![]),
            Err(TryNewAnswerError::OptionEmpty),
        ));
        assert!(MultipleSelectionAnswer::<SubmissionSource>::try_new(vec![]).is_ok());
    }

    #[test]
    fn answer_try_new_content_empty() {
        assert!(matches!(
            CompletionAnswer::<StandardSource>::try_new(""),
            Err(TryNewAnswerError::ContentEmpty),
        ));
        assert!(CompletionAnswer::<SubmissionSource>::try_new("").is_ok());
    }

    #[test]
    fn answer_matches() {
        let a1 = SingleSelectionAnswer::<StandardSource>::try_new(0).unwrap();
        let a2 = MultipleSelectionAnswer::<StandardSource>::try_new(vec![1]).unwrap();
        let a3 = CompletionAnswer::<StandardSource>::try_new("answer").unwrap();
        let a4: Answer<SubmissionSource> = CompletionAnswer::<SubmissionSource>::try_new("answer")
            .unwrap()
            .into();

        assert!(!a1.check(&a4));
        assert!(!a2.check(&a4));
        assert!(a3.check(&a4));
    }
}
