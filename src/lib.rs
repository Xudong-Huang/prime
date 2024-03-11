use std::iter::FromIterator;
use std::sync::atomic::{AtomicBool, Ordering};

// quick algorithm for usize sqrt
// fn fast_sqrt(n: usize) -> usize {
//     let mut delta = 3;
//     let mut square = 1;
//     while square <= n {
//         square += delta;
//         delta += 2;
//     }
//     (delta >> 1) - 1
// }

fn filter(vec: &[AtomicBool], step: usize) {
    // step form beginning, ignore the very first one which is a prime number
    let mut i = step / 2 + step;

    // mark the non-prime ones
    let len = vec.len();
    while i < len {
        // concurrent write the same value is ok!!
        vec[i].store(false, Ordering::Relaxed);
        i += step;
    }
}

pub fn prime(max: usize) -> impl Iterator<Item = usize> + 'static {
    // early return
    if max <= 210 {
        return generator::Gn::new_scoped(move |mut s| {
            let vec = [
                2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79,
                83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167,
                173, 179, 181, 191, 193, 197, 199,
            ];
            for i in vec.iter().take_while(|v| **v <= max) {
                s.yield_with(*i);
            }
            generator::done!();
        });
    }

    // alloc the vec in heap, ignore the step=2 items(odd numbers)
    let vec =
        Vec::from_iter(std::iter::from_fn(|| Some(AtomicBool::new(true))).take((max + 1) / 2));
    // mark 1 as non-prime
    vec[0].store(false, Ordering::Relaxed);

    let top = (max as f32).sqrt() as usize;
    // let top = fast_sqrt(max);

    // skip step=2 which is already filtered
    may::coroutine::scope(|s| {
        for i in prime(top).skip(1) {
            let v = &vec;
            may::go!(s, move || filter(v, i));
        }
    });

    generator::Gn::new_scoped(move |mut s| {
        s.yield_with(2);
        for (i, _) in vec
            .into_iter()
            .enumerate()
            .filter(|(_, v)| v.load(Ordering::Relaxed))
        {
            s.yield_with(i * 2 + 1);
        }
        generator::done!();
    })
}

#[cfg(test)]
mod tests {
    // extern crate test;
    use super::*;

    #[test]
    fn it_works() {
        let sum = prime(500).sum::<usize>();
        assert_eq!(sum, 21536);
    }

    // #[bench]
    // fn bench(b: &mut test::Bencher) {
    //     may::config().set_workers(4);
    //     b.iter(|| prime(1_000_000));
    // }
}
