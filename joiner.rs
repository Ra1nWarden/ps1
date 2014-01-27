use std::str;
use std::os;
use std::io::File;

fn main() {
   let args : ~[~str] = os::args();
   if args.len() != 3 {
      println!("Usage: {:s} <inputfile.share1> <inputfile.share2>", args[0]);
   }
   else {
   	let share1 = args[1].clone();
	let share2 = args[2].clone();
	let path1 = Path::new(share1.clone());
	let path2 = Path::new(share2.clone());
	let mut share1_file = File::open(&path1);
	let mut share2_file = File::open(&path2);
	let share1_byte: ~[u8] = share1_file.read_to_end();
	let share2_byte: ~[u8] = share2_file.read_to_end();
        let decoded_msg = join(share1_byte, share2_byte);
	let decoded_str = str::from_utf8(decoded_msg);
	print(decoded_str);
	
   }

}

fn xor(a: &[u8], b: &[u8]) -> ~[u8] {
   let mut ret = ~[];
   for i in range(0, a.len()) {
       ret.push(a[i] ^ b[i]);
   }
   ret
}

fn join(share1_file: &[u8], share2_file: &[u8]) -> ~[u8] {
    let decoded_byte = xor(share1_file, share2_file);
    decoded_byte
}