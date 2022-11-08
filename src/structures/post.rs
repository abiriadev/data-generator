use chrono::{serde::ts_seconds, DateTime, Duration, Utc};
use fake::{
	faker::{
		chrono::en::DateTimeBetween,
		lorem::en::{Paragraphs, Sentence},
	},
	Dummy, Fake,
};
use serde::Serialize;

use super::user::User;
use crate::{dummy::Nanoid, utils::join_string};

const WEEKS: i64 = 12;

#[derive(Serialize, Debug, Dummy)]
#[serde(rename(serialize = "camelCase"))]
pub struct Post {
	id: Nanoid,

	#[dummy(faker = "DateTimeBetween(Utc::now() - Duration::weeks(WEEKS),
	Utc::now())")]
	#[serde(serialize_with = "ts_seconds::serialize")]
	created_at: DateTime<Utc>,

	#[dummy(faker = "DateTimeBetween(Utc::now() - Duration::weeks(WEEKS),
	Utc::now())")]
	#[serde(serialize_with = "ts_seconds::serialize")]
	updated_at: DateTime<Utc>,

	author: User,

	#[dummy(faker = "Sentence(4..8)")]
	title: String,

	#[dummy(faker = "Paragraphs(3..6)")]
	#[serde(serialize_with = "join_string")]
	content: Vec<String>,

	category: Category,
}

#[derive(Serialize, Dummy, Debug)]
#[serde(rename_all(serialize = "lowercase"))]
pub enum Category {
	Report,
	Qa,
	Info,
}
