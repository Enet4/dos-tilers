#!/usr/bin/env sh
set -eu

# only call build.sh if the file was not built yet
if [ ! -f ./dist/release/tilers.exe ]; then
    ./build.sh release
fi
mkdir -p ./dist/js-dos
# add tilers.exe, assets, dosbox.conf and CWSDPMI.EXE into new zip file
cd bundle
rm -f dos_tilers.zip
cp ../dist/release/tilers.exe ./
zip -q dos_tilers.zip \
    CWSDPMI.EXE \
    tilers.exe \
    .jsdos/dosbox.conf
rm -f tilers.exe

# rename it as dos_tilers.jsdos
cp dos_tilers.zip ../dist/js-dos/dos_tilers.jsdos
rm dos_tilers.zip
echo "Created bundle dist/js-dos/dos_tilers.jsdos"
