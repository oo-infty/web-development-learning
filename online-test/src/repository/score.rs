use std::error::Error;
use std::fmt::{Debug, Formatter, Result as FmtResult};
use std::sync::Arc;

use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use snafu::{prelude::*, OptionExt};
use tokio::time::Duration;

use crate::domain::entity::{score::Score, user::User};
use crate::domain::repository::score::{
    NotFoundSnafu, Record, ScoreRepository, ScoreRepositoryError,
};

use super::connection::AsyncSqlitePool;

pub struct ScoreSqliteRepository {
    pool: Arc<AsyncSqlitePool>,
}

impl ScoreSqliteRepository {
    pub fn new(pool: Arc<AsyncSqlitePool>) -> Self {
        Self { pool }
    }

    async fn query_impl(
        &self,
        user: &User,
        limit: i64,
        score_desc: bool,
    ) -> Result<Vec<Record>, ScoreRepositoryError> {
        use crate::repository::schema::scores::dsl;

        let mut connection = self
            .pool
            .get()
            .await
            .map_err(|err| -> Box<dyn Error + Send> { Box::new(err) })
            .whatever_context("Could not connect to database")?;

        let stmt = dsl::scores
            .select(DbScore::as_select())
            .filter(dsl::name.eq(user.inner()))
            .limit(limit);

        let res = if score_desc {
            stmt.order_by(dsl::score.desc()).load(&mut connection).await
        } else {
            stmt.order_by(dsl::id.desc()).load(&mut connection).await
        };

        let scores = res
            .map_err(|err| -> Box<dyn Error + Send> { Box::new(err) })
            .whatever_context("Could not load scores from database")?
            .into_iter()
            .map(Into::into)
            .collect();

        Ok(scores)
    }
}

#[async_trait::async_trait]
impl ScoreRepository for ScoreSqliteRepository {
    async fn insert(
        &self,
        user: User,
        score: Score,
        end_time: DateTime<Utc>,
        duration: Duration,
    ) -> Result<(), ScoreRepositoryError> {
        use crate::repository::schema::scores::dsl;

        let mut connection = self
            .pool
            .get()
            .await
            .map_err(|err| -> Box<dyn Error + Send> { Box::new(err) })
            .whatever_context("Could not connect to database")?;

        let insertion = DbScoreInsertion {
            name: user.inner().to_owned(),
            score: score.inner(),
            end_time: end_time.to_rfc3339(),
            duration: duration.as_secs() as i32,
        };

        diesel::insert_into(dsl::scores)
            .values(insertion)
            .execute(&mut connection)
            .await
            .map_err(|err| -> Box<dyn Error + Send> { Box::new(err) })
            .whatever_context("Could not save score to database")?;

        Ok(())
    }

    async fn query_all_sorted(&self, user: &User) -> Result<Vec<Record>, ScoreRepositoryError> {
        self.query_impl(&user, 100, true).await
    }

    async fn query_best(&self, user: &User) -> Result<Record, ScoreRepositoryError> {
        self.query_impl(&user, 1, true)
            .await?
            .into_iter()
            .next()
            .context(NotFoundSnafu { user: user.clone() })
    }

    async fn query_latest(&self, user: &User) -> Result<Record, ScoreRepositoryError> {
        self.query_impl(&user, 1, false)
            .await?
            .into_iter()
            .next()
            .context(NotFoundSnafu { user: user.clone() })
    }
}

impl Debug for ScoreSqliteRepository {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "ScoreSqliteRepository {{ pool: Arc<AsyncSqlitePool> }}")
    }
}

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = crate::repository::schema::scores)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
struct DbScore {
    score: f32,
    end_time: String,
    duration: i32,
}

impl From<DbScore> for Record {
    fn from(value: DbScore) -> Self {
        Self {
            score: Score::try_new(value.score)
                .unwrap_or_else(|_| unreachable!("Score should be already validated")),
            end_time: DateTime::parse_from_rfc3339(&value.end_time)
                .unwrap_or_else(|_| unreachable!("End time should be already validated"))
                .into(),
            duration: Duration::from_secs(value.duration as u64),
        }
    }
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::repository::schema::scores)]
struct DbScoreInsertion {
    name: String,
    score: f32,
    end_time: String,
    duration: i32,
}
