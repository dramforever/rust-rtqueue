// Solution to https://loj.ac/problem/6203

use std::io;
use std::convert::TryInto;
use rtqueue::Queue;

fn main() {
    fn parse_line() -> Vec<u32> {
        let mut line: String = "".to_string();
        io::stdin().read_line(&mut line)
            .expect("read_line");

        line.split_whitespace()
            .map(|val| { val.parse().expect("Parse") })
            .collect()
    }

    let (n, ty) = match parse_line().as_slice() {
        [ n, ty ] => (*n, *ty),
        x => panic!("Bad header line {:?}", x),
    };

    let mut hash: u32 = 0;

    let ver_size: usize = (n + 1).try_into()
        .expect("main: size too large");

    let mut version: Vec<Queue<u32>> =
        Vec::with_capacity(ver_size);

    version.push(Queue::new());

    for _ in 1 ..= n {

        let h = if ty == 1 { hash } else { 0 };

        match parse_line().as_slice() {
            [ 1, v, t ] => {
                let v = v ^ h;
                let t = t ^ h;
                version.push(version[v as usize].push_back(t));
            },
            [ 2, v ] => {
                let v = v ^ h;

                let (res_queue, res_val) = version[v as usize].pop_front()
                    .expect("main: queue is empty");

                version.push(res_queue);

                hash = hash.wrapping_mul(31).wrapping_add(res_val);
            },
            x => {
                panic!("Bad operation {:?}", x);
            },
        }
    }

    println!("{}", hash);

    // for (i, q) in version.iter().enumerate() {
    //     println!("{} {:?}", i, q);
    // }
}
