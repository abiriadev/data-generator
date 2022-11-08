use chrono::{DateTime, Utc};
use fake::{
	faker::{chrono::raw::DateTime as FakeDateTime, name::raw::Name},
	locales::EN,
	Dummy, Fake,
};
use serde::Serialize;

use crate::{dummy::Nanoid, utils::to_timestamp};

#[derive(Serialize, Debug, Dummy)]
#[serde(rename(serialize = "camelCase"))]
pub struct User {
	id: Nanoid,

	#[dummy(faker = "Name(EN)")]
	name: String,

	#[dummy(faker = "FakeDateTime(EN)")]
	#[serde(serialize_with = "to_timestamp")]
	created_at: DateTime<Utc>,
}
