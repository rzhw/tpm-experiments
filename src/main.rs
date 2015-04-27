extern crate trousers;

use std::io;
use std::str::FromStr;
use trousers::*;

use std::ffi::CString;

type newtComponent = u32;
#[link(name = "newt")]
extern {
    fn newtInit();
    fn newtCls();
    fn newtWaitForKey();
    fn newtDrawRootText(col: i32, row: i32, text: *const i8);
    fn newtFinished();
    fn newtCenteredWindow(width: i32, height: i32, text: *const i8) -> i32;
    fn newtForm(vertBar: newtComponent, help: *const i8, flags: i32) -> newtComponent;
    fn newtFormAddComponent(form: newtComponent, co: newtComponent);
    fn newtFormDestroy(form: newtComponent);
    fn newtRunForm(form: newtComponent) -> newtComponent;
    fn newtButton(left: i32, top: i32, text: *const i8) -> newtComponent;
    fn newtLabel(left: i32, top: i32, text: *const i8) -> newtComponent;
}

fn main() {
    unsafe {
        newtInit();
        newtCls();
        newtDrawRootText(0, 0, CString::new("Some root text").unwrap().as_ptr());
        newtCenteredWindow(80, 30, CString::new("View PCRs").unwrap().as_ptr());
    }
    // TODO: Any cleaner way to write this?
    let contextresult = TssContext::new();
    if let Ok(context) = contextresult {
        if let Ok(_) = context.connect() {
            if let Ok(tpm) = context.get_tpm_object() {
                unsafe {
                    newtLabel(4, 4, CString::new("I'M IN UR TPM READING UR PCRZ (From Rust!)").unwrap().as_ptr());
                }
                for i in 0..24 {
                    let mut s = String::new();
                    if let Ok(vec) = tpm.pcr_read(i) {
                        s.push_str(std::str::from_utf8(format!("PCR {:02}", i).as_bytes()).unwrap());
                        for j in 0..vec.len() {
                            if j % 4 == 0 {
                                s.push_str(" ");
                            }
                            s.push_str(std::str::from_utf8(format!("{:02x}", vec[j]).as_bytes()).unwrap());
                        }
                        s.push_str("\n");
                        unsafe {
                            let form = newtForm(0, std::ptr::null(), 0);
                            let label = newtLabel(4, 4+(i as i32), CString::new(s).unwrap().as_ptr());
                            newtFormAddComponent(form, label);
                            let button = newtButton(79, 0, CString::new("OK").unwrap().as_ptr());
                            newtFormAddComponent(form, button);
                            newtRunForm(form);
                            newtFormDestroy(form);
                        }
                    }
                }
/*
                println!("Let's extend a PCR!");
                let to_extend = get_input::<u32>("Pick a PCR:");
                if let Ok(new_pcr_value) = tpm.pcr_extend(to_extend, b"abcdefghijklmnopqrst") {
                    println!("Extended the PCR! New PCR value:");
                    print!("PCR {:02}", to_extend);
                    for j in 0..new_pcr_value.len() {
                        if j % 4 == 0 {
                            print!(" ");
                        }
                        print!("{:02x}", new_pcr_value[j]);
                    }
                    print!("\n");
                } else {
                    println!("Failed to extend :(");
                }

                println!("Now let's reset a PCR!");
                let to_reset = get_input::<u32>("Pick a PCR:");
                if let Ok(pcrs) = context.create_pcr_composite_info_long() {
                    if let Ok(_) = pcrs.select_pcr_index_ex(to_reset, 1) {
                        if let Ok(_) = tpm.pcr_reset(&pcrs) {
                            println!("Reset the PCR! New PCR value:");
                            if let Ok(vec) = tpm.pcr_read(to_reset) {
                                print!("PCR {:02}", to_reset);
                                for j in 0..vec.len() {
                                    if j % 4 == 0 {
                                        print!(" ");
                                    }
                                    print!("{:02x}", vec[j]);
                                }
                            } else {
                                println!("Failed to read it!");
                            }
                        } else {
                            println!("Failed to reset!");
                        }
                    } else {
                        println!("Failed to select index!");
                    }
                } else {
                    println!("Failed to create PCR composite info object!");
                }
                */
            } else {
                println!("Failed to get TPM handle :(")
            }
        } else {
            println!("Failed to connect :(");
        }
    } else {
        println!("Failed :(");
    }

    /*
    match blah {
        Ok(context) => println!("Context created! {:?}", context.handle),
        Err(e) => println!("Context failed with err {:?}", e),
    }
    println!("Hello world!");
    */
    unsafe {
        newtWaitForKey();
        newtFinished();
    }
}

fn get_input<A: FromStr>(message: &str) -> A {
    get_input_custom_errmsg(message, "Try again")
}
fn get_input_custom_errmsg<A: FromStr>(message: &str, err_message: &str) -> A {
    println!("{}", message);
    let mut line = String::new();
    match io::stdin().read_line(&mut line) {
        Ok(_) =>
            match line.trim().parse() {
            Ok(x) => x,
            // TODO: Rust doesn't optimize tail calls?
            Err(_) => {
                println!("{}", err_message);
                get_input(message)
            }
        },
        Err(_) => {
            println!("{}", err_message);
            get_input(message)
        }
    }
}
