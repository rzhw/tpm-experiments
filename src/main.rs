extern crate trousers;
extern crate newt;

use std::io;
use std::str::FromStr;
use trousers::*;
use std::ffi::CString;

fn view_pcrs(tpm: &TssTPM) -> Result<(), TssResult> {
    newt::centered_window(80, 30, "View PCRs");
    let form = newt::Form::new(None, None, 0);
    for i in 0..24 {
        let mut s = String::new();
        let vec = try!(tpm.pcr_read(i));
        s.push_str(std::str::from_utf8(format!("PCR {:02}", i).as_bytes()).unwrap());
        for j in 0..vec.len() {
            if j % 4 == 0 {
                s.push_str(" ");
            }
            s.push_str(std::str::from_utf8(format!("{:02x}", vec[j]).as_bytes()).unwrap());
        }
        s.push_str("\n");
        let pcr_label = newt::Label::new(1, 1+(i as i32), &*s);
        form.add_component(&pcr_label);
    }
    let button = newt::Button::new(1, 26, "OK");
    form.add_component(&button);
    form.run();
    Ok(())
}

fn extend_pcr(tpm: &TssTPM) -> Result<(), TssResult> {
    newt::centered_window(80, 30, "Extend PCR");
    let form = newt::Form::new(None, None, 0);
    let entry = newt::Entry::new(1, 1, None, 10, 0);
    form.add_component(&entry);
    let button = newt::Button::new(1, 26, "OK");
    form.add_component(&button);
    form.run();
    let string = entry.get_value();

    show_error(&*string);

    Ok(())
}

fn show_error(error: &str) {
    newt::centered_window(60, 10, "Error");
    let form = newt::Form::new(None, None, 0);
    let label = newt::Label::new(0, 0, error);
    form.add_component(&label);
    let button = newt::Button::new(1, 5, "OK");
    form.add_component(&button);
    form.run();
}

fn main() {
    newt::init();
    newt::cls();
    newt::draw_root_text(0, 0, "Some root text");
    newt::centered_window(80, 30, "View PCRs");
    // TODO: Any cleaner way to write this?
    let contextresult = TssContext::new();
    if let Ok(context) = contextresult {
        if let Ok(_) = context.connect() {
            if let Ok(tpm) = context.get_tpm_object() {
                view_pcrs(&tpm);
                extend_pcr(&tpm);
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

    newt::finished();
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
