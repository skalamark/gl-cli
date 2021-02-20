clear && cargo fmt && cargo build --release
rm ~/.glanguage/bin/gl
cp target/release/gl ~/.glanguage/bin/gl
