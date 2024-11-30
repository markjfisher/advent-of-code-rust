use crate::util::parse::*;

pub struct Room<'a> {
    name: &'a str,
    sector_id: u32
}

pub fn parse(input: &str) -> Vec<Room<'_>> {
    let mut valid = Vec::new();
    let to_index = |b: u8| (b - b'a') as usize;

    // the room is everything before the last 11 characters, as the sector id and checksum in brackets
    'outer: for line in input.lines() {
        let llen = line.len();
        let name = &line[..llen - 11];
        let sector_id = (&line[llen - 10..llen - 7]).unsigned();
        let checksum = line[llen - 6..llen - 1].as_bytes();

        let mut freq = [0; 26];
        let mut fof = [0; 64];
        let mut highest = 0;

        for b in name.bytes() {
            if b != b'-' {
                let index = to_index(b);
                let current = freq[index];
                let next = freq[index] + 1;

                freq[index] = next;
                fof[current] -= 1;
                fof[next] += 1;

                highest = highest.max(next);
            }
        }

        // Initial check.
        if freq[to_index(checksum[0])] != highest {
            continue;
        }

        // Check each pair making sure that the frequency is non-increasing and that there are
        // no letters in between (`fof` should be zero for all intervening letters).
        // If the frequency is equal then also make sure letters are in alphabetical order.
        for w in checksum.windows(2) {
            let end = freq[to_index(w[0])];
            let start = freq[to_index(w[1])];

            if start > end || (start == end && w[1] <= w[0]) {
                continue 'outer;
            }

            if (start + 1..end).any(|i| fof[i] != 0) {
                continue 'outer;
            }
        }

        valid.push(Room { name, sector_id });
    };


    valid
}

pub fn part1(input: &[Room<'_>]) -> u32 {
    input.iter().map(|room| room.sector_id).sum()
}

pub fn part2(input: &[Room<'_>]) -> u32 {
    // need to find the room with name "northpole object storage", so length 24, with hyphen at 9 and 16
    for &Room { name, sector_id } in input {
        let bytes = name.as_bytes();
        if bytes.len() == 24 && bytes[9] == b'-' && bytes[16] == b'-' {
            let mut buffer = String::with_capacity(24);
            for b in name.bytes() {
                if b == b'-' {
                    buffer.push(' ');
                } else {
                    let rotate = (sector_id % 26) as u8;
                    let decrypted = (b - b'a' + rotate) % 26 + b'a';    
                    buffer.push(decrypted as char);
                }
            }

            if buffer == "northpole object storage" {
                return sector_id;
            }
        }
    }
    unreachable!()
}