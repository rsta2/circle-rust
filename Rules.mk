#
# Rules.mk
#

CIRCLERUSTHOME ?= ..
CIRCLEHOME ?= $(CIRCLERUSTHOME)/circle

-include $(CIRCLERUSTHOME)/Config.mk
-include $(CIRCLEHOME)/Config.mk
-include $(CIRCLEHOME)/Config2.mk

AARCH ?= 32
RASPPI ?= 4

RUST_MODE ?= debug

ifeq ($(strip $(AARCH)),32)
ifeq ($(strip $(RASPPI)),2)
TARGET = kernel7
else ifeq ($(strip $(RASPPI)),3)
TARGET = kernel8-32
else ifeq ($(strip $(RASPPI)),4)
TARGET = kernel7l
else
$(error RASPPI must be set to 2, 3 or 4)
endif
RUST_TARGET = armv7a-none-eabi
PREFIX ?= arm-none-eabi-
LINKSCRIPT = $(CIRCLERUSTHOME)/lib/circle32.ld
else
ifeq ($(strip $(RASPPI)),3)
TARGET = kernel8
else ifeq ($(strip $(RASPPI)),4)
TARGET = kernel8-rpi4
else ifeq ($(strip $(RASPPI)),5)
TARGET = kernel_2712
else
$(error RASPPI must be set to 3, 4 or 5)
endif
RUST_TARGET = aarch64-unknown-none
PREFIX64 ?= aarch64-none-elf-
PREFIX = $(PREFIX64)
LINKSCRIPT = $(CIRCLERUSTHOME)/lib/circle64.ld
endif

OBJCOPY	= $(PREFIX)objcopy
OBJDUMP	= $(PREFIX)objdump

LIBPATH += -L $(CIRCLERUSTHOME)/lib/wrapper -L $(CIRCLEHOME)/lib

LIBS += -lwrapper -lcircle

$(TARGET).img: $(SRCS)
	@echo "  RUSTC $(PACKAGE)"
	@cargo rustc -q --target=$(RUST_TARGET) -- $(LIBPATH) $(LIBS) \
		-C link-arg=--script=$(LINKSCRIPT)
	@cp target/$(RUST_TARGET)/$(RUST_MODE)/$(PACKAGE) ./$(TARGET).elf
	@echo "  DUMP  $(TARGET).lst"
	@$(OBJDUMP) -d $(TARGET).elf > $(TARGET).lst
	@echo "  COPY  $(TARGET).img"
	@$(OBJCOPY) $(TARGET).elf -O binary $(TARGET).img
	@echo -n "  WC    $(TARGET).img => "
	@wc -c < $(TARGET).img

clean:
	@echo "  CLEAN " `pwd`
	@cargo -q clean
	@rm -f *.d *.o *.a *.elf *.lst *.img *.hex *.cir *.map *~ $(EXTRACLEAN)


# Same as in circle/Rules.mk follows:

ifneq ($(strip $(SDCARD)),)
install: $(TARGET).img
	cp $(TARGET).img $(SDCARD)
	sync
endif

ifneq ($(strip $(TFTPHOST)),)
tftpboot: $(TARGET).img
	tftp -m binary $(TFTPHOST) -c put $(TARGET).img
endif

#
# Eclipse support
#

SERIALPORT  ?= /dev/ttyUSB0
USERBAUD ?= 115200
FLASHBAUD ?= 115200
REBOOTMAGIC ?=

$(TARGET).hex: $(TARGET).img
	@echo "  COPY  $(TARGET).hex"
	@$(OBJCOPY) $(TARGET).elf -O ihex $(TARGET).hex

# Command line to run node and python.
# Including the '.exe' forces WSL to run the Windows host version
# of these commands.  If putty and node are available on the windows
# machine we can get around WSL's lack of serial port support
ifeq ($(strip $(WSL_DISTRO_NAME)),)
NODE=node
PUTTY=putty
PUTTYSERIALPORT=$(SERIALPORT)
else
NODE=node.exe
PUTTY=putty.exe
PUTTYSERIALPORT=$(subst /dev/ttyS,COM,$(SERIALPORT))		# Remap to windows name
endif

ifeq ($(strip $(USEFLASHY)),)

# Flash with python
flash: $(TARGET).hex
ifneq ($(strip $(REBOOTMAGIC)),)
	python3 $(CIRCLEHOME)/tools/reboottool.py $(REBOOTMAGIC) $(SERIALPORT) $(USERBAUD)
endif
	python3 $(CIRCLEHOME)/tools/flasher.py $(TARGET).hex $(SERIALPORT) $(FLASHBAUD)

else

# Flash with flashy
ifeq ($(strip $(USEFLASHY)),1)
	FLASHY ?= $(NODE) $(CIRCLEHOME)/tools/flashy/flashy.js
else
	FLASHY ?= flashy
endif

flash: $(TARGET).hex
	$(FLASHY) \
		$(SERIALPORT) \
		--flashBaud:$(FLASHBAUD) \
		--userBaud:$(USERBAUD) \
		--reboot:$(REBOOTMAGIC) \
		$(FLASHYFLAGS) \
		$(TARGET).hex

endif

# Monitor in putty
monitor:
	$(PUTTY) -serial $(PUTTYSERIALPORT) -sercfg $(USERBAUD)

# Monitor in terminal (Linux only)
cat:
	stty -F $(SERIALPORT) $(USERBAUD) cs8 -cstopb -parenb -icrnl
	cat $(SERIALPORT)
