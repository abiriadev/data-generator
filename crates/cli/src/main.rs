use schema::{structures::post::Post, utils::generate_n_times};

const LENGTH: usize = 5;

fn main() -> Result<(), serde_json::Error> {
	generate_n_times::<Post, LENGTH>()
		.iter()
		.map(serde_json::to_string)
		.try_for_each(|user| Ok(println!("{}", user?)))
}
