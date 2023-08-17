FROM ubuntu:16.10
LABEL authors="edwgiz"

# https://cdn.kernel.org/pub/linux/kernel/v4.x/
# https://releases.linaro.org/components/toolchain/binaries/6.3-2017.02/arm-linux-gnueabihf/

RUN apt install gcc make gcc-arm-linux-gnueabi binutils-arm-linux-gnueabi
RUN apt install patchelf
RUN patchelf --set-interpreter /lib/ld-linux-armhf.so.3 helloworld-arm

RUN apt install -y libusb-dev libdbus-1-dev libglib2.0-dev libudev-dev libical-dev libreadline-dev
RUN apt install -y udev

RUN apt install wget
RUN wget http://www.kernel.org/pub/linux/bluetooth/bluez-5.47.tar.xz
RUN tar xvf bluez-5.47.tar.xz
RUN cd bluez-5.47

ENV CC=/usr/bin/arm-linux-gnueabi-gcc
ENV LDFLAGS=-pthread
RUN ./configure --enable-library  \
    --host=arm-linux-gnueabihf  \
    --enable-dependency-tracking  \
    --disable-obex \
    --disable-cups  \
    --disable-manpages \
    --disable-systemd


ENTRYPOINT ["top", "-b"]
