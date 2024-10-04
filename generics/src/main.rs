trait BitOps<T> {

    fn or(a: T, b: u8) -> T;
    fn shl(a: T, b: u8) -> T;
}

macro_rules! bit_imp {
    ($tp:ty) => {
        impl BitOps<$tp> for $tp {
            fn or(a: $tp, b: u8) -> $tp {
                a | (b as $tp)
            }

            fn shl(a: $tp, b: u8) -> $tp {
                a << b
            }
        }
    }
}

bit_imp!(u32);
bit_imp!(u64);

fn bytes_to_number<T>(b: &[u8]) -> T 
where T: Default + BitOps<T> {
    b
        .iter()
        .rev()
        .enumerate()
        .fold(T::default(), |acc, (i, &b)| T::or(T::shl(acc, i as u8 * 8u8), b))
}

fn main() {
    let mut n = bytes_to_number::<u32>(&[1, 10, 3]);
    let mut n = bytes_to_number::<u64>(&[1, 10, 3]);
    let mut s = String::new();

    while n > 0 {
        s.push((48 + (n & 1) as u8) as char);
        n >>= 1;
    }

    println!("{:?}", s);

}
