use fake::{faker::name::raw::Name, locales::EN, Dummy, Fake};
use serde::Serialize;

const LENGTH: usize = 5;

#[derive(Serialize, Debug, Dummy)]
struct User {
	#[dummy(faker = "Name(EN)")]
	name: String,
}

fn main() -> Result<(), serde_json::Error> {
	Ok(fake::vec![User; LENGTH]
		.iter()
		.map(serde_json::to_string)
		.try_for_each(|user| Ok(println!("{}", user?)))?)
}
