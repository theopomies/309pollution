# 308 REEDPIPES
The ambient air quality monitoring in France is ensured by independent associations, members of the ATMO federation, and, on behalf of the State and public authorities, are responsible for implementing means of monitoring.
Why not we? The Lozère market seems easily open for the taking. . .
So, we decide to start a project based on collaborative initiatives like CitoyensCapteurs in order to acquire data. All that’s left to do is to create a little software for viewing the data. . .
We receive the data in triplets (x, y, p) where x and y are the coordinates (presumably integers so it’s sim- pler) on a normal grid and p the pollution level (in percentage). We will consider that the pollution is non- existent on the grid’s other points.
Our program will use Bézier surfaces to smooth out the data and display the value of the pollution level in a point inside the observed area.
## How to build
```sh
$ make re
```

## Examples
```sh
$> cat file.csv
0;0;20
0;1;12
1;0;50
1;1;30
1;2;50
2;2;80
```

```sh
./309pollution 3 file.csv 0 2
0.00
```

```sh
./309pollution 3 file.csv 0.6 2
28.20
```

```sh
./309pollution 3 file.csv 1.3 2
56.55
```

```sh
./309pollution 3 file.csv 1 1.5
33.94
```

```sh
./309pollution 3 file.csv 0.8 0.8
26.11
```
