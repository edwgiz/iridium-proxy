FROM ubuntu:15.10
LABEL authors="edwgiz"

RUN sed -i "s/archive.ubuntu/old-releases.ubuntu/g" /etc/apt/sources.list
RUN apt update
RUN apt install -y gcc make
RUN apt install -y wget build-essential cpio unzip rsync file bc python libncurses-dev vi

RUN wget --no-check-certificate https://buildroot.org/downloads/buildroot-2019.08.tar.gz
#RUN tar -xf buildroot-2019.08.tar.gz -C /opt

# qemu-system-x86_64 -kernel bzImage -drive file=rootfs.ext4,if=virtio,format=raw -append "root=/dev/vda console=ttyS0" -nographic -net nic,model=virtio

ENTRYPOINT ["bin/bash"]