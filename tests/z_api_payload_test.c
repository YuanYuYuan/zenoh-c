//
// Copyright (c) 2022 ZettaScale Technology
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ZettaScale Zenoh Team, <zenoh@zettascale.tech>

#include <assert.h>
#include <stddef.h>
#include <stdio.h>
#include <string.h>

#include "zenoh.h"

#undef NDEBUG
#include <assert.h> 

void test_reader() {
    uint8_t data[] = {0, 1, 2, 3, 4, 5, 6, 7, 8, 9};
    uint8_t data_out[10] = {0};
    z_bytes_t bytes = {.start = data, .len = 10 };

    zc_owned_payload_t payload = zc_payload_encode_from_bytes(bytes);
    zc_payload_reader reader;
    zc_payload_reader_init(z_loan(payload), &reader);
    assert(zc_payload_reader_remaining(&reader) == 10);

    zc_payload_reader_read(&reader, data_out, 5);
    assert(zc_payload_reader_remaining(&reader) == 5);
    zc_payload_reader_read(&reader, data_out, 5);
    assert(zc_payload_reader_remaining(&reader) == 0);
    assert(memcmp(data, data_out, 10));
}

int main(int argc, char **argv) {
    test_reader();
}
