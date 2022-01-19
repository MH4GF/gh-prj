#!/bin/sh

# this workaround is for building Mac OS
# ref: https://github.com/rust-build/rust-build.action/issues/36#issuecomment-1014738213

echo "Running pre-build script"
sed '/^export CXX.*/a export PATH="/opt/osxcross/target/bin:$PATH"\nexport LIBZ_SYS_STATIC=1' -i /build.sh
