# Example helices

Below are some example `helix-families` (a collection of helices that make up a structure) that you can open directly in `helix-FFT`.

### B-DNA - [open model](https://nemoandrea.github.io/helixiser/#name=B-DNA&m=1###&radius=1&rise=0.34&frequency=10&unit_size=0.18&radius=1&rise=0.34&frequency=10&unit_size=0.18&rotation=143)

*The default example in `helix-FFT`, the diffraction pattern of the `double` helix features the characteristic missing layer-line (@ n=3) as observed experimentally in Rosalind Franklin's famous 'Photo 51'*

## Protein Filaments

### F-Actin - [open model](https://nemoandrea.github.io/helixiser/#name=F-actin&n=6&m=3&s=0.19###&radius=2.5&rise=2.75&frequency=2.166&unit_size=3&offset=0&rotation=0&handedness=right)

*A ubiquitous helical filament in cells, F-actin is a `single` helix that can, due to the large angle between subunits, be seen as two slowly turning helices.*

### Microtubule (13 Protofilaments) - [open model](https://nemoandrea.github.io/helixiser/#name=Microtubule%20(13PF)&n=14&m=1&s=0.13###&radius=12.5&rise=0.946&frequency=13&unit_size=2.5&offset=0&handedness=right&radius=12.5&rise=0.946&frequency=13&unit_size=2.5&offset=4.1&handedness=right&radius=12.5&rise=0.946&frequency=13&unit_size=2.5&offset=8.2&handedness=right)

*Another common helical filament in cells, Microtubules are not **quite** helical, but are close enough for the analytic solution to reproduce experimental data. Microtubules of this variety can be seen as three-start helices (i.e. three helices).* 

### Microtubule (14 Protofilaments) - [open model](https://nemoandrea.github.io/helixiser/#name=Microtubule%20(14PF)&n=14&m=1&s=0.13###&radius=13.46&rise=0.946&frequency=13.9&unit_size=2.5&offset=0&rotation=0&handedness=right&radius=13.46&rise=0.946&frequency=13.9&unit_size=2.5&offset=4.38&rotation=1&handedness=right&radius=13.46&rise=0.946&frequency=13.9&unit_size=2.5&offset=8.766&rotation=2&handedness=right)

>:exclamation: The protofilament skew angle is not exact for this model (it's exaggerated). It should be +0.73 [1]

*the 14 protofilament MT also makes a three start helix, but to accommodate the extra protofilament, the number of subunits per pitch is no longer an integer value, and the whole lattice starts to slowly twist (tilt of the MT 'seam'). For the 14PF MT, the seam makes a **right** handed helix.*  

### Microtubule (12 Protofilaments) - [open model](https://nemoandrea.github.io/helixiser/#name=Microtubule%20(12PF)&n=14&m=1&s=0.13###&radius=11.54&rise=0.946&frequency=12.1&unit_size=2.5&offset=0&rotation=0&handedness=right&radius=11.54&rise=0.946&frequency=12.1&unit_size=2.5&offset=3.82&rotation=-0.85&handedness=right&radius=11.54&rise=0.946&frequency=12.1&unit_size=2.5&offset=7.63&rotation=-1.7&handedness=right)

> :exclamation: The protofilament skew angle is not exact for this model (it's exaggerated). It should be -0.86 [1]

*the 12 protofilament MT also makes a three start helix, but to accommodate one less protofilament, the number of subunits per pitch is no longer an integer value, and the whole lattice starts to slowly twist (tilt of the MT 'seam'). For the 12PF MT, the seam makes a **left** handed helix.*  

## Viruses



### Tobacco Mosaic Virus (TMV) [open model](https://nemoandrea.github.io/helixiser/#name=Tobacco%20Mosaic%20Virus%20&n=20&m=1&s=0.6###&radius=7&rise=0.114&frequency=16.333&unit_size=0.7&offset=0&rotation=0&handedness=right&radius=4&rise=0.114&frequency=16.333&unit_size=1&offset=0.75&rotation=0&handedness=right&radius=5&rise=0.114&frequency=16.333&unit_size=0.7&offset=0.5&rotation=0&handedness=right&radius=6&rise=0.114&frequency=16.333&unit_size=0.7&offset=0.25&rotation=0&handedness=right)

*Single stranded RNA plant virus. Quite a bit of liberty has been taken in this example with the 'densities' of the protein. Parameters based on [3]*

### Filamentous Archea viruses [2]

*Double stranded DNA viruses that infect archaeal hosts. Their archeal hosts are hyperthermophiles (i.e. live in extremely hot environments). The DNA is maintained in A-form inside the subunits. The DNA is not considered in the models below, which simply show the protein subunits that make up the filament. For more details on the biology, consult [2].*
* SIRV2 (archea virus) [open model](https://nemoandrea.github.io/helixiser/#name=SIRV2&n=20&m=1&s=0.65###&radius=6.1&rise=0.291&frequency=14.6&unit_size=1.7)
* SSRV1 (archea virus) [open model](https://nemoandrea.github.io/helixiser/#name=SSRV1&n=20&m=1&s=0.65###&radius=6.1&rise=0.294&frequency=14.66&unit_size=1.7)
* SFV1 (archea virus) [open model](https://nemoandrea.github.io/helixiser/#name=SFV1&n=20&m=1&s=0.65###&radius=7.5&rise=0.276&frequency=17.14&unit_size=1.8) 
* PFV2 (archea virus) [open model](https://nemoandrea.github.io/helixiser/#name=PFV2&n=20&m=1&s=0.65###&radius=6.7&rise=0.286&frequency=15.70&unit_size=1.8)
* AFV1 (archea virus) [open model](https://nemoandrea.github.io/helixiser/#name=AFV1&n=20&m=1&s=0.39###&radius=4.2&rise=0.46&frequency=9.3&unit_size=1.9)
* SIFV (archea virus) [open model](https://nemoandrea.github.io/helixiser/#name=SIFV&n=20&m=1&s=0.39###&radius=4.2&rise=0.548&frequency=9.34&unit_size=2)

## References for model parameters

```
[1] Chrétien, Denis, et al. "Determination of microtubule polarity by cryo-electron microscopy." Structure 4.9 (1996): 1031-1040.
[2] Wang, Fengbin, et al. "Structures of filamentous viruses infecting hyperthermophilic archaea explain DNA stabilization in extreme environments." Proceedings of the National Academy of Sciences 117.33 (2020): 19643-19652.
[3] Gelderblom, Hans R. "Structure and classification of viruses." Medical Microbiology. 4th edition. University of Texas Medical Branch at Galveston, 1996.
```



