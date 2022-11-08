use fake::{faker::internet::en::FreeEmail, Dummy, Fake, Faker};
use lazy_static::lazy_static;
use nanoid::nanoid;
use serde::Serialize;
use tap::Tap;
use url::Url;

#[derive(Serialize, Debug)]
pub struct Nanoid<const N: usize = 21>(String);

impl<const N: usize> Dummy<Faker> for Nanoid<N> {
	fn dummy_with_rng<R>(_config: &Faker, _rng: &mut R) -> Self
	where R: rand::Rng + ?Sized {
		Self(nanoid!(N))
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

		Self(
			GRAVATA_BASE_URL
				.join(&format!(
					"{:x}",
					md5::compute(FreeEmail().fake::<String>())
				))
				.expect("Can't join hashed email to gravatar base url")
				.tap_mut(|u| {
					u.query_pairs_mut()
						.append_pair("d", "identicon");
				}),
		)
	}
}
