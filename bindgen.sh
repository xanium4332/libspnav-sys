#!/bin/sh
bindgen --version
bindgen --with-derive-default libspnav/spnav.h -o src/lib.rs -- -Iinclude
