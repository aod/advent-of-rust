const MAGIC: usize = 20201227;
const SUBJ_NUM: usize = 7;

pub(crate) fn loop_size(public_key: usize) -> usize {
    let mut val = 1;
    let mut lsize = 0;
    while val != public_key {
        val = (val * SUBJ_NUM) % MAGIC;
        lsize += 1;
    }
    lsize
}

pub(crate) fn encryption_key(public_key: usize, lsize: usize) -> usize {
    let mut key = 1;
    for _ in 0..lsize {
        key = (key * public_key) % MAGIC;
    }
    key
}
