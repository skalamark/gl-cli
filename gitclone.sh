git clone -b develop https://github.com/skalamark/gl.git
git clone -b develop https://github.com/skalamark/gl-core.git
git clone -b develop https://github.com/skalamark/gl-runtime.git
git clone -b develop https://github.com/skalamark/gl-std.git

cd gl
cargo build
echo "Usage 'gl'"

