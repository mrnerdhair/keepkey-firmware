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

#include <libopencm3/cm3/scb.h>

#include "keepkey/board/util.h"

#include <inttypes.h>

static const char *hexdigits = "0123456789ABCDEF";

void data2hex(const void *data, uint32_t len, char *str) {
  uint32_t i;
  const uint8_t *cdata = (uint8_t *)data;
  for (i = 0; i < len; i++) {
    str[i * 2] = hexdigits[(cdata[i] >> 4) & 0xF];
    str[i * 2 + 1] = hexdigits[cdata[i] & 0xF];
  }
  str[len * 2] = 0;
}
