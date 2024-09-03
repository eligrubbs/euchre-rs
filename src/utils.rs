use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;


pub fn get_rdm_gen(seed: Option<u64>) -> ChaCha8Rng {
    match seed {
        Some(num) => {ChaCha8Rng::seed_from_u64(num)},
        None => {ChaCha8Rng::from_entropy()}
    }
}
