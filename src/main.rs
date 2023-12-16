use sha3::{Digest, Keccak256};

// $ cast sig "increaseAllowance(address,uint256)"
// 0x39509351
// $ cast sig "decreaseAllowance(address,uint256)"
// 0xa457c2d7

fn main() {
    let mut num = 0usize;

    let mut inc = [0; 4];
    hex::decode_to_slice("39509351", &mut inc).unwrap();
    let mut dec = [0; 4];
    hex::decode_to_slice("a457c2d7", &mut dec).unwrap();

    loop {
        let cur_word = format!("pwn_{num}()");
        let mut hasher = Keccak256::new();
        hasher.update(cur_word.as_bytes());
        let result = hasher.finalize();
        let sel = result.get(0..4).unwrap();
        if sel == inc || sel == dec {
            println!("{cur_word}");
            break;
        }
        num += 1;
    }
}
