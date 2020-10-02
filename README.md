# Helical Fourier Transform
This repository contains the source code for **helixiser**, a web-app that will compute and display the diffraction pattern of helices. 

This may be useful in educational contexts or to quickly verify whether an observed diffraction pattern is compatible with a certain helical structure.

You can use it here [nemoandrea.github.io/helixiser/](https://nemoandrea.github.io/helixiser/), no installation required!



# Tutorial & Examples

You can have a look at the [tutorial](doc/tutorial.md) and  [examples](doc/examples.md).

 If you have any interesting structures that you'd like to have added to the list of examples, send me  message on twitter (link on helix-fft page) or email.

## Implementation details

You might be curious how some of the computations are implemented in Helixiser, or perhaps you suspect something is wrong/missing from the algorithms. Below you will find some short doc pages that will get you up to speed with the implementations.

* [FFT in helixiser](doc/fft.md)

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

* `npm` -  (node package manager), this is required to build the Javascript and Vue side of the project. 
* `Rustup` - Rust (programming language) installer; this is required for the Rust and WebAssembly part of the project. Allows `cargo` (the rust package manager) to be used systemwide

If you have these two installed on your system, you can run the project locally by navigating to the project directory in terminal and typing:

`npm run serve`

Which will run the project on localhost. It should also automatically tie in and compile the WebAssembly required for the project. The first time you execute this, all the dependencies specified in `package.json` and `cargo.toml` for Javascript and Rust respectively will be installed (which may take a while). 

# Running locally and long term support

If you want to run helixiser locally (or if you fear the the hosting page will go down at some point in the future), you can use the python script in [tool/run_build_local.py](tool/run_build_local.py). You will need to do the following:

1. clone the helixiser repository (https://github.com/NemoAndrea/helixiser)

   > Note: the helixiser repository is the **deployed** version of this repository. It is what you get if you run `npm run build` in **helix-fft** repository's directory.

2. copy the the `run_build_local.py` file into the root of the /helixiser repository.
3. run the python file **from** the /helixiser directory. (i.e. os.path is the /helixiser directory)

Now helixiser should open in a browser window. We are done!

> Note: the helixiser repository only contains the last build version of helixiser. If you want an older version to run locally, you will have to use the version history of **helix-fft** and run `npm run build yourself`. You can then copy over the build (which will be stored in `/dist`) to a folder named "helixiser" and proceed from step `2`. 

