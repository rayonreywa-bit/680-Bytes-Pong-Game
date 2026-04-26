#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::mem::MaybeUninit;

#[link(name = "user32")]
#[link(name = "kernel32")]
unsafe extern "system" {
    fn GetStdHandle(n: i32) -> isize;
    fn WriteConsoleA(h: isize, b: *const u8, l: u32, w: *mut u32, r: *mut u8) -> i32;
    fn GetAsyncKeyState(v: i32) -> i16;
    fn Sleep(m: u32);
    fn ExitProcess(c: u32) -> !;
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn mainCRTStartup() -> ! {
    let stdout = GetStdHandle(-11);
    let (mut p1y, mut p2y) = (8, 8);
    let (mut bx, mut by) = (20, 10);
    let (mut dx, mut dy) = (1, 1);

    let mut buf = MaybeUninit::<[u8; 823]>::uninit();
    let ptr = buf.as_mut_ptr() as *mut u8;

    *ptr.add(0) = 27; 
    *ptr.add(1) = b'['; 
    *ptr.add(2) = b'H';

    loop {
        // 1. الإدخال
        if GetAsyncKeyState(0x57) < 0 && p1y > 1 { p1y -= 1; }
        if GetAsyncKeyState(0x53) < 0 && p1y < 15 { p1y += 1; }

        // 2. الذكاء الاصطناعي
        if by > p2y + 1 && p2y < 15 { p2y += 1; }
        else if by < p2y + 1 && p2y > 1 { p2y -= 1; }

        // 3. الفيزياء والاصطدام
        bx += dx; by += dy;
        if by == 1 || by == 18 { dy = -dy; }
        
        if (bx == 2 && ((by - p1y) as u32) < 4) || 
           (bx == 37 && ((by - p2y) as u32) < 4) {
            dx = -dx;
        }

        if bx <= 0 || bx >= 39 { bx = 20; dx = -dx; }

        // 4. الرسم المباشر
        let mut i = 3;
        for y in 0..20 {
            for x in 0..40 {
                *ptr.add(i) = if y == 0 || y == 19 { b'-' }
                else if x == 0 || x == 39 { b'|' }
                else if x == bx && y == by { b'O' }
                else if x == 2 && ((y - p1y) as u32) < 4 { b'#' }
                else if x == 37 && ((y - p2y) as u32) < 4 { b'#' }
                else { b' ' };
                i += 1;
            }
            *ptr.add(i) = b'\n';
            i += 1;
        }

        // تم الإصلاح هنا: تمرير null بدلاً من dx يمنع تخريب سرعة الكرة
        WriteConsoleA(stdout, ptr, 823, core::ptr::null_mut(), core::ptr::null_mut());
        Sleep(30);
        if GetAsyncKeyState(0x1B) < 0 { ExitProcess(0); }
    }
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! { unsafe { ExitProcess(1) } }