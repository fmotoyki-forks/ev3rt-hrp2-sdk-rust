#![feature(lang_items, asm, core_intrinsics)]
#![no_std]

#[no_mangle]
pub extern "C" fn twice(val: u32) -> u32 {
	let u = val + 2;
	u
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[lang = "panic_fmt"]
extern "C" fn panic_fmt() {}