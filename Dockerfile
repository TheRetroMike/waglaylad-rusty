FROM ubuntu:jammy
RUN apt-get update -y
RUN apt-get install wget unzip -y
WORKDIR /opt/
RUN wget https://github.com/user-attachments/files/17392272/Linux.-.waglayalad-rusty.zip
RUN unzip Linux.-.waglayalad-rusty.zip
chmod +x 'Linux - waglayalad-rusty'/*
RUN mv 'Linux - waglayalad-rusty'/* /usr/bin/
CMD /usr/bin/waglaylad
