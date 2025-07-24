pub mod input;
pub mod plot;
pub mod trainer;

use rand::seq::SliceRandom;
use rand::thread_rng;
use redeem_properties::utils::data_handling::PeptideData;

pub fn sample_peptides(peptides: &[PeptideData], n: usize) -> Vec<PeptideData> {
    let mut rng = thread_rng();
    let sample_size = n.min(peptides.len());
    peptides
        .choose_multiple(&mut rng, sample_size)
        .cloned()
        .collect()
}
