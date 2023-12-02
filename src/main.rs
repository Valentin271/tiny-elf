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
    let word_len = word.len() as i32;

    let program = {
        use tiny_elf::asm::{Memory, Mnemonic::*, Register::*};

        Program::default()
            .add(Mov(Rbx, (-2).into()))
            .add(Imul(Rbx, (-2i8).into()))
            .add(Cmp(Rbx, 4))
            .add(Je("print".into()))
            .label("read")
            .add(Mov(Rax, 0.into()))
            .add(Mov(Rdi, 1.into()))
            .add(Mov(Rsi, Memory::from("msg").into()))
            .add(Mov(Rdx, word_len.into()))
            .add(Syscall)
            .label("printn")
            .add(Mov(Rax, 1.into()))
            .add(Mov(Rdi, 1.into()))
            // push what to print on stack, 10 being \n
            .add(Push(10.into()))
            .add(Push(5.into()))
            // "convert" number 5 to ascii
            .add(Pop(Rbx))
            .add(Add(Rbx, 48.into()))
            .add(Push(Rbx.into()))
            // Give parameter
            .add(Mov(Rsi, Rsp.into()))
            // clear stack
            .add(Pop(Rbx))
            .add(Pop(Rbx))
            // back to normal
            .add(Mov(Rdx, 16.into())) // 8 bytes addresses
            .add(Syscall)
            .label("print")
            .add(Mov(Rax, 1.into()))
            .add(Mov(Rdi, 1.into()))
            .add(Mov(Rsi, Memory::from("msg").into()))
            .add(Mov(Rdx, word_len.into()))
            .add(Syscall)
            .label("stop")
            .add(Mov(Rax, 60.into()))
            .add(Mov(Rdi, 0.into()))
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
