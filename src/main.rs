use std::path::PathBuf;

fn main() {
    let filename = "src/bpf_test.o";

    let path = PathBuf::from(filename);
    let file = match elf::File::open_path(path) {
        Ok(f) => f,
        Err(e) => panic!("Error: {:?}", e),
    };

    let text_scn = match file.get_section(".text") {
        Some(s) => s,
        None => panic!("Failed to look up .classifier section"),
    };
    let data_section = file.get_section(".data").map(|x|  {
        let s = &x.data;
        s.as_slice()
    });

    let prog = &text_scn.data;

    let mut vm = rbpf::EbpfVmFixedMbuff::new(
        Some(prog),
        0x40,
        0x50,
        data_section
    ).unwrap();

    let mem = &mut [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    let res = vm.execute_program(mem).unwrap();

    println!(" program returned: {:?}", res);
}
