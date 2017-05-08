#![feature(test)]
extern crate test;

extern crate may;
#[macro_use]
extern crate generator;

use may::coroutine;
use generator::Generator;

fn filter<'a>(vec: &'a [u8], step: usize) {
    #[allow(mutable_transmutes)]
    let mut_vec: &mut [u8] = unsafe { ::std::mem::transmute(vec) };
    // step form beginning
    let mut i = 1;

    // mark the non-prime ones, skip the frist one
    for v in &mut mut_vec[step..] {
        if i == step {
            *v = 1;
            i = 1;
            continue;
        }
        i += 1;
    }
}

pub fn prime(max: usize) -> Generator<'static, (), usize> {

    //=========================
    // early return
    //=========================

    // if max <= 2 {
    //     return generator::Gn::new(|| 2);
    // }

    if max <= 210 {
        return generator::Gn::new_scoped(move |mut s| {
            let vec = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71,
                       73, 79, 83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151,
                       157, 163, 167, 173, 179, 181, 191, 193, 197, 199];
            for i in vec.iter() {
                if *i > max {
                    break;
                }

                s.yield_with(*i);
            }
            done!();
        });
    }

    // alloc the vec in heap
    let mut vec = vec![0u8; max];
    // mark 1 as non-prime
    vec[0] = 1;
    let top = (max as f32).sqrt() as usize + 1;
    // println!("top = {}", top);

    coroutine::scope(|s| for i in prime(top) {
                         let v = &vec;
                         s.spawn(move || filter(&v, i));
                     });



    generator::Gn::new_scoped(move |mut s| {
                                  for (i, v) in vec.iter().enumerate() {
                                      if *v == 0 {
                                          s.yield_with(i + 1);
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
