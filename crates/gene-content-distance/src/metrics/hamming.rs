pub fn hamming_distance(a: &[u8], b: &[u8]) -> usize {
    // Zip will silently fail if the slices mistmatch in dimension
    assert_eq!(a.len(), b.len());

    // Filter for the conditon then count the true
    a.iter().zip(b.iter()).filter(|(x, y)| x != y).count()
}

pub fn hamming_packed(a: &[u64], b: &[u64]) -> usize {
    // Zip will silently fail if the slices mistmatch in dimension
    assert_eq!(a.len(), b.len());

    a.iter()
        .zip(b.iter())
        .map(|(x, y)| (x ^ y).count_ones() as usize)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backends::bitpacked::pack_genes;

    #[test]
    fn identical_genomes() {
        let a = [1, 0, 1, 1];
        assert_eq!(hamming_distance(&a, &a), 0);
    }

    #[test]
    fn different_genomes() {
        let a = [1, 0, 1, 1, 0, 1];
        let b = [1, 1, 0, 1, 0, 0];

        assert_eq!(hamming_distance(&a, &b), 3);
    }

    #[test]
    fn completely_different() {
        let a = [1, 1, 1, 1];
        let b = [0, 0, 0, 0];

        assert_eq!(hamming_distance(&a, &b), 4);
    }

    #[test]
    fn packed_hamming_distance() {
        let a = pack_genes(&[1, 0, 1, 1, 0, 1]);
        let b = pack_genes(&[1, 1, 0, 1, 0, 0]);

        assert_eq!(hamming_packed(&a, &b), 3);
    }
}
