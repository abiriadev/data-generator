use data_generator::{structures::User, utils::generate_n_times};

const LENGTH: usize = 5;

fn main() -> Result<(), serde_json::Error> {
	generate_n_times::<User, LENGTH>()
		.iter()
		.map(serde_json::to_string)
		.try_for_each(|user| Ok(println!("{}", user?)))
}
