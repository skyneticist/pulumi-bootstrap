!~/bin/sh

cargo build --release
npm pack
npm i -g ./pulumime-1.0.0.tgz
# cp ./node_modules/pulumime/bin/pulumi-bootstrap.exe ./local_test/pulumime.exe
