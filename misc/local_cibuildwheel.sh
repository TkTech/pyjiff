#!/bin/bash

export CIBW_TEST_REQUIRES="pytest"
export CIBW_TEST_COMMAND="pytest"

export CIBW_ENVIRONMENT_LINUX='PATH=$HOME/.cargo/bin:$PATH'
export CIBW_BEFORE_ALL_LINUX="curl -sSf https://sh.rustup.rs | sh -s -- -y"
export CIBW_BEFORE_ALL_WINDOWS=rustup target add i686-pc-windows-msvc

cibuildhweel