extern crate libc;

extern {
    fn gnostr_get_relays(input: libc::c_int) -> libc::c_int;
}

fn main() {
    let input = 128;
    let _output = unsafe { gnostr_get_relays(input) };
    //println!("{} * 2 = {}", input, output);
}
