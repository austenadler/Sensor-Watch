.SUFFIXES:

CFLAGS += $(INCLUDES) $(DEFINES)

OBJS = $(addprefix $(BUILD)/, $(notdir %/$(subst .c,.o, $(SRCS))))

SUBMODULES = tinyusb

COBRA = cobra -f

ifndef EMSCRIPTEN
RUST_BUILD_STD = panic_abort
all: $(BUILD)/$(BIN).elf $(BUILD)/$(BIN).hex $(BUILD)/$(BIN).bin $(BUILD)/$(BIN).uf2 size
else
RUST_BUILD_STD = std,panic_abort
all: $(BUILD)/$(BIN).html
endif

$(BUILD)/$(BIN).html: $(RUST_LIB) $(OBJS)
	@echo HTML $@
	@$(CC) $(LDFLAGS) $(OBJS) $(LIBS) -o $@ \
		-s ASYNCIFY=1 \
		-s EXPORTED_RUNTIME_METHODS=lengthBytesUTF8,printErr \
		-s EXPORTED_FUNCTIONS=_main \
		--shell-file=$(TOP)/watch-library/simulator/shell.html

$(BUILD)/$(BIN).elf: $(RUST_LIB) $(OBJS)
	@echo LD $@
	@$(CC) $(LDFLAGS) $(OBJS) $(LIBS) -o $@

$(RUST_LIB):
	cargo +nightly build --release --manifest-path $(TOP)/rust/Cargo.toml --target $(RUST_TARGET) -Z build-std=$(RUST_BUILD_STD) -Z build-std-features=panic_immediate_abort
	# arm-none-eabi-strip $(RUST_LIB)

$(BUILD)/$(BIN).hex: $(BUILD)/$(BIN).elf
	@echo OBJCOPY $@
	@$(OBJCOPY) -O ihex $^ $@

$(BUILD)/$(BIN).bin: $(BUILD)/$(BIN).elf
	@echo OBJCOPY $@
	@$(OBJCOPY) -O binary $^ $@

$(BUILD)/$(BIN).uf2: $(BUILD)/$(BIN).bin
	@echo UF2CONV $@
	@$(UF2) $^ -co $@

.PHONY: $(SUBMODULES) $(RUST_LIB)
$(SUBMODULES):
	git submodule update --init

install:
	@$(UF2) -D $(BUILD)/$(BIN).uf2

$(BUILD)/%.o: | $(SUBMODULES) $(RUST_LIB) directory
	@echo CC $@
	@$(CC) $(CFLAGS) $(filter %/$(subst .o,.c,$(notdir $@)), $(SRCS)) -c -o $@

directory:
	@$(MKDIR) -p $(BUILD)

size: $(BUILD)/$(BIN).elf
	@echo size:
	@$(SIZE) -t $^

clean:
	@echo clean
	@-rm -rf $(BUILD)

analyze:
	@$(COBRA) basic $(INCLUDES) $(DEFINES) $(SRCS)

DEPFILES := $(OBJS:%.o=%.o.d)

-include $(wildcard $(DEPFILES))
