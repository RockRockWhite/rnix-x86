BUILD:=./target
SRC:=./src

TARGET := i686-unknown-none
# TARGET := i686-unknown-linux-musl
MODE := release

# Building mode argument
ifeq ($(MODE), release)
	MODE_ARG := --release
endif

ifeq ($(TARGET), i686-unknown-none)
	TARGET_ARG := --target "i686-unknown-none.json"
endif

ENTRYPOINT:=0x10000

$(BUILD)/bootloader/%.bin: $(SRC)/bootloader/%.asm
	$(shell mkdir -p $(dir $@))
	nasm -f bin $< -o $@

$(BUILD)/kernel/%.o: $(SRC)/kernel/%.asm
	$(shell mkdir -p $(dir $@))
	nasm -f elf32 $(DEBUG) $< -o $@

$(BUILD)/kernel/%.a: $(BUILD)/$(TARGET)/$(MODE)/%.a
	$(shell mkdir -p $(dir $@))
	cp $< $@

.PHONY: $(BUILD)/$(TARGET)/$(MODE)/libkernel.a
$(BUILD)/$(TARGET)/$(MODE)/libkernel.a:
	cargo xbuild $(MODE_ARG) $(TARGET_ARG)

$(BUILD)/kernel.bin: $(BUILD)/kernel/start.o \
	$(BUILD)/kernel/libkernel.a
	$(shell mkdir -p $(dir $@))
	ld.lld -m elf_i386 -static $^ -o $@ -Ttext $(ENTRYPOINT)

$(BUILD)/system.bin: $(BUILD)/kernel.bin
	objcopy -O binary $< $@

$(BUILD)/system.map: $(BUILD)/kernel.bin
	nm $< | sort > $@ 

$(BUILD)/master.img: $(BUILD)/bootloader/boot.bin \
	$(BUILD)/bootloader/loader.bin \
	$(BUILD)/system.bin \
	$(BUILD)/system.map

	yes | bximage -q -hd=16 -func=create -sectsize=512 -imgmode=flat $@
	dd if=$(BUILD)/bootloader/boot.bin of=$@ bs=512 count=1 conv=notrunc
	dd if=$(BUILD)/bootloader/loader.bin of=$@ bs=512 count=4 seek=2 conv=notrunc
	dd if=$(BUILD)/system.bin of=$@ bs=512 count=1000 seek=10 conv=notrunc

test: $(BUILD)/kernel/libkernel.a

.PHONY: clean
clean:
	cargo clean

.PHONY: bochs
bochs: $(BUILD)/master.img
	bochs -q

.PHONY: qemu
qemu: $(BUILD)/master.img
	qemu-system-i386 \
	-m 32M \
	-boot c \
	-hda $<

.PHONY: qemug
qemug: $(BUILD)/master.img
	qemu-system-i386 \
	-s -S \
	-m 32M \
	-boot c \
	-hda $<