#![feature(test)]
extern crate test;

extern crate may;
#[macro_use]
extern crate generator;

use may::coroutine;
use generator::Generator;

fn filter<'a>(vec: &'a [bool], step: usize) {
    // the least step is 3, we already filter step=2 in the vec representation
    if step < 3 {
        return;
    }
    #[allow(mutable_transmutes)]
    let mut_vec: &mut [bool] = unsafe { ::std::mem::transmute(vec) };
    // step form beginning, ignore the very first one which is a prime number
    // let mut i = step + step - 1;
    let mut i = step / 2 + step;
    let len = vec.len();

    // mark the non-prime ones, skip the frist one
    while i < len {
        // concurrent write the same value is ok!!
        mut_vec[i] = false;
        i += step;
    }
}

pub fn prime(max: usize) -> Generator<'static, (), usize> {

    //=========================
    // early return
    //=========================

    if max <= 2 {
        return generator::Gn::new(|| 2);
    }

    // if max <= 210 {
    //     return generator::Gn::new_scoped(move |mut s| {
    //         let vec = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71,
    //                    73, 79, 83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151,
    //                    157, 163, 167, 173, 179, 181, 191, 193, 197, 199];
    //         for i in vec.iter() {
    //             if *i > max {
    //                 break;
    //             }
    //             s.yield_with(*i);
    //         }
    //         done!();
    //     });
    // }

    // alloc the vec in heap, ignore the step=2 items(odd numbers)
    let mut vec = vec![true; (max + 1) / 2];
    // mark 1 as non-prime
    vec[0] = false;
    let top = (max as f32).sqrt() as usize + 1;
    // println!("top = {}", top);

    coroutine::scope(|s| for i in prime(top) {
                         let v = &vec;
                         s.spawn(move || filter(&v, i));
                     });



    generator::Gn::new_scoped(move |mut s| {
        s.yield_with(2);
        for (i, v) in vec.iter().enumerate() {
            if *v {
                s.yield_with(i * 2 + 1);
            }
        }
        done!();
    })
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut sum = 0;
        for v in prime(500) {
            sum += v;
        }
        assert_eq!(sum, 21536);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        may::config().set_workers(4);
        b.iter(|| prime(1_000_000));
    }
}
