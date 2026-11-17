# Link the Rust kernel with the same compiled drivers and support archives
# as the C hardware target; no Circle archive is rebuilt here.
LIBS = $(REI_KERNEL_LIB) \
       $(REI_DRIVER_BUNDLE)/addon/fatfs/libfatfs.a \
       $(REI_DRIVER_BUNDLE)/lib/fs/libfs.a \
       $(REI_DRIVER_BUNDLE)/lib/net/libnet.a \
       $(REI_DRIVER_BUNDLE)/lib/sched/libsched.a

include $(CIRCLEHOME)/Rules.mk

LDFLAGS := $(filter-out --gc-sections,$(LDFLAGS))
