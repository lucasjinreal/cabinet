# !/usr/bin/sh
sudo chmod -R 777 *
if [ "$(uname)" == "Darwin" ]; then
    # Do something under Mac OS X platform
    sudo cp bin/cab_macos /usr/local/bin/cab
elif [ "$(expr substr $(uname -s) 1 5)" == "Linux" ]; then
    # Do something under GNU/Linux platform
    sudo apt install -y libcurl4-openssl-dev
    sudo cp bin/cab_linux /usr/local/bin/cab
elif [ "$(expr substr $(uname -s) 1 10)" == "MINGW32_NT" ]; then
    # Do something under 32 bits Windows NT platform
    echo "this is windows 32bit"
    
elif [ "$(expr substr $(uname -s) 1 10)" == "MINGW64_NT" ]; then
    # Do something under 64 bits Windows NT platform
    echo "this is windows 64bit"
fi

echo "You just installed cabinet - ultimate tool box!"
