extern crate trousers;

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
