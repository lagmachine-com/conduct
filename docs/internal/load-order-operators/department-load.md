---
title: Department Load
layout: default
parent: Load Order Operators
nav_order: 1
---

# Department Load

The Department Load operator loads all available elements, contributed by a specified department

```yaml
assets:
  3d:
    character:
    - suzanneA: 
        departments:
          model:
            - meshA
            - meshB
            - meshC
          lookdev:
            - shader_graph

load_order:
- !department_load model
- shader_graph
```

This configuration will load the all of the `model` departments elements, `meshA`, `meshB`, `meshC` due to the use of `department_load`.

Finally, it will load the remaining element `shader_graph`