# Generate a runnable binary and compilable assembly
bin:
	cargo run --features asm
	chmod u+x bin

# Disassemble the generated bin
dis: bin
	objdump -b binary --start-address 0xb0 -m i386:x86-64 -D bin

# Compile the generated assembly with nasm to prove it works
asm: bin
	nasm -f elf64 dump.asm
	\ld dump.o -o dump.out

clean:
	rm -rf bin dump.asm *.o *.out
