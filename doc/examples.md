# Example helices

Below are some example `helix-families` (a collection of helices that make up a structure) that you can open directly in `helix-FFT`.

### B-DNA - [open model](https://nemoandrea.github.io/helixiser/#name=B-DNA&radius=1&rise=0.34&frequency=10&unit_size=0.18&radius=1&rise=0.34&frequency=10&unit_size=0.18&rotation=143)

*The default example in `helix-FFT`, the diffraction pattern of the `double` helix features the characteristic missing layer-line (@ n=3) as observed experimentally in Rosalind Franklin's famous 'Photo 51'*



### F-Actin - [open model](https://nemoandrea.github.io/helixiser/#name=F-actin&radius=2.5&rise=2.75&frequency=2.166&unit_size=3&offset=0&rotation=0&handedness=right)

*A ubiquitous helical filament in cells, F-actin is a `single` helix that can, due to the large angle between subunits, be seen as two slowly turning helices. **This model should be viewed with at least `n=6`, `m=3`***

### Microtubule (13 Protofilaments) - [open model](https://nemoandrea.github.io/helixiser/#name=Microtubule%20(13PF)&radius=12.5&rise=0.946&frequency=13&unit_size=2.5&offset=0&handedness=right&radius=12.5&rise=0.946&frequency=13&unit_size=2.5&offset=4.1&handedness=right&radius=12.5&rise=0.946&frequency=13&unit_size=2.5&offset=8.2&handedness=right)

*Another common helical filament in cells, Microtubules are not quite helical, but are close enough for the analytic solution to reproduce experimental data. Microtubules of this variety can be seen as three-start helices (i.e. three helices).* **This model should be viewed with at least`n=13` and `m=1`**

### Microtubule (14 Protofilaments) - [open model](https://nemoandrea.github.io/helixiser/#name=Microtubule%20(14PF)&radius=13.46&rise=0.946&frequency=13.9&unit_size=2.5&offset=0&rotation=0&handedness=right&radius=13.46&rise=0.946&frequency=13.9&unit_size=2.5&offset=4.38&rotation=1&handedness=right&radius=13.46&rise=0.946&frequency=13.9&unit_size=2.5&offset=8.766&rotation=2&handedness=right)

>:exclamation: The protofilament skew angle is not exact for this model (it's exaggerated). It should be +0.73 [1]

*the 14 protofilament MT also makes a three start helix, but to accommodate the extra protofilament, the number of subunits per pitch is no longer an integer value, and the whole lattice starts to slowly twist (tilt of the MT 'seam'). For the 14PF MT, the seam makes a **right** handed helix.*  **This model should be viewed with at least `n=14` and `m=1 `**

### Microtubule (12 Protofilaments) - [open model](https://nemoandrea.github.io/helixiser/#name=Microtubule%20(12PF)&radius=11.54&rise=0.946&frequency=12.1&unit_size=2.5&offset=0&rotation=0&handedness=right&radius=11.54&rise=0.946&frequency=12.1&unit_size=2.5&offset=3.82&rotation=-0.85&handedness=right&radius=11.54&rise=0.946&frequency=12.1&unit_size=2.5&offset=7.63&rotation=-1.7&handedness=right)

> :exclamation: The protofilament skew angle is not exact for this model (it's exaggerated). It should be -0.86 [1]

*the 12 protofilament MT also makes a three start helix, but to accommodate one less protofilament, the number of subunits per pitch is no longer an integer value, and the whole lattice starts to slowly twist (tilt of the MT 'seam'). For the 12PF MT, the seam makes a **left** handed helix.*  **This model should be viewed with at least `n=12` and `m=1 `**


## References for model parameters

```
[1] Chrétien, Denis, et al. "Determination of microtubule polarity by cryo-electron microscopy." Structure 4.9 (1996): 1031-1040.
```



