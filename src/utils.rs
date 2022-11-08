use fake::{vec, Dummy, Faker};
use serde::Serializer;

pub fn generate_n_times<T, const N: usize>() -> Vec<T>
where T: Dummy<Faker> {
	vec![T; N]
}

pub fn join_string<S>(
	paragraphs: &[String],
	serializer: S,
) -> Result<S::Ok, S::Error>
where
	S: Serializer,
{
	serializer.serialize_str(&paragraphs.join("\n\n"))
}
