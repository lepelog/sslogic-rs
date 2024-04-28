use rand::seq::index::sample;
use rand::Rng;

pub fn sample_stable<'a, R: Rng, I>(
    rng: &mut R,
    items: &'a [I],
    count: usize,
) -> impl Iterator<Item = &'a I> {
    let mut indices = sample(rng, items.len(), count).into_vec();
    indices.sort_unstable();
    indices.into_iter().map(|i| &items[i])
}
