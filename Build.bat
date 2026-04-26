@echo off
echo [1/3] Setting up environment...
call "C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Auxiliary\Build\vcvars32.bat" > nul

echo [2/3] Compiling Rust code...
rustc --target i686-pc-windows-msvc --edition 2021 -C opt-level=z -C panic=abort -C debug-assertions=off -C overflow-checks=off -C lto=fat --emit obj src\main.rs -o main.obj

echo [3/3] Crinkling to EXE...
crinkler.exe /ENTRY:mainCRTStartup /SUBSYSTEM:CONSOLE /TINYIMPORT /TINYHEADER /COMPRESSION:SLOW /OUT:game_tiny.exe main.obj kernel32.lib user32.lib

echo Done! Final size:
dir game_tiny.exe | find "game_tiny.exe"
pause