use chrono::{serde::ts_seconds, DateTime, Duration, Utc};
use fake::{
	faker::{chrono::en::DateTimeBetween, name::en::Name},
	Dummy, Fake,
};
use serde::Serialize;

use crate::dummy::{GravataUrl, Nanoid};

const WEEKS: i64 = 12;

#[derive(Serialize, Debug, Dummy)]
#[serde(rename(serialize = "camelCase"))]
pub struct User {
	id: Nanoid,

	#[dummy(faker = "Name()")]
	name: String,

	#[dummy(
		faker = "DateTimeBetween(Utc::now() - Duration::weeks(WEEKS), Utc::now())"
	)]
	#[serde(serialize_with = "ts_seconds::serialize")]
	created_at: DateTime<Utc>,

	profile: GravataUrl,
}
