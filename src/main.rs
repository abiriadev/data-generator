use data_generator::structures::User;

const LENGTH: usize = 5;

fn main() -> Result<(), serde_json::Error> {
	Ok(fake::vec![User; LENGTH]
		.iter()
		.map(serde_json::to_string)
		.try_for_each(|user| Ok(println!("{}", user?)))?)
}
