# Turn off my screen
A small utility to turn off (set brightness to 0) the screen using the /sys/class/backlight controls in linux.

It's a small tool I needed on my computer, so consider it a small learning project, but it might work for also your computer.

## Other considerations
It also uses the setuid flag to actually enable any user to change the backlight, as that file is only writable by root.

To protect against security vulnerabilities it is completely not configurable and accepts no command-line parameters (and also, it just needs to work on my laptop. Customize the code for yours!).

Comes with a PKGBUILD to actually build a package on arch linux, I tested it only on my pc (which has already all the deps installed), so it might not work out of the box. 

Requires only a rust compiler and GNU make.

PRs are welcome, even for small improvements!