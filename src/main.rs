extern crate trousers;
extern crate newt;

use std::io;
use std::str::FromStr;
use trousers::*;
use std::ffi::CString;

fn read_pcr_as_str(tpm: &TssTPM, pcr_index: u32) -> Result<String, TssResult> {
    let mut s = String::new();
    let vec = try!(tpm.pcr_read(pcr_index));
    s.push_str(std::str::from_utf8(format!("PCR {:02}", pcr_index).as_bytes()).unwrap());
    for j in 0..vec.len() {
        if j % 4 == 0 {
            s.push_str(" ");
        }
        s.push_str(std::str::from_utf8(format!("{:02x}", vec[j]).as_bytes()).unwrap());
    }
    s.push_str("\n");
    Ok(s)
}

fn show_view_pcrs(tpm: &TssTPM) -> Result<(), TssResult> {
    newt::centered_window(80, 30, "View PCRs");
    let form = newt::form(None, None, 0);
    for i in 0..24 {
        let pcr_str = try!(read_pcr_as_str(tpm, i));
        let pcr_label = newt::label(1, 1+(i as i32), &*pcr_str);
        newt::form_add_component(form, pcr_label);
    }
    let button = newt::button(1, 26, "OK");
    newt::form_add_component(form, button);
    newt::run_form(form);
    newt::form_destroy(form);
    Ok(())
}

fn show_extend_pcr(tpm: &TssTPM) -> Result<(), TssResult> {
    loop {
        newt::centered_window(80, 30, "Extend PCR");
        let form = newt::form(None, None, 0);
        let entry = newt::entry(1, 1, None, 10, 0);
        newt::form_add_component(form, entry);
        let ok_button = newt::button(1, 26, "OK");
        let cancel_button = newt::button(10, 26, "Cancel");
        newt::form_add_component(form, ok_button);
        newt::form_add_component(form, cancel_button);

        if newt::run_form(form) == ok_button {
            let string = newt::entry_get_value(entry);
            let parsed = string.parse::<u32>();

            if let Ok(to_extend) = parsed {
                let old_pcr_str = try!(read_pcr_as_str(tpm, to_extend));
                try!(tpm.pcr_extend(to_extend, b"abcdefghijklmnopqrst"));
                let new_pcr_str = try!(read_pcr_as_str(tpm, to_extend));
                newt::centered_window(60, 10, "Success");
                let form = newt::form(None, None, 0);
                let old_label = newt::label(0, 0, "Old value:");
                let old_text = newt::label(0, 1, &*old_pcr_str);
                let new_label = newt::label(0, 2, "New value:");
                let new_text = newt::label(0, 3, &*new_pcr_str);
                let button = newt::button(1, 5, "OK");
                newt::form_add_component(form, old_label);
                newt::form_add_component(form, old_text);
                newt::form_add_component(form, new_label);
                newt::form_add_component(form, new_text);
                newt::form_add_component(form, button);
                newt::run_form(form);
                newt::form_destroy(form);
                return Ok(());
            } else {
                show_error("Not an integer");
            }
        } else {
            newt::form_destroy(form);
            return Ok(());
        }
    }
}

fn show_reset_pcr(tpm: &TssTPM) -> Result<(), TssResult> {
    loop {
        newt::centered_window(80, 30, "Reset PCR");
        let form = newt::form(None, None, 0);
        let entry = newt::entry(1, 1, None, 10, 0);
        newt::form_add_component(form, entry);
        let ok_button = newt::button(1, 26, "OK");
        let cancel_button = newt::button(10, 26, "Cancel");
        newt::form_add_component(form, ok_button);
        newt::form_add_component(form, cancel_button);

        if newt::run_form(form) == ok_button {
            let string = newt::entry_get_value(entry);
            let parsed = string.parse::<u32>();

            if let Ok(to_reset) = parsed {
                let old_pcr_str = try!(read_pcr_as_str(tpm, to_reset));
                let pcrs = try!(tpm.context.create_pcr_composite_info_long());
                try!(pcrs.select_pcr_index_ex(to_reset, 1));
                try!(tpm.pcr_reset(&pcrs));
                let new_pcr_str = try!(read_pcr_as_str(tpm, to_reset));
                newt::centered_window(60, 10, "Success");
                let form = newt::form(None, None, 0);
                let old_label = newt::label(0, 0, "Old value:");
                let old_text = newt::label(0, 1, &*old_pcr_str);
                let new_label = newt::label(0, 2, "New value:");
                let new_text = newt::label(0, 3, &*new_pcr_str);
                let button = newt::button(1, 5, "OK");
                newt::form_add_component(form, old_label);
                newt::form_add_component(form, old_text);
                newt::form_add_component(form, new_label);
                newt::form_add_component(form, new_text);
                newt::form_add_component(form, button);
                newt::run_form(form);
                newt::form_destroy(form);
                return Ok(());
            } else {
                show_error("Not an integer");
            }
        } else {
            newt::form_destroy(form);
            return Ok(());
        }
    }
}

fn show_error(message: &str) {
    show_message("Error", message);
}

fn show_message(title: &str, message: &str) {
    newt::centered_window(60, 10, title);
    let form = newt::form(None, None, 0);
    let label = newt::label(0, 0, message);
    newt::form_add_component(form, label);
    let button = newt::button(1, 5, "OK");
    newt::form_add_component(form, button);
    newt::run_form(form);
    newt::form_destroy(form);
}

fn show_menu(tpm: &TssTPM) {
    loop {
        newt::centered_window(80, 30, "Menu");
        let form = newt::form(None, None, 0);
        let listbox = newt::listbox(0, 0, 30, newt::NEWT_FLAG_RETURNEXIT);
        newt::listbox_append_entry(listbox, "View PCRs", 0);
        newt::listbox_append_entry(listbox, "Extend PCR", 1);
        newt::listbox_append_entry(listbox, "Reset PCR", 2);
        newt::listbox_append_entry(listbox, "Exit", 3);
        newt::form_add_component(form, listbox);
        newt::run_form(form);
        newt::form_destroy(form);
        let result = newt::listbox_get_current(listbox);
        if result == 0 {
            show_view_pcrs(tpm);
        } else if result == 1 {
            show_extend_pcr(tpm);
        } else if result == 2 {
            show_reset_pcr(tpm);
        } else if result == 3 {
            return;
        }
    }
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
                show_menu(&tpm);
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
