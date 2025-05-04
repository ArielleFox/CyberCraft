#!/bin/bash

LATESTVERSION="Python-3.14.0a7"
echo "Downloading Latest Version: " $LATESTVERSION

mkdir -p ~/.cache/
cd ~/.cache/

echo "Downloading " $LATESTVERSION
wget https://www.python.org/ftp/python/3.14.0/$LATESTVERSION.tgz

echo "Extracting " $LATESTVERSION
tar -xf $LATESTVERSION.tgz
rm -f $LATESTVERSION.tgz

cd $LATESTVERSION
echo "Configuring " $LATESTVERSION
./configure --enable-optimizations --disable-gil --enable-wasm-pthreads --enable-wasm-dynamic-linking --enable-loadable-sqlite-extensions --enable-safety
echo "Building" $LATESTVERSION
make
sudo make install

echo "Installed " $LATESTVERSION " In Directory:"
pwd
cd -
