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
    let upward_data = "We went upward\n";

    let program = {
        use tiny_elf::asm::{Memory, Mnemonic::*, Register::*};

        Program::default()
            .add(Mov(Rax, 8.into()))
            .add(Mov(Rbx, 2.into()))
            .add(IDiv(Rbx))
            .add(Cmp(Rax, 4))
            .add(Je("foo".into()))
            .label("upward")
            .insert_data("upward_data", upward_data)
            .add(Mov(Rsi, Memory::from("upward_data").into()))
            .add(Mov(Rdx, (upward_data.len() as i32).into()))
            .add(Call("print".into()))
            .add(Jmp("exit".into()))
            .label("read")
            .add(Mov(Rax, 0.into()))
            .add(Mov(Rdi, 0.into()))
            .add(Mov(Rsi, Memory::from("msg").into()))
            .add(Mov(Rdx, word_len.into()))
            .add(Syscall)
            .label("foo")
            .add(Mov(Rsi, Memory::from("msg").into()))
            .add(Mov(Rdx, word_len.into()))
            .add(Call("print".into()))
            .add(Jmp("upward".into()))
            // functions
            .func("print")
            .add(Mov(Rax, 1.into()))
            .add(Mov(Rdi, 1.into()))
            .add(Syscall)
            .func_end()
            .label("exit")
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
