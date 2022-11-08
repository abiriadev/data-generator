use fake::{faker::internet::raw::FreeEmail, locales::EN, Dummy, Fake, Faker};
use lazy_static::lazy_static;
use nanoid::nanoid;
use serde::Serialize;
use url::Url;

#[derive(Serialize, Debug)]
pub struct Nanoid(String);

impl Dummy<Faker> for Nanoid {
	fn dummy_with_rng<R>(_config: &Faker, _rng: &mut R) -> Self
	where R: rand::Rng + ?Sized {
		Nanoid(nanoid!())
	}
}

#[derive(Serialize, Debug)]
pub struct GravataUrl(Url);

impl Dummy<Faker> for GravataUrl {
	fn dummy_with_rng<R>(_config: &Faker, _rng: &mut R) -> Self
	where R: rand::Rng + ?Sized {
		const GRAVATA_BASE: &'static str = "https://gravatar.com/avatar/";

		lazy_static! {
			static ref GRAVATA_BASE_URL: Url = Url::parse(GRAVATA_BASE)
				.expect("Can't parse base url for gravatar properly");
		}

		let mut u = GRAVATA_BASE_URL
			.join(&format!(
				"{:x}",
				md5::compute(FreeEmail(EN).fake::<String>())
			))
			.expect("Can't join hashed email to gravatar base url");

		u.query_pairs_mut()
			.append_pair("d", "identicon");

		Self(u)
	}
}
