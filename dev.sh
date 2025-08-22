#!/usr/bin/env bash
#
# This is a script for local DEVELOPMENT of the application.
# DO NOT USE it in your package manager or CI. 
# 
# To build the application successfully you must either be inside the `nix
# develop` shell (preferred) or have all dependencies and their correct versions
# installed on your system. 

set -o errexit
set -o nounset
set -o pipefail

meson setup build -Dprofile=dev -Dprefix=$(pwd)/installdir
meson compile -C build
meson install -C build
XDG_DATA_DIRS=$(pwd)/installdir/share:$XDG_DATA_DIRS ./installdir/bin/^EXECUTABLE^
