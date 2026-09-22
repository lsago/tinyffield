/// Formats GF(2) coefficients packed into a vec of u64s, with bit i representing x^i.
pub(crate) fn fmt(bitfield: &[u64], f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut chunks = bitfield.iter().enumerate().rev().peekable();

    let mut written = false;
    while let Some((chunk_index, chunk)) = chunks.next() {
        let mut bits = chunk.clone();
        while bits != 0 {
            let bit_index = (u64::BITS - 1 - bits.leading_zeros()) as usize;

            if written {
                write!(f, " + ")?;
            }

            if bit_index == 0 && chunk_index == 0 {
                write!(f, "1")?;
            } else if bit_index == 1 && chunk_index == 0 {
                write!(f, "x")?;
            } else {
                write!(f, "x^{}", bit_index + chunk_index * 64)?;
            }
            written = true;

            // clear this bit
            bits &= !(1u64 << bit_index);
        }
    }

    if !written {
        write!(f, "0")?;
    }

    Ok(())
}
