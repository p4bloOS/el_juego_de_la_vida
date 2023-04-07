cd ..
echo "------------------Comenzado proceso de compilación"
cargo build --release
cp target/release/el_juego_de_la_vida compilacion/el_juego_de_la_vida_linux_x86.bin
echo "-----------------Compilado para Linux"
cargo build --target x86_64-pc-windows-gnu --release
cp target/x86_64-pc-windows-gnu/release/el_juego_de_la_vida.exe compilacion/el_juego_de_la_vida_windows_x86.exe
echo "-----------------Compilado para Windows"
trunk build --release
rm -r compilacion/version_web/ --force
cp -r dist/ compilacion/version_web/
echo "-----------------Compilado para web. Proceso finalizado."
