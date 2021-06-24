# leo add (w/o login) & remove

LEO new my-app && cd my-app || exit 1
LEO add justice-league/u8u32
echo "import u8u32.u8_u32; function main() {}" > src/main.leo
LEO build
LEO remove u8u32
LEO clean
