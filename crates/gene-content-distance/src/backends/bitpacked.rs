pub fn pack_genes(genes: &[u8]) -> Vec<u64> {
    let mut packed_genes = vec![0u64; genes.len().div_ceil(64)];

    // for 3 genes packed_genes = [float64 float64 float64]
    for (pack, chunk) in packed_genes.iter_mut().zip(genes.chunks(64)) {
        for (pos, gene) in chunk.iter().enumerate() {
            if *gene != 1 {
                continue;
            }

            *pack |= 1u64 << pos;
        }
    }
    packed_genes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pack_small_genome() {
        let genes = [1, 0, 1, 1];

        assert_eq!(pack_genes(&genes), vec![0b1101]);
    }

    #[test]
    fn pack_empty_genome() {
        assert_eq!(pack_genes(&[]), Vec::<u64>::new());
    }

    #[test]
    fn pack_more_than_64_genes() {
        let genes = vec![1u8; 65];

        assert_eq!(pack_genes(&genes), vec![u64::MAX, 1]);
    }
}
