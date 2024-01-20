
use std::io::Read;

// NOTE: The test only fails if this ends with j
pub const MAGIC: [u8; 5] = *b"1234j";


// NOTE: The test fails for value in 8188..=8191
// These value put the string to search accross two call to read because the buffer size is 8192 by default
pub const BEGIN: usize = 8191;


#[derive(Default)]
struct R {
    read: usize,
}

impl Read for R {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        //dbg!(buf.len());
        if self.read > 100000 {
            return Ok(0);
        }
        let mut from = 0;
        if self.read < BEGIN {
            from = buf.len().min(BEGIN - self.read);
            for x in 0..from {
                buf[x] = 0;
            }
            self.read += from;
        }
        if self.read >= BEGIN && self.read <= BEGIN + MAGIC.len() {
            let to = buf.len().min(BEGIN + MAGIC.len() - self.read + from);
            if to > from {
                buf[from..to]
                    .copy_from_slice(&MAGIC[self.read - BEGIN..self.read - BEGIN + to - from]);
                self.read += to - from;
                from = to;
            }
        }
        for x in from..buf.len() {
            buf[x] = 0;
            self.read += 1;
        }
        Ok(buf.len())
    }
}

fn main() -> std::io::Result<()> {
    let aut = aho_corasick::AhoCorasick::new(&[&MAGIC]);

    // While reading from a vector, it works:
    let mut buf = vec![];
    R::default().read_to_end(&mut buf)?;
    let from_whole = aut.find_iter(&buf).next().unwrap().start();

    //But using stream_find_iter fails!
    let mut file = R::default();
    let begin = aut
        .stream_find_iter(&mut file)
        .next()
        .expect("NOT FOUND!!!!")? // Panic here
        .start();
    println!("Found at {}", begin);
    assert_eq!(from_whole, begin);
    Ok(())
}