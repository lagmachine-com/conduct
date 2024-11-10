---
title: Department Switch
layout: default
parent: Load Order Operators
nav_order: 2
---

# Department Switch

The Department Switch operator allows you to configure which elements are loaded, based on the department which is loading the asset.

```yaml
load_order:
- mesh
- !department_switch
  _: anim_cache
  anim: rig
- shader_graph
```

This configuration will load the `model` element for all departments.

Then, if the loading department is `anim` it will load `rig`, and for all other departments (as specified by the `_` placeholder) it will load `anim_cache`


Finally, the `shader_graph` is loaded for all departments.