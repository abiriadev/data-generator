use chrono::{DateTime, Utc};
use fake::{
	faker::{chrono::en::DateTime as FakeDateTime, name::en::Name},
	Dummy, Fake,
};
use serde::Serialize;

use crate::{
	dummy::{GravataUrl, Nanoid},
	utils::to_timestamp,
};

#[derive(Serialize, Debug, Dummy)]
#[serde(rename(serialize = "camelCase"))]
pub struct User {
	id: Nanoid,

	#[dummy(faker = "Name()")]
	name: String,

	#[dummy(faker = "FakeDateTime()")]
	#[serde(serialize_with = "to_timestamp")]
	created_at: DateTime<Utc>,

	profile: GravataUrl,
}
