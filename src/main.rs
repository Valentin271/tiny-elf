#[cfg(feature = "asm")]
fn main() -> std::io::Result<()> {
    use std::{env, fs::File, io::prelude::Write};

    use tiny_elf::{
        asm::{AsAsm, Program},
        prelude::*,
        program_header::Flags,
    };

    let word = env::args()
        .nth(1)
        .unwrap_or("Hello World, this is my tiny executable\n".into());
    let word_len = word.len() as u32;

    let program = {
        use tiny_elf::asm::{Memory, Mnemonic::*, Register::*};

        Program::default()
            .add(Push(6u8.into()))
            .add(Cmp(Rsp, 5))
            .add(Jmp("print".into()))
            .label("read")
            .add(Mov(Rax, 0u32.into()))
            .add(Mov(Rdi, 1u32.into()))
            .add(Mov(Rsi, Memory::from("msg").into()))
            .add(Mov(Rdx, word_len.into()))
            .add(Syscall)
            .label("print")
            .add(Mov(Rax, 1u32.into()))
            .add(Mov(Rdi, 1u32.into()))
            .add(Mov(Rsi, Memory::from("msg").into()))
            .add(Mov(Rdx, word_len.into()))
            .add(Syscall)
            .label("printn")
            .add(Mov(Rax, 1u32.into()))
            .add(Mov(Rdi, 1u32.into()))
            .add(Push(53u8.into()))
            .add(Mov(Rsi, Rsp.into()))
            .add(Mov(Rdx, 1u32.into()))
            .add(Syscall)
            .label("stop")
            .add(Mov(Rax, 60u32.into()))
            .add(Mov(Rdi, 0u32.into()))
            .add(Syscall)
    };
    let program = program.insert_data("msg", &word);

    let mut elf = Elf::new(program.clone());
    elf.add_data(program.data(), Flags::all());
    elf.backpatch();

    {
        let mut file = File::create("dump.asm").unwrap();
        file.write_all(program.as_asm().as_bytes())?;
    }

    {
        let mut file = File::create(env::args().nth(2).unwrap_or("bin".into()))?;
        file.write_all(&elf.as_bytes())?;
    }

    Ok(())
}

#[cfg(not(feature = "asm"))]
fn main() -> std::process::ExitCode {
    use std::process::ExitCode;

    eprintln!("Enable 'asm' feature to run");

    ExitCode::FAILURE
}
