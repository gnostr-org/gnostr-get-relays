extern crate libc;

extern {
    pub fn gnostr_get_relays(input: libc::c_int) -> libc::c_int;
}
extern {
    pub fn get_list() -> libc::c_char;
}

pub fn get_relays(input: i32) {
    let _output = unsafe { gnostr_get_relays(input) };
	let _result = input*2;
    //println!("{} * 2 = {}", input, _result);
}
