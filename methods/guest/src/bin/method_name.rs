// TODO: Rename this file to change the name of this method from METHOD_NAME

#![no_main]
use risc0_zkvm::guest::env;
risc0_zkvm::entry!(main);
use rbpf::ebpf;
use std::io::{Error, ErrorKind};
pub fn main() {

   let prog = &[
    0xb4, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // mov32 r0, 0
    0xb4, 0x01, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, // mov32 r1, 2
    0x04, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, // add32 r0, 1
    0x0c, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // add32 r0, r1
    0x95, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00  // exit
    ];

    let vm = rbpf::EbpfVmNoData::new(Some(prog)).unwrap();

    assert_eq!(vm.execute_program().unwrap(), 0x3);


    let stack = vec![0u8;ebpf::STACK_SIZE];

        let mut reg: [u64;11] = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, stack.as_ptr() as u64 + stack.len() as u64
        ];

    let vm = rbpf::EbpfVmNoData::new(Some(prog)).unwrap();
    eprintln!("Initializing reg inside the function:");

    let result = test_execute_program(vm, prog);

    eprintln!("Reg as input:");
    let result = test_execute_program2(prog, reg);
}

pub fn test_execute_program2(prog: &[u8], mut reg: [u64;11]) -> Result<u64, Error> {

    let mut insn_ptr:usize = 0;

    while insn_ptr * ebpf::INSN_SIZE < prog.len() {
        let insn = ebpf::get_insn(prog, insn_ptr);
        insn_ptr += 1;
        let _dst = insn.dst as usize;
        eprintln!("{:?}", insn);
        eprintln!("{:?}", reg);
        if (insn.opc == ebpf::EXIT) { return Ok(reg[0]) }
        reg[_dst] = insn.imm  as u32                                as u64;
    }
    unreachable!()
}


pub fn test_execute_program(s: rbpf::EbpfVmNoData, mut prog: &[u8]) -> Result<u64, Error> {

    let stack = vec![0u8;ebpf::STACK_SIZE];

    let mut reg: [u64;11] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, stack.as_ptr() as u64 + stack.len() as u64
    ];

    let mut insn_ptr:usize = 0;
    while insn_ptr * ebpf::INSN_SIZE < prog.len() {
        let insn = ebpf::get_insn(prog, insn_ptr);
        insn_ptr += 1;
        let _dst = insn.dst as usize;
        eprintln!("{:?}", insn);
        eprintln!("{:?}", reg);
        if (insn.opc == ebpf::EXIT) { return Ok(reg[0]) }
        reg[_dst] = insn.imm  as u32                                as u64;
    }
    unreachable!()
}
