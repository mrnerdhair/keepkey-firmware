/*
 * This file is part of the TREZOR project.
 *
 * Copyright (C) 2014 Pavol Rusnak <stick@satoshilabs.com>
 *
 * This library is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with this library.  If not, see <http://www.gnu.org/licenses/>.
 */

#ifndef MEMORY_H
#define MEMORY_H

//#include <libopencm3/cm3/mpu.h>

#include <stddef.h>
#include <stdbool.h>
#include <inttypes.h>
#include <assert.h>

// clang-format off
/*

 flash memory layout:
 --------------------
   name    |          range          |  size   |     function     |   MPU Protection
-----------+-------------------------+---------+------------------+----------------------
 Sector  0 | 0x08000000 - 0x08003FFF |  16 KiB | bootstrap code   |  signature dependent 
 Sector  1 | 0x08004000 - 0x08007FFF |  16 KiB | storage/config   |     full access
-----------+-------------------------+---------+------------------+----------------------
 Sector  2 | 0x08008000 - 0x0800BFFF |  16 KiB | storage/config   |     full access
 Sector  3 | 0x0800C000 - 0x0800FFFF |  16 KiB | storage/config   |     full access
-----------+-------------------------+---------+------------------+----------------------
 Sector  4 | 0x08010000 - 0x0801FFFF |  64 KiB | empty            |     full access
 Sector  5 | 0x08020000 - 0x0803FFFF | 128 KiB | bootloader code  |  signature dependent
 Sector  6 | 0x08040000 - 0x0805FFFF | 128 KiB | bootloader code  |  signature dependent
 Sector  7 | 0x08060000 - 0x0807FFFF | 128 KiB | application code |     full access
 Sector  8 | 0x08080000 - 0x0809FFFF | 128 KiB | application code |     full access
 Sector  9 | 0x080A0000 - 0x080BFFFF | 128 KiB | application code |     full access
 Sector 10 | 0x080C0000 - 0x080DFFFF | 128 KiB | application code |     full access
 Sector 11 | 0x080E0000 - 0x080FFFFF | 128 KiB | application code |     full access

 Application metadata area (first 256 bytes of application code)
 -------------------------
 offset | type/length |  description
--------+-------------+-------------------------------
 0x0000 |  4 bytes    |  magic = 'KPKY'
 0x0004 |  uint32     |  length of the code (codelen)
 0x0008 |  uint8      |  signature index #1
 0x0009 |  uint8      |  signature index #2
 0x000A |  uint8      |  signature index #3
 0x000B |  uint8      |  SIG_FLAG (old bootloaders test this entire byte as a flag)
 0x000C |  uint32     |  META_FLAGS
 0x0010 |  48 bytes   |  reserved
 0x0040 |  64 bytes   |  signature #1
 0x0080 |  64 bytes   |  signature #2
 0x00C0 |  64 bytes   |  signature #3
 0x0100 |  32K-256 B  |  persistent storage

 SIG_FLAG != 0 -> restore storage after flashing (if signatures are ok)
 META_FLAGS & 0x00000001 == true -> boot into firmware update mode

 */
// clang-format on

#define FLASH_PTR(x) (const uint8_t *)(x)

#define OPTION_BYTES_1 ((uint64_t *)0x1FFFC000)
#define OPTION_BYTES_2 ((uint64_t *)0x1FFFC008)
#define OPTION_RDP 0xCCFF
#define OPTION_WRP 0xFF9E

#define OTP_MFG_ADDR 0x1FFF7800
#define OTP_MFG_SIG 0x08012015
#define OTP_MFG_SIG_LEN 4
#define OTP_MODEL_ADDR 0x1FFF7820
#define OTP_BLK_LOCK(x) (0x1FFF7A00 + (x - 0x1FFF7800) / 0x20)

#define BSTRP_FLASH_SECT_LEN 0x4000
#define STOR_FLASH_SECT_LEN 0x4000
#define UNUSED_FLASH_SECT0_LEN 0x10000
#define BLDR_FLASH_SECT_LEN 0x20000
#define APP_FLASH_SECT_LEN 0x20000

#define BSTRP_FLASH_SECT_START 0x08000000
#define BLDR_FLASH_SECT_START 0x08020000

/* meta info */
#define META_MAGIC_STR "KPKY"

/* Flash Info */
#define FLASH_ORIGIN (0x08000000)
#define FLASH_TOTAL_SIZE (1024 * 1024)
#define FLASH_END (FLASH_ORIGIN + FLASH_TOTAL_SIZE)

/* Boot Strap Partition */
#define FLASH_BOOTSTRAP_START (FLASH_ORIGIN)  // 0x0800_0000 - 0x0800_3FFF
#define FLASH_BOOTSTRAP_LEN (0x4000)

/* Storage/Configuration Partition */

#define FLASH_STORAGE_LEN (0x4000)

/*<  0x801_0000 - 0x801_FFFF is empty  >*/

/* Boot Loader Partition */
#define FLASH_BOOT_START (0x08020000)  // 0x0802_0000 - 0x0805_FFFF
#define FLASH_BOOT_LEN (0x40000)

/* Application Partition */
#define FLASH_META_START (FLASH_BOOT_START + FLASH_BOOT_LEN)  // 0x0806_0000
#define FLASH_META_DESC_LEN (0x100)

#define FLASH_APP_START \
  (FLASH_META_START + FLASH_META_DESC_LEN)  // 0x0806_0200 - 0x080F_FFFF
#define FLASH_APP_LEN (FLASH_END - FLASH_APP_START)

#define SIG_FLAG (*(uint8_t const *)FLASH_SIG_FLAG)

#define META_FLAGS (*(uint8_t const *)FLASH_META_FLAGS)


/* Misc Info. */
#define FLASH_BOOTSTRAP_SECTOR 0

#define FLASH_BOOTSTRAP_SECTOR_FIRST 0
#define FLASH_BOOTSTRAP_SECTOR_LAST 0

#define FLASH_STORAGE_SECTOR_FIRST 1
#define FLASH_STORAGE_SECTOR_LAST 3

#define FLASH_VARIANT_SECTOR_FIRST 4
#define FLASH_VARIANT_SECTOR_LAST 4

#define FLASH_BOOT_SECTOR_FIRST 5
#define FLASH_BOOT_SECTOR_LAST 6

#define FLASH_APP_SECTOR_FIRST 7
#define FLASH_APP_SECTOR_LAST 11

#define STORAGE_SECT_DEFAULT FLASH_STORAGE1

// Storage Protection:
//
// Due to a disclosure at 35c3 by Dmitry Nedospasov, Josh Datko and Thomas
// Roth, we cannot load read the secrets into ram anymore during firmware
// update. To mitigate this, and still allow preservation of secrets across
// firmware updates, we check for the presence of some magic bytes at the
// beginning of the storage sector immediately following the active sector.
// If the bytes do not match the known magic, secrets are wiped before booting.
//
// We use a non-trivial sequence of bytes for the magic in order to severely
// increase difficulty of glitching past the check:
//
// echo -n "boot allowed" | shasum -a 256
#define STORAGE_PROTECT_OFF_MAGIC                                            \
  "\x31\x88\x4e\xb8\x48\x2a\x28\x09\xe3\x74\x61\xd9\x6a\xd7\xf0\xed\x8c\xdd" \
  "\x7c\xa6\x07\x3e\x68\x6a\x15\xc0\x89\xc6\x11\x89\x95\xa0"
#define STORAGE_PROTECT_ON_MAGIC                                             \
  "\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00" \
  "\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"

/* Application Meta format */
typedef enum {
  FLASH_INVALID,
  FLASH_BOOTSTRAP,
  FLASH_STORAGE1,
  FLASH_STORAGE2,
  FLASH_STORAGE3,
  FLASH_UNUSED0,
  FLASH_BOOTLOADER,
  FLASH_APP,
} Allocation;
_Static_assert(sizeof(Allocation) == sizeof(uint8_t), "Allocation is not backed by a uint8_t");

typedef struct {
  int sector;
  size_t start;
  uint32_t len;
  Allocation use;
} FlashSector;

static const FlashSector flash_sector_map[] = {
    {0, 0x08000000, BSTRP_FLASH_SECT_LEN, FLASH_BOOTSTRAP},
    {1, 0x08004000, STOR_FLASH_SECT_LEN, FLASH_STORAGE1},
    {2, 0x08008000, STOR_FLASH_SECT_LEN, FLASH_STORAGE2},
    {3, 0x0800C000, STOR_FLASH_SECT_LEN, FLASH_STORAGE3},
    {4, 0x08010000, UNUSED_FLASH_SECT0_LEN, FLASH_UNUSED0},
    {5, 0x08020000, BLDR_FLASH_SECT_LEN, FLASH_BOOTLOADER},
    {6, 0x08040000, BLDR_FLASH_SECT_LEN, FLASH_BOOTLOADER},
    {7, 0x08060000, APP_FLASH_SECT_LEN, FLASH_APP},
    {8, 0x08080000, APP_FLASH_SECT_LEN, FLASH_APP},
    {9, 0x080A0000, APP_FLASH_SECT_LEN, FLASH_APP},
    {10, 0x080C0000, APP_FLASH_SECT_LEN, FLASH_APP},
    {11, 0x080E0000, APP_FLASH_SECT_LEN, FLASH_APP},
    {-1, 0, 0, FLASH_INVALID}};

void mpu_config(void);

bool find_active_storage(Allocation *storage_location);

/// Find the storage location fff*after* the active one.
Allocation next_storage(Allocation active);

void memory_getDeviceLabel(char *str, size_t len);

/// Write the marker that allows the firmware to boot with secrets preserved.
bool storage_protect_off(void);

extern void *_timerusr_isr;
extern void *_buttonusr_isr;
extern void *_mmhusr_isr;

#endif
