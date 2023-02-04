use std::fs::File;
use std::io::{self, Write};

fn write_file(staff: (String, String, u8)) {
    let (name, department, code) = staff;
    let services = match code {
        7 => "Strategy consulting, Analytics consulting, Change management and experience",
        8 => "Corporate finance, Tax planning, Audit services",
        9 => "Corporate and growth strategy, Customer experience, HR transformation",
        _ => "Unknown"
    };

    
    let file_name = format!("{}_{}.txt", name.to_lowercase(), department.to_lowercase());

   
    let mut file = File::create(file_name).expect("Unable to create file");
    writeln!(file, "Name: {}", name).expect("Unable to write to file");
    writeln!(file, "Department: {}", department).expect("Unable to write to file");
    writeln!(file, "Services: {}", services).expect("Unable to write to file");
}

fn main() {
    let staff = [
        ("Aigbona Juliet", "Consulting", 7),
        ("Ehis Ero", "Strategy", 9),
        ("Adamu Sagamu", "Tax", 8),
        ("Akpevwe Iloka", "Assurance", 7),
        ("Maria Akinsola", "Transactions and corporate finance", 9),
        ("Gbenga Daniels", "People and workforce", 8)
    ];

    for staff_member in staff.iter() {
        write_file(*staff_member);
    }
}

