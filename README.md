# Helical Fourier Transform
This repository contains the source code for **helix-fft**, a web-app that will compute and display the diffraction pattern of helices. 

This may be useful in educational contexts or to quickly verify whether an observed diffraction pattern is compatible with a certain helical structure.

You can use it here [nemoandrea.github.io/helix-deploy/](https://nemoandrea.github.io/helixiser/), no installation required!



# Tutorial & Example

You can have a look at the [tutorial](doc/tutorial.md) and  [examples](doc/examples.md).

 If you have any interesting structures that you'd like to have added to the list of examples, send me  message on twitter (link on helix-fft page) or email.

# Prior work & References

 There have been various other tools that have similar aims to helix-FFT, which have all required local installation. These tools are otherwise great and will also work if you do not mind installations.

**Helix** - (MS visual basic 6.0)

```
Knupp, Carlo, and John M. Squire. "HELIX: a helical diffraction simulation program." Journal of applied crystallography 37.5 (2004): 832-835.
```

**HelicalDiffractionSimulator** - (Python and Javascript) - [view repository](https://gitlab.tudelft.nl/aj-lab/HelicalDiffractionSimulator)

```
HelicalDiffractionSimulator - Web Service to Simulate the diffraction patterns of helical (protein) specimen. Stefan Huber, Arjen Jakobi. (2017)
```

# Modifying helix-FFT

If you want to make your own modifications, or run the code in this repository that is used to build the deployed version, you will need the following:

* `npm` -  (node package manager), this is required to build the javascript and vue side of the project. 
* `Rustup` - Rust (programming language) installer; this is required for the Rust and WebAssembly part of the project. Allows `cargo` (the rust package manager) to be used systemwide

If you have these two installed on your system, you can run the project locally by navigating to the project directory in terminal and typing:

`npm run serve`

Which will run the project on localhost. It should also automatically tie in and compile the WebAssembly required for the project. The first time you execute this, all the dependencies specified in `package.json` and `cargo.toml` for Javascript and Rust respectively will be installed (which may take a while). 

