use structopt::StructOpt;

#[derive(StructOpt)]
enum Command {
    Euclid {
         m: usize,
         n: usize,
    },
}

fn main() {
    let cmd = Command::from_args();
    match cmd {
        Command::Euclid { m, n } => {
            println!("euclid m = {}, n = {}: {}", m, n, euclid(m, n))
        },
    }
}


fn euclid(mut m: usize, mut n: usize) -> usize {
    if m >= n {
        let t = m;
        m = n;
        n = t;
    }
    let mut r = m % n;
    while r != 0 {
       println!("{} % {} = {}", m, n, r);
       m = n;
       n = r;
       r = m % n;
    }
    n
}
