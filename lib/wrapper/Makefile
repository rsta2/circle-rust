#
# Makefile
#

CIRCLERUSTHOME = ../..
CIRCLEHOME = $(CIRCLERUSTHOME)/circle

-include $(CIRCLERUSTHOME)/Config.mk

BINDGEN ?= $(CIRCLERUSTHOME)/bindgen/target/release/bindgen

OBJS	= wrapper.o

EXTRACLEAN += wrapper.rs

all: libwrapper.a wrapper.rs

wrapper.rs: wrapper.h
	@echo "  GEN   $@"
	@$(BINDGEN) wrapper.h --use-core > wrapper.rs

libwrapper.a: $(OBJS)
	@echo "  AR    $@"
	@rm -f $@
	@$(AR) cr $@ $(OBJS)

include $(CIRCLEHOME)/Rules.mk

-include $(DEPS)
