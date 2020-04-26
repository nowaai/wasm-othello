pub const LOWEST_BIT_MASK: u64 = 0x0000000000000001u64;
const TOP_BOTTOM_MASK: u64 = 0xffffffffffffffffu64;
const LEFT_RIGHT_MASK: u64 = 0x7e7e7e7e7e7e7e7eu64;

pub fn search_onbit_index(bits: u64) -> Vec<u64> {
    let mut indexes: Vec<u64> = Vec::new();
    let mut bitseq = bits;
    let mut counter: u64 = 0;
    while bitseq != 0 {
        if (bitseq & LOWEST_BIT_MASK) != 0 {
            indexes.push(counter)
        }
        counter += 1;
        bitseq >>= 1;
    }
    return indexes;
}

pub fn get_moves(player: u64, opponent: u64) -> u64 {
    let mut m: u64 = get_moves_left(player, opponent, TOP_BOTTOM_MASK, 8);
    m |= get_moves_left(player, opponent, LEFT_RIGHT_MASK, 7);
    m |= get_moves_right(player, opponent, LEFT_RIGHT_MASK, 1);
    m |= get_moves_right(player, opponent, LEFT_RIGHT_MASK, 9);
    m |= get_moves_right(player, opponent, TOP_BOTTOM_MASK, 8);
    m |= get_moves_right(player, opponent, LEFT_RIGHT_MASK, 7);
    m |= get_moves_left(player, opponent, LEFT_RIGHT_MASK, 1);
    return m | get_moves_left(player, opponent, LEFT_RIGHT_MASK, 9);
}

pub fn get_moves_right(player: u64, opponent: u64, mask: u64, offset: u64) -> u64 {
    let e: u64 = opponent & mask;
    let mut m: u64 = (player << offset) & e;
    m |= (m << offset) & e;
    m |= (m << offset) & e;
    m |= (m << offset) & e;
    m |= (m << offset) & e;
    m |= (m << offset) & e;
    return (m << offset) & !(player | opponent);
}

pub fn get_moves_left(player: u64, opponent: u64, mask: u64, offset: u64) -> u64 {
    let e: u64 = opponent & mask;
    let mut m: u64 = (player >> offset) & e;
    m |= (m >> offset) & e;
    m |= (m >> offset) & e;
    m |= (m >> offset) & e;
    m |= (m >> offset) & e;
    m |= (m >> offset) & e;
    return (m >> offset) & !(player | opponent);
}

pub fn get_reverses(player: u64, opponent: u64, _move: u64) -> u64 {
    let mut m = get_reverses_left(player, opponent, _move, TOP_BOTTOM_MASK, 8);
    m |= get_reverses_left(player, opponent, _move, LEFT_RIGHT_MASK, 7);
    m |= get_reverses_right(player, opponent, _move, LEFT_RIGHT_MASK, 1);
    m |= get_reverses_right(player, opponent, _move, LEFT_RIGHT_MASK, 9);
    m |= get_reverses_right(player, opponent, _move, TOP_BOTTOM_MASK, 8);
    m |= get_reverses_right(player, opponent, _move, LEFT_RIGHT_MASK, 7);
    m |= get_reverses_left(player, opponent, _move, LEFT_RIGHT_MASK, 1);
    return m | get_reverses_left(player, opponent, _move, LEFT_RIGHT_MASK, 9);
}

pub fn get_reverses_right(player: u64, opponent: u64, _move: u64, mask: u64, offset: u64) -> u64 {
    let e = opponent & mask;
    let mut m = (_move << offset) & e;
    m |= (m << offset) & e;
    m |= (m << offset) & e;
    m |= (m << offset) & e;
    m |= (m << offset) & e;
    m |= (m << offset) & e;
    let mut o = (player >> offset) & e;
    o |= (o >> offset) & e;
    o |= (o >> offset) & e;
    o |= (o >> offset) & e;
    o |= (o >> offset) & e;
    o |= (o >> offset) & e;
    return m & o;
}

pub fn get_reverses_left(player: u64, opponent: u64, _move: u64, mask: u64, offset: u64) -> u64 {
    let e: u64 = opponent & mask;
    let mut m: u64 = (_move >> offset) & e;
    m |= (m >> offset) & e;
    m |= (m >> offset) & e;
    m |= (m >> offset) & e;
    m |= (m >> offset) & e;
    m |= (m >> offset) & e;
    let mut o: u64 = (player << offset) & e;
    o |= (o << offset) & e;
    o |= (o << offset) & e;
    o |= (o << offset) & e;
    o |= (o << offset) & e;
    o |= (o << offset) & e;
    return m & o;
}

pub fn count_bits(bits: u64) -> u64 {
    let mut count = (bits & 0x5555555555555555) + ((bits >> 1) & 0x5555555555555555);
    count = (count & 0x3333333333333333) + ((count >> 2) & 0x3333333333333333);
    count = (count & 0x0f0f0f0f0f0f0f0f) + ((count >> 4) & 0x0f0f0f0f0f0f0f0f);
    count = (count & 0x00ff00ff00ff00ff) + ((count >> 8) & 0x00ff00ff00ff00ff);
    count = (count & 0x0000ffff0000ffff) + ((count >> 16) & 0x0000ffff0000ffff);
    (count & 0x00000000ffffffff) + ((count >> 32) & 0x00000000ffffffff)
}
