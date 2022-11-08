use fake::{faker::internet::raw::FreeEmail, locales::EN, Dummy, Fake, Faker};
use nanoid::nanoid;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Nanoid(String);

impl Dummy<Faker> for Nanoid {
	fn dummy_with_rng<R>(_config: &Faker, _rng: &mut R) -> Self
	where R: rand::Rng + ?Sized {
		Nanoid(nanoid!())
	}
}

#[derive(Serialize, Debug)]
pub struct GravataUrl(String);

impl Dummy<Faker> for GravataUrl {
	fn dummy_with_rng<R>(_config: &Faker, _rng: &mut R) -> Self
	where R: rand::Rng + ?Sized {
		Self(format!(
			"https://gravatar.com/avatar/{:x}?d=identicon",
			md5::compute(FreeEmail(EN).fake::<String>())
		))
	}
}
