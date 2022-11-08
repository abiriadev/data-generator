use fake::{Dummy, Faker};
use nanoid::nanoid;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Nanoid(String);

impl Dummy<Faker> for Nanoid {
	fn dummy_with_rng<R>(_config: &Faker, _rng: &mut R) -> Self
	where
		R: rand::Rng + ?Sized,
	{
		Nanoid(nanoid!())
	}
}
