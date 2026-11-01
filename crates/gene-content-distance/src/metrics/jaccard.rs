pub fn jaccard_distance(a: &[u8], b: &[u8]) -> f64 {
    // Zip will silently fail if the slices mistmatch in dimension
    assert_eq!(a.len(), b.len());

    let (intersection, union) =
        a.iter()
            .zip(b.iter())
            .fold((0usize, 0usize), |(intersection, union), (&x, &y)| {
                (
                    intersection + usize::from(x == 1 && y == 1),
                    union + usize::from(x == 1 || y == 1),
                )
            });
    1.0 - intersection as f64 / union as f64
}

pub fn jaccard_packed(a: &[u64], b: &[u64]) -> usize {
    // Zip will silently fail if the slices mistmatch in dimension
    assert_eq!(a.len(), b.len());

    a.iter()
        .zip(b.iter())
        .map(|(x, y)| (x ^ y).count_ones() as usize)
        .sum()
}
