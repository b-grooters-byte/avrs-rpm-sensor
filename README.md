![Banner](images/banner.png "Banner")

This project is an RPM sensor example using embedded Rust, Arduino Uno R3, and a hall-effect sensor. This project builds on the [avrs-hall-sensor](https://gihub.com/bytetrail/avrs-hall-sensor) project so you may want to review that project prior to starting this project for environment setup and build instructions.

# Getting Started

The parts list below is for the project as described here. You may substitute a different motor; however, you would then need to build your own motor and sensor mount.

## Parts List
* Arduino Uno R3
* [A3144 Hall Effect Sensor](https://www.amazon.com/gp/product/B07QNX6HWT)
* 10k&#x3a9; Resistor
* Jumper wires
* [L9110S DC Motor Drive Module](https://www.amazon.com/gp/product/B00M0F243E)
* [4mm x 1mm neodymium magnet](https://www.amazon.com/gp/product/B076Q8SMBJ) - 2ea
* [0-6V 5000RPM Mini Motor](https://amazon.com/https://www.amazon.com/gp/product/B07YBVQZNJ)
* [M3 6mm Nylon Standoff](https://www.amazon.com/gp/product/B014K8MXO8/) - 4ea

## Power
I have the Arduino powered through USB power. You have several options for supplying power to the motor controller. I have the parts listed for 2 options:

__5V Power Supply__
* [Lithium Battery Shield Micro USB](https://www.amazon.com/gp/product/B07SZKNST4)
* 18650 Li-ion Battery - 2ea

__6V Power Supply__
* [Voltage Regulator](https://www.amazon.com/gp/product/B07WQJ2GD6)
* 7.4V LiPo or Li-ion Battery

In addition you will need appropriate connectors and wiring for the power supply. I show both options in the [construction guide](CONSTRUCTION.md).



# Assembly
We printed a motor mount that holds the motor, a flywheel with magnets, and a Hall Effect sensor for this project. The [construction guide](CONSTRUCTION.md) explains the basic assembly process.

![Completed](images/assembly-007.png)

__NOTE__: I have observed some variability in the hall effect sensors used. Once the mount is completed and wired up you may need to adjust the distance between the flywheel and the sensor if you are getting inconsistent detection results from the sensor.

# Building the Code
This project uses the same environment I setup in the [avrs-hall-sensor](https://github.com/bytetrail/avrs-hall-sensor) project. I suggest reading through the setup guide in that project prior to starting this project if you do not already have a Rust AVR build environment setup.
