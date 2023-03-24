use std::{fs, process, env, io::Write};

fn main() {
    let args: Vec<String> = env::args().collect();

    let program_name = &args[0];

    if args.len() < 3 {
        eprintln!("insufficient number of arguments provided");
        eprintln!("Usage: {program_name} <input.png> <output.png>");
        process::exit(1);
    }

    let input_file_path = &args[1];
    let output_file_path = &args[2];

    let input_file_bytes = fs::read(input_file_path).unwrap_or_else(|err| {
        eprintln!("could not read file: {err}");
        process::exit(1);
    });

    let mut output_file = fs::File::create(output_file_path).unwrap_or_else(|err| {
        eprintln!("could not create output file: {err}");
        process::exit(1);
    });

    const PNG_SIG_CAP: usize = 8;
    let png_sig: [u8; PNG_SIG_CAP] = [137, 80, 78, 71, 13, 10, 26, 10];

    let mut cursor = 0;
    let mut sig = &input_file_bytes[cursor..cursor+PNG_SIG_CAP];
    cursor += PNG_SIG_CAP;

    if sig != png_sig {
        eprintln!("{input_file_path} is not a valid PNG file.\n");
        process::exit(1);
    }

    output_file.write(&mut sig).unwrap_or_else(|err| {
        eprintln!("could not write to the output file: {err}");
        process::exit(1);
    });

    let iend_chunk_type: [u8; 4] = [73, 69, 78, 68];
    let idata_chunk_type: [u8; 4] = [73, 68, 65, 84];

    let mut running = true;
    while running {

        let mut chunk_length = &input_file_bytes[cursor..cursor+4];
        cursor += 4;

        let chunk_length_arr: [u8; 4] = chunk_length.try_into().unwrap();

        let chunk_length_int: usize = u32::from_be_bytes(chunk_length_arr).try_into().unwrap();

        output_file.write(&mut chunk_length).unwrap_or_else(|err| {
            eprintln!("could not write to the output file: {err}");
            process::exit(1);
        });

        let mut chunk_type = &input_file_bytes[cursor..cursor+4];
        cursor += 4;

        if chunk_type == iend_chunk_type {
            running = false;
        }

        output_file.write(&mut chunk_type).unwrap_or_else(|err| {
            eprintln!("could not write to the output file: {err}");
            process::exit(1);
        });

        let mut chunk_data = &input_file_bytes[cursor..cursor+chunk_length_int];
        cursor += chunk_length_int;

        output_file.write(&mut chunk_data).unwrap_or_else(|err| {
            eprintln!("could not write to the output file: {err}");
            process::exit(1);
        });

        let mut crc = &input_file_bytes[cursor..cursor+4];
        cursor += 4;

        output_file.write(&mut crc).unwrap_or_else(|err| {
            eprintln!("could not write to the output file: {err}");
            process::exit(1);
        });

        if chunk_type == idata_chunk_type {
            let size: u32 = 3;
            let bytes: &[u8; 4] = &size.to_le_bytes();
            let mut injected_length_buf: &[u8] = bytes.as_ref();

            output_file.write(&mut injected_length_buf).unwrap_or_else(|err| {
                eprintln!("could not write to the output file: {err}");
                process::exit(1);
            });

            let injected_type = "coCK";
            let mut injected_type_buf = injected_type.as_bytes();

            output_file.write(&mut injected_type_buf).unwrap_or_else(|err| {
                eprintln!("could not write to the output file: {err}");
                process::exit(1);
            });

            let injected_data = "YEP";
            let mut injected_data_buf = injected_data.as_bytes();

            output_file.write(&mut injected_data_buf).unwrap_or_else(|err| {
                eprintln!("could not write to the output file: {err}");
                process::exit(1);
            });

            let mut injected_crc: &[u8] = &[0, 0, 0, 0];
            
            output_file.write(&mut injected_crc).unwrap_or_else(|err| {
                eprintln!("could not write to the output file: {err}");
                process::exit(1);
            });
        }

    }
}
