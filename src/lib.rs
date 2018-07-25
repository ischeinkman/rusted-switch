#![feature(start, lang_items, const_fn, panic_implementation)]
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![crate_type = "staticlib"]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[no_mangle]
#[start]
pub unsafe extern fn main(_argc : isize, _argv : * const * const u8) ->isize {
    printf("\x1b[16;16HPress PLUS to exit.".as_ptr() as *const i8);

    gfxInitDefault();
    consoleInit(_NULL as *mut PrintConsole);
    while appletMainLoop() {
        hidScanInput();

        let kDown = HidControllerKeys(hidKeysDown(HidControllerID::CONTROLLER_P1_AUTO) as u32);

        if kDown == HidControllerKeys::KEY_PLUS {
            break;
        }

        printf("This key is pressed: %d\n".as_ptr() as *const i8, kDown);

        gfxFlushBuffers();
        gfxSwapBuffers();
        gfxWaitForVsync();
    }
	gfxExit();
    0
}

pub mod lang_items;
