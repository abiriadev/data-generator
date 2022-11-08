use chrono::{DateTime, Utc};
use fake::{vec, Dummy, Faker};
use serde::Serializer;

pub fn generate_n_times<T, const N: usize>() -> Vec<T>
where T: Dummy<Faker> {
	vec![T; N]
}

pub fn to_timestamp<S>(
	date: &DateTime<Utc>,
	serializer: S,
) -> Result<S::Ok, S::Error>
where
	S: Serializer,
{
	serializer.serialize_i64(date.timestamp())
}
