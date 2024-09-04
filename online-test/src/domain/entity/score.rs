use snafu::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Score(f32);

impl Score {
    pub fn try_new(score: f32) -> Result<Self, TryNewScoreError> {
        ensure!(0f32 <= score && score <= 100f32, InvalidSnafu);
        Ok(Self(score))
    }

    pub fn inner(&self) -> f32 {
        self.0
    }
}

impl From<Score> for f32 {
    fn from(value: Score) -> Self {
        value.0
    }
}

#[derive(Debug, Clone, Snafu, PartialEq, Eq)]
#[non_exhaustive]
pub enum TryNewScoreError {
    #[snafu(display("Score should be in [0, 100]"))]
    Invalid,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_new_score() {
        let actual = Score::try_new(100f32).unwrap();
        let expected = Score(100f32);
        assert_eq!(actual, expected);
    }

    #[test]
    fn try_new_score_invalid() {
        let small = Score::try_new(-1f32);
        assert!(matches!(small, Err(TryNewScoreError::Invalid)));
        let large = Score::try_new(101f32);
        assert!(matches!(large, Err(TryNewScoreError::Invalid)));
    }
}
