use fake::{vec, Dummy, Faker};

pub fn generate_n_times<T, const N: usize>() -> Vec<T>
where
	T: Dummy<Faker>,
{
	vec![T; N]
}
