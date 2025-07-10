$(hvisor_bin): elf
	@if ! command -v mkimage > /dev/null; then \
		sudo apt update && sudo apt install u-boot-tools; \
	fi && \
	$(OBJCOPY) $(hvisor_elf) --strip-all -O binary $(hvisor_bin).tmp && \
	mkimage -n hvisor_img -A arm64 -O linux -C none -T kernel -a 0x00500000 \
	-e 0x00500000 -d $(hvisor_bin).tmp $(hvisor_bin) && \
	rm -rf $(hvisor_bin).tmp
HVISOR_BIN_FULL_PATH = $(shell readlink -f $(hvisor_bin))
ROOT_LINUX_IMAGE = $(image_dir)/vmlinux
ROOT_LINUX_IMAGE_BIN = $(ROOT_LINUX_IMAGE).bin

ROOT_LINUX_DTB = $(shell readlink -f $(image_dir)/dts/zone0.dtb)

TARGET_FIT_IMAGE = fitImage
TARGET_FIT_IMAGE_PATH = $(shell readlink -f $(TARGET_FIT_IMAGE))

GDB ?= aarch64-linux-gnu-gdb
READELF ?= aarch64-linux-gnu-readelf
OBJDUMP = aarch64-linux-gnu-objdump

HVISOR_TMP_PATH = $(shell readlink -f $(hvisor_bin).tmp)
GCC_OBJCOPY = aarch64-linux-gnu-objcopy
.PHONY: dtb
dtb:
	make -C $(image_dir)/dts

.PHONY: gen-fit
gen-fit: $(hvisor_bin) dtb
	set -x
	@if [ ! -f $(image_dir)/its/fitImage.its ]; then \
                echo "Error: ITS file not found at $(image_dir)/its/fitImage.its"; \
                exit 1; \
        fi
	$(OBJCOPY) $(hvisor_elf) --strip-all -O binary $(HVISOR_TMP_PATH)

	$(GCC_OBJCOPY) $(ROOT_LINUX_IMAGE) --strip-all -O binary $(ROOT_LINUX_IMAGE_BIN)
	$(info ROOT_LINUX_IMAGE_BIN = $(ROOT_LINUX_IMAGE_BIN))
	$(info ROOT_LINUX_DTB = $(ROOT_LINUX_DTB))
	$(info HVISOR_TMP_PATH = $(HVISOR_TMP_PATH))

	@sed \
                -e "s|__ROOT_LINUX_IMAGE__|$(ROOT_LINUX_IMAGE_BIN)|g" \
                -e "s|__ROOT_LINUX_DTB__|$(ROOT_LINUX_DTB)|g" \
                -e "s|__HVISOR_TMP_PATH__|$(HVISOR_TMP_PATH)|g" \
                $(image_dir)/its/fitImage.its > temp-fit.its
	mkimage -f temp-fit.its fitImage
	@echo "Generated FIT image: fitImage"

