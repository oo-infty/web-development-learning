use snafu::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User(String);

impl User {
    pub fn try_new<S: AsRef<str>>(user: S) -> Result<Self, TryNewUserError> {
        ensure!(!user.as_ref().is_empty(), EmptySnafu);
        ensure!(
            user.as_ref()
                .find(|c: char| !c.is_ascii_alphanumeric() && c != '-' && c != '_')
                .is_none(),
            InvalidSnafu
        );
        Ok(Self(user.as_ref().into()))
    }

    pub fn inner(&self) -> &str {
        &self.0
    }
}

impl From<User> for String {
    fn from(value: User) -> Self {
        value.0
    }
}

#[derive(Debug, Clone, Snafu, PartialEq, Eq)]
#[non_exhaustive]
pub enum TryNewUserError {
    #[snafu(display("Username should not be empty"))]
    Empty,
    #[snafu(display("Username should only contains alphabets, numbers, dash and underscore"))]
    Invalid,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_new_user() {
        let actual = User::try_new("oo-infty").unwrap();
        let expected = User("oo-infty".into());
        assert_eq!(actual, expected);
    }

    #[test]
    fn try_new_user_empty() {
        let actual = User::try_new("");
        assert!(matches!(actual, Err(TryNewUserError::Empty)));
    }

    #[test]
    fn try_new_user_invalid() {
        let actual = User::try_new("invalid!");
        assert!(matches!(actual, Err(TryNewUserError::Invalid)));
    }
}
