use fake::faker::name::raw::Name;
use fake::locales::EN;
use fake::Dummy;
use fake::Fake;
use serde::Serialize;

use crate::dummy::Nanoid;

#[derive(Serialize, Debug, Dummy)]
pub struct User {
	id: Nanoid,

	#[dummy(faker = "Name(EN)")]
	name: String,
}
