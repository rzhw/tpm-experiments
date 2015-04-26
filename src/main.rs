extern crate trousers;

use std::io;
use std::str::FromStr;
use trousers::*;

fn main() {
    // TODO: Any cleaner way to write this?
    let contextresult = TssContext::new();
    if let Ok(context) = contextresult {
        if let Ok(_) = context.connect() {
            if let Ok(tpm) = context.get_tpm_object() {
                println!("I'M IN UR TPM READING UR PCRZ (From Rust!)");
                for i in 0..24 {
                    if let Ok(vec) = tpm.pcr_read(i) {
                        print!("PCR {:02}", i);
                        for j in 0..vec.len() {
                            if j % 4 == 0 {
                                print!(" ");
                            }
                            print!("{:02x}", vec[j]);
                        }
                        print!("\n");
                    }
                }

                println!("");
                let to_extend = get_input::<u32>("Let's extend a PCR! Pick a PCR:");
                if let Ok(new_pcr_value) = tpm.pcr_extend(to_extend, b"abcdefghijklmnopqrst") {
                    println!("Extended the PCR!");
                } else {
                    println!("Failed to extend :(");
                }

                println!("PCRs are now:");
                for i in 0..24 {
                    if let Ok(vec) = tpm.pcr_read(i) {
                        print!("PCR {:02}", i);
                        for j in 0..vec.len() {
                            if j % 4 == 0 {
                                print!(" ");
                            }
                            print!("{:02x}", vec[j]);
                        }
                        print!("\n");
                    }
                }
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
