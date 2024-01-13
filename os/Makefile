# Building
TARGET := riscv64gc-unknown-none-elf
MODE := release
KERNEL_ELF := target/$(TARGET)/$(MODE)/os
KERNEL_BIN := $(KERNEL_ELF).bin

# Building mode argument
ifeq ($(MODE), release)
	MODE_ARG := --release
endif

# BOARD
BOOTLOADER := ../bootloader/rustsbi-qemu.bin

# KERNEL ENTRY
KERNEL_ENTRY_PA := 0x80200000

# Binutils
OBJDUMP := rust-objdump --arch-name=riscv64
OBJCOPY := rust-objcopy --binary-architecture=riscv64

QEMU_ARGS := -machine virt \
			 -nographic \
			 -bios $(BOOTLOADER) \
			 -device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA)

run: build
	@qemu-system-riscv64 $(QEMU_ARGS)

debug: build
	@tmux new-session -d \
		"qemu-system-riscv64 $(QEMU_ARGS) -s -S" && \
		tmux split-window -h "gdb-multiarch -ex 'file $(KERNEL_ELF)' -ex 'set arch riscv:rv64' -ex 'target remote localhost:1234'" && \
		tmux -2 attach-session -d

clean:
	@cargo clean

gdbserver: build
	@qemu-system-riscv64 $(QEMU_ARGS) -s -S

gdbclient:
	@gdb-multiarch -ex 'file $(KERNEL_ELF)' -ex 'set arch riscv:rv64' -ex 'target remote localhost:1234'

build: $(KERNEL_BIN)
	@echo "build success"

$(KERNEL_BIN): kernel
	@$(OBJCOPY) $(KERNEL_ELF) --strip-all -O binary $@

kernel:
	@cd ../user && make build
	@echo Platform: $(BOARD)
	@cargo build $(MODE_ARG)

.PHONY: build kernel clean run debug gdbserver gdbclient
