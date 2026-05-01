use crate::uart::*;
use core::fmt::*;

// Console output.
// Output is written to the screen and serial port.
// fn printint(xx: i32, base: u32, sign: bool) {
//   let digits = "0123456789abcdef";
//   let mut buf = Vec::with_capacity(16);
//   let mut x: u32;

//   if sign && xx < 0 {
//     x = -xx as u32;
//   } else {
//     x = xx as u32;
//   }

//   loop {
//     buf.push(digits.chars().nth((x % base) as usize).unwrap());
//     x /= base;
//     if x == 0 {
//       break;
//     }
//   }

//   if sign && xx < 0 {
//     buf.push('-');
//   }

//   // Print in reverse order (since we added digits from least to most significant)
//   for &c in buf.iter().rev() {
//     consputc(c);
//   }
// }

// pub fn cprintf(fmt: &str, args: &[u32]) {
//   let mut argp = args.iter();
//   let mut i = 0;
//   let mut fmt_chars = fmt.chars();

//   while let Some(c) = fmt_chars.next() {
//     if c != '%' {
//       consputc(c);
//       continue;
//     }

//     match fmt_chars.next() {
//       Some('d') => {
//         if let Some(&val) = argp.next() {
//           printint(val as i32, 10, true);
//         }
//       }
//       Some('x') | Some('p') => {
//         if let Some(&val) = argp.next() {
//           printint(val as i32, 16, false);
//         }
//       }
//       Some('s') => {
//         if let Some(&val) = argp.next() {
//           let s = unsafe { std::ffi::CStr::from_ptr(val as *const i8) };
//           let s = s.to_str().unwrap_or("(null)");
//           for c in s.chars() {
//             consputc(c);
//           }
//         }
//       }
//       Some('%') => {
//         consputc('%');
//       }
//       Some(_) => {
//         // Print unknown % sequence
//         consputc('%');
//         consputc(c);
//       }
//       None => break,
//     }
//   }
// }

pub struct Console {}
impl Write for Console {
    fn write_str(&mut self, s: &str) -> Result {
         for c in s.chars() {
            consputc(c);
        }
        Ok(())
    }
}


#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => ({
        use core::fmt::*;
        use crate::console::Console;
        let mut c = Console {};
        let _ = writeln!(&mut c, $($arg)*);
    });
}

// #[macro_export]
// macro_rules! println {
//     ($($arg:tt)*) => ({
//         let _ = cprintf($($arg)*);
//     });
// }

const BACKSPACE: char = '\x08';

fn consputc(c: char) {
  if c == BACKSPACE {
    uartputc(BACKSPACE);
    uartputc(' ');
    uartputc(BACKSPACE);
  } else {
    uartputc(c);
  }
}
