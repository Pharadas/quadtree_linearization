# Quadtree Linearization
Method for linearization and compression of quadtrees

This project aims to linearize, compress and travel through n-trees by using bit manipulation and grouping similar information together.  

# N-Tree Codification

To store the 'feature' of a quad this algorithm stores all posibles states in a list, which is then indexed with N bits necessary.  
There are a few cases that can be represented:  
- 0 This means that the quad is empty, and it will not store anything within it.  
- 10 This means that the quad does contain something, but it is not homogeneous, so we will have to store the inner quads.  
- 11 This means that the quad is homogeneous all the way down, the next N bits will be the index of the feature.  
  
Once we are in the deepest quad, the nomenclature changes:
- 1 This means this quad is not empty and the next N bits will represent the feature.  
- 0 This means that the quad is empty and we should skip to the next one

# N-Tree decodification
To recover a tree from a stream of bits, the algorithm follows the same process backwards.

## Examples:
![image](https://user-images.githubusercontent.com/60682906/187241255-27272178-730f-4512-8566-809b14f7c16d.png)  
  
The representation (Formatted for easier reading
```
10
  10 <- Upper left quad                                        10 <- Upper right quad       0 <- Lower left quad 0 <- Lower right quad
    0 10                    0 10                                 0 10                   0 0
        0 0 0 10                1100 10                    0 0       0 0 10           0
                 0 1100 0 0             0 0 0 10                           0 0 0 1111
                                                0 111 100 0                           
```
