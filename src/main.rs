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

    let vheader = parse_line();
    assert!(vheader.len() == 2);

    let n = vheader[0];
    let ty = vheader[1];

    let mut hash: u32 = 0;

    let ver_size: usize = (n + 1).try_into()
        .expect("main: size too large");

    let mut version: Vec<Queue<u32>> =
        Vec::with_capacity(ver_size);

    version.push(Queue::new());

    for _ in 1 ..= n {
        let vaction = parse_line();

        let h = if ty == 1 { hash } else { 0 };

        if vaction[0] == 1 {
            assert!(vaction.len() == 3);
            let v = vaction[1] ^ h;
            let t = vaction[2] ^ h;
            version.push(version[v as usize].push_back(t));
        } else {
            assert!(vaction[0] == 2);
            assert!(vaction.len() == 2);

            let v = vaction[1] ^ h;

            let (res_queue, res_val) = version[v as usize].pop_front()
                .expect("main: queue is empty");

            version.push(res_queue);

            hash = hash.wrapping_mul(31).wrapping_add(res_val);
        }
    }

    println!("{}", hash);

    // for (i, q) in version.iter().enumerate() {
    //     println!("{} {:?}", i, q);
    // }
}
