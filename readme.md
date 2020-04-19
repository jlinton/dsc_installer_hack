dsc-it-100 interface installer code hack, in rust
=================================================

This branch contains a program to brute force the
installer code on a DSC-1575 control panel. It
should work with little changes across the entire
DSC powerseries control panel line that interfaces
with the DSC-IT-100 RS-232 module. 

Its mostly a hack utility, so the source needs
to be edited to change the interface (default
/dev/ttyUSB0) if needed. When run it simply
starts sending the *8-CODE sequence at 0000
working its way to 9999. If during this process
it detects the panel actually accept the code
it will quit and display said code. 

Building/Running
----------------

This should build/run on any POSIX like rust 
development environment with:

`cargo build`

then
`target/debug/rust_proj`

It was developed on an opensuse machine with
a rustc from rustup. It was mostly run on a
fedora 30 based rpi3.

If there is a communication problem look at the 
serialport::open_with_settings() call, which
specifies the serial port as /dev/ttyUSB0 and the
baud rate is selected immediately above it. Those
two settings may need to be adjusted if your
dsc-it-100 is not at 9600bps, or its not connected
to a RS-232->USB converter.

Background
----------
This program was written because I wanted something
somewhat lightweight to run on a rpi3 to interface to
the DSC PowerSeries alarm control panels. These panels
are probably the most common alarm control panels in the
area I live in by a large margin. They are fairly
inexpensive, fairly well documented, and they have
been around with little changes for decades. So unlike
the CADT model of product design the base four wire
power/data interface protocol they use is dead simple
and can be monitored with simple GPIO/resistor pins on
pretty common microcontroller. 
My old house had a PC-1616 I installed, but when I
moved to the new house a few years ago, it already
had a hard wired system left over from when it was
built twenty years ago. And like many of these panels
it had an unknown installer code. I wanted to extend
the system and reprogram it to send text messages
via a computer interface I controlled.

Why rust? Because I thought this would be a short
"real life" program for gaining some additional
rust skills.

