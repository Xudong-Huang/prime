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
    let mut vec = vec![0u8; max];
    // mark 1 as non-prime
    vec[0] = 1;

    filter(&vec, 2);
    filter(&vec, 3);
    filter(&vec, 5);
    filter(&vec, 7);
    filter(&vec, 11);

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
        for v in prime(200) {
            println!("p = {}", v);
        }
    }
}
