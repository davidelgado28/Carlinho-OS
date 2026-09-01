#!/bin/bash
set -e

echo "[Carlinhos OS] Compilando o compilador Carlos (Rust)."
cd lang
cargo build --release
cd ..

echo "[Carlinhos OS] Compilando componentes do sistema C++ (CMake)."
mkdir -p build
cd build
cmake ..
make

echo "[Carlinhos OS] Build completo finalizado com sucesso."
