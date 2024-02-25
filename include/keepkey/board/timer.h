/*
 * This file is part of the KeepKey project.
 *
 * Copyright (C) 2015 KeepKey LLC
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

#ifndef TIMER_H
#define TIMER_H

#include <stdint.h>
#include <stdbool.h>

void timerisr_usr(void);

void timer_init(void);
void delay_ms(uint32_t ms);
void delay_us(uint32_t us);

uint64_t get_clock_ms(void);
void set_clock_compare_ms(uint64_t ms);
extern void rust_clock_compare_callback(uint64_t ms);

/// Defense against Fault Injection: random delay of a few miliseconds
/// \returns the argument passed
uint32_t fi_defense_delay(volatile uint32_t value);

#endif
