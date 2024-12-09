use aoclib::{input, output};

type FileID = usize;
type BlockSize = usize;

fn main() {
    let input = input("input");

    let dense: Vec<BlockSize> = input
        .chars()
        .flat_map(|x| x.to_string().parse::<BlockSize>())
        .collect();
    let blocks = expand(&dense);

    p1(&blocks);
    p2(&dense);
}

fn expand(dense: &[BlockSize]) -> Vec<Option<FileID>> {
    let mut v = Vec::new();
    let mut file_id = 0;
    for (i, num) in dense.iter().enumerate() {
        if i % 2 == 0 {
            // file
            v.extend_from_slice(&vec![Some(file_id); *num]);
            file_id += 1;
        } else {
            // free
            v.extend_from_slice(&vec![None; *num]);
        }
    }

    v
}

#[inline]
fn p1(blocks: &[Option<FileID>]) {
    let mut r = blocks.len() - 1;
    let mut l = 0usize;

    let mut chsum = 0;

    'outer: while l <= r {
        if let Some(file_id) = blocks[l] {
            chsum += l * file_id;
        } else {
            // try from right
            while blocks[r].is_none() {
                r -= 1;
                if l >= r {
                    break 'outer;
                }
            }
            let file_id = blocks[r].unwrap();
            chsum += l * file_id;
            r -= 1;
        }
        l += 1;
    }

    output(chsum);
}

#[derive(Debug)]
enum MemType {
    File(FileID),
    Free,
}

#[derive(Debug)]
struct MemNode {
    pub start: usize,
    pub t: MemType,
    pub size: BlockSize,
}

impl MemNode {
    fn chsum(&self) -> usize {
        if let MemType::File(file_id) = self.t {
            (self.start..self.start + self.size).fold(0, |acc, x| acc + x * file_id)
        } else {
            0
        }
    }

    #[inline]
    fn is_free(&self) -> bool {
        self.file_id().is_none()
    }

    #[inline]
    fn is_file(&self) -> bool {
        self.file_id().is_some()
    }

    fn file_id(&self) -> Option<FileID> {
        match self.t {
            MemType::Free => None,
            MemType::File(id) => Some(id),
        }
    }
}

#[inline]
fn p2(dense: &[BlockSize]) {
    let mut start = 0;
    let mut file_id = 0;
    let mut memory: Vec<MemNode> = dense
        .iter()
        .enumerate()
        .map(|(i, &size)| {
            let node = if i % 2 == 0 {
                // file
                let node = MemNode {
                    start,
                    t: MemType::File(file_id),
                    size,
                };
                file_id += 1;
                node
            } else {
                // free
                MemNode {
                    start,
                    t: MemType::Free,
                    size,
                }
            };
            start += size;
            node
        })
        .filter(|x| x.size > 0)
        .collect();

    // we now want to move files into free memory blocks from right to left
    // `memory` is already ordered left to right

    let mut l = 0usize;
    let mut r = memory.len() - 1;

    'outer: while r > l {
        if memory[r].is_file() {
            // look for a free block
            while memory[l].is_file() || memory[l].size < memory[r].size {
                if l >= r {
                    l = 0;
                    r -= 1;
                    continue 'outer;
                }
                l += 1;
            }
            // found a free block
            memory.swap(l, r);
            let tmp = memory[l].start;
            memory[l].start = memory[r].start;
            memory[r].start = tmp;

            // we might need to insert a smaller free memory block !!swapped indeces!!
            let block_diff = memory[r].size - memory[l].size;
            if block_diff > 0 {
                memory.insert(
                    l + 1,
                    MemNode {
                        start: memory[l].start + memory[l].size,
                        t: MemType::Free,
                        size: block_diff,
                    },
                );
                r += 1;
            }

            // we might need to merge free blocks forwards and backwards
            if r + 1 < memory.len() && memory[r + 1].is_free() {
                // merge r and r+1
                memory[r].size += memory[r + 1].size;
                memory.remove(r + 1);
            }
            // we can always check backwards
            if memory[r - 1].is_free() {
                // merge r-1 and r
                memory[r - 1].size += memory[r].size;
                memory.remove(r);
            }
            l = 0;
        }
        r -= 1;
    }

    output(memory.iter().map(|node| node.chsum()).sum::<usize>());
}
