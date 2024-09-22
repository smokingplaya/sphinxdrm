# Sphinx drm
My attempt to make a https-based drm system for my projects.
Here I used technologies that I have little knowledge of: Redis, Actix (I first wrote a web server in Rust), Docker

I decided to make this repository public a month after I created it,\
as it would be another project in my portfolio.

# The idea
The idea was that the project, before it starts, would send an HTTPS request to this drm web server,\
with the mac-adress of the device on which the product should run. HTTPS cannot be spoofed,\
so it will be impossible/difficult to bypass this drm.\
If HTTPS request returns code 200 - then the project is started, if not - then the process dies.

# Usage
```sh
sudo apt install git
git clone https://github.com/smokingplaya/sphinxdrm
sudo docker build -t sphinx_drm .
sudo docker compose up
```
