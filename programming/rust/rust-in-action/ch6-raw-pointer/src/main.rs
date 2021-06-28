/**
 * The code here results into segmentation faults.
 * We are trying to access memory addresses which are not accessible to the program.
 * The error is generated when the program requests the OS to read memory addresses.
 *
 * We can try to read the byte at 0th address, but that would be de-referencing a null ptr.
 */
fn main() {
    let mut n_nonzero = 0;

    for i in 1..10000 {
        let ptr = i as *const u8;
        let byte_at_address = unsafe { *ptr };

        if byte_at_address != 0 {
            n_nonzero += 1;
        }
    }

    println!("non-zero bytes in memory: {}", n_nonzero);
}
