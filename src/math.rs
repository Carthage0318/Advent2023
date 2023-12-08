use num::{Integer, Num, Signed, Unsigned};
use prime_factorization::Factorization;

/// Sign is not guaranteed if one or both input arguments are negative.
pub fn gcd<T: Num + Copy + Integer>(a: T, b: T) -> T {
    if b.is_zero() {
        a
    } else {
        gcd(b, a % b)
    }
}

#[allow(dead_code)]
pub fn is_coprime_unsigned<T: Num + Copy + Unsigned + Integer>(a: T, b: T) -> bool {
    gcd(a, b).is_one()
}

#[allow(dead_code)]
pub fn is_coprime_signed<T: Num + Copy + Signed + Integer>(a: T, b: T) -> bool {
    gcd(a, b).abs().is_one()
}

/// Returns `None` if the argument is empty.
/// Otherwise will return `Some` with the least common multiple of the values.
pub fn lcm<T: Num + Copy + Unsigned + Integer>(values: &[T]) -> Option<T> {
    values.iter().copied().reduce(|a, b| (a / gcd(a, b)) * b)
}

/// Computes the gcd, as well as bezout coefficients.
/// returns `(gcd, m, n)` where `am + bn = gcd`.
///
/// Sign of the gcd is not guaranteed if one or both of the arguments is negative.
fn extended_euclidean<T: Num + Copy + Signed + Integer>(a: T, b: T) -> (T, T, T) {
    fn internal<T: Num + Copy + Integer>(r0: T, r1: T, s0: T, s1: T, t0: T, t1: T) -> (T, T, T) {
        if r1.is_zero() {
            (r0, s0, t0)
        } else {
            let (q, r2) = r0.div_rem(&r1);
            let s2 = s0 - q * s1;
            let t2 = t0 - q * t1;
            internal(r1, r2, s1, s2, t1, t2)
        }
    }

    internal(a, b, T::one(), T::zero(), T::zero(), T::one())
}

/// Computes a value satisfying the system of congruences provided by the inputs.
/// The moduli are not required to be pairwise coprime to call this function,
/// but if they are not, the possibility of no solution exists.
/// In such a situation, `None` is returned.
pub fn chinese_remainder_theorem(remainders: &[u64], moduli: &[u64]) -> Option<u64> {
    if remainders.len() != moduli.len() {
        return None;
    }

    let init_modulus = moduli[0];
    let init_remainder = remainders[0].rem_euclid(init_modulus);

    remainders
        .iter()
        .zip(moduli)
        .skip(1)
        .try_fold((init_remainder, init_modulus), |(a1, n1), (&a2, &n2)| {
            // Handle possibility that n1 and n2 may not be coprime.
            // If they aren't, we need to check that the system is solvable,
            // and simplify if so.
            let gcd = gcd(n1, n2);
            if a1.rem_euclid(gcd) != a2.rem_euclid(gcd) {
                // There's disagreement on the remainder for the shared gcd. No solution.
                return None;
            }

            // Now force n2 to be coprime to n1
            let n2 = if gcd == 1 {
                n2
            } else {
                let prime_factorization = Factorization::run(gcd);
                prime_factorization
                    .factors
                    .iter()
                    .fold(n2, |mut current, &factor| {
                        while current % factor == 0 {
                            current /= factor;
                        }
                        current
                    })
            };
            let new_mod = n1 * n2;

            let (_, m1, m2) = extended_euclidean(n1 as i128, n2 as i128);
            let solution = (a1 as i128 * m2 * n2 as i128 + a2 as i128 * m1 * n1 as i128)
                .rem_euclid(new_mod as i128) as u64;

            Some((solution, new_mod))
        })
        .map(|(solution, _)| solution)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        let cases = [
            (1u32, 1u32, 1u32),
            (12, 8, 4),
            (8, 12, 4),
            (12, 7, 1),
            (7, 12, 1),
            (5, 3, 1),
            (1, 1, 1),
            (10, 10, 10),
        ];

        for (a, b, d) in cases {
            assert_eq!(d, gcd(a, b), "gcd({}, {})", a, b);
        }
    }

    #[test]
    fn is_coprime_unsigned_test() {
        let cases = [
            (1u32, 1u32, true),
            (12, 8, false),
            (8, 12, false),
            (100, 57, true),
            (10, 15, false),
            (10, 9, true),
        ];

        for (a, b, result) in cases {
            assert_eq!(
                result,
                is_coprime_unsigned(a, b),
                "is_coprime_unsigned({}, {})",
                a,
                b
            );
        }
    }

    #[test]
    fn is_coprime_signed_test() {
        let cases = [
            (1, 1, true),
            (-1, 1, true),
            (12, -8, false),
            (-8, -12, false),
            (100, 57, true),
            (10, -15, false),
            (-10, 9, true),
        ];

        for (a, b, result) in cases {
            assert_eq!(
                result,
                is_coprime_signed(a, b),
                "is_coprime_signed({}, {})",
                a,
                b
            );
        }
    }

    #[test]
    fn test_extended_euclidean() {
        let cases = [
            (1, 1, 1),
            (12, 8, 4),
            (8, 12, 4),
            (12, 7, 1),
            (7, 12, 1),
            (5, 3, 1),
            (1, 1, 1),
            (10, 10, 10),
            (240, 46, 2),
            (46, 240, 2),
        ];

        for (a, b, gcd) in cases {
            let (d, m, n) = extended_euclidean(a, b);
            assert_eq!(gcd, d, "extended_euclidean - gcd({}, {})", a, b);
            assert_eq!(
                gcd,
                a * m + b * n,
                "extended_euclidean - bezot({}, {})",
                a,
                b
            );
        }
    }

    #[test]
    fn test_crt() {
        let cases = [
            (vec![2u64, 3, 2], vec![3u64, 5, 7], 23u64),
            (vec![1, 1], vec![4, 2], 1),
            (vec![1, 1], vec![2, 4], 1),
        ];

        for (remainders, moduli, expected) in cases {
            assert_eq!(
                expected,
                chinese_remainder_theorem(&remainders, &moduli).unwrap(),
                "CRT: Rem {remainders:?} | Moduli {moduli:?}"
            );
        }

        assert_eq!(None, chinese_remainder_theorem(&[2, 3], &[10, 15]));
    }
}
