---
title: Element Operators
layout: default
nav_order: 3
---

# Element Operators

While explicitly defining elements in an asset is fine for some cases, there are instances when more logic is required. Element Operators allow for the element list to be manipulated dynamically

Consider the following asset:

```yaml
assets:
  3d:
    character:
    - suzanneA: 
        departments:
          model:
            - mesh
          rigging:
            - rig
          anim:
            - anim_cache
```

This configuration is defining a simple rigged character model. In this instance, it doesn't make sense for departments other than `anim` to load the rig, instead most other departments should be loading the `anim_cache` which `anim` exports. We can use element operators to achieve this.

```yaml
assets:
  3d:
    character:
    - suzanneA: 
        departments:
          model:
            - mesh
          rigging:
            - !department_is anim: rig
          anim:
            - !department_is_not anim: anim_cache
```

{: .note }
> While the `rig` element will only show up for *load* by `anim` department, since the element is owned by `rigging`, the element will still show up for `rigging` department during *export*.

Here we are using the [department_is](./department_is) and [department_is_not](./department_is_not) element operators, which will filter their element based on the current department.

Loading this config from the `anim` department will result in elements `mesh` and `rig`, wheras any other department will receive `mesh` and `anim_cache`
