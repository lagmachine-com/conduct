---
title: depends
layout: default
parent: Element Operators
nav_order: 3
---

# Depend (Element Operator)

## Example Configuration

```yaml
assets:
  3d:
    character:
    - suzanneA: 
        departments:
          model: 
            - mesh
    - suzanneInstancer: 
        departments:
          layout: 
            - !depend(suzanneA): instancer
```

The `depend` operator allows you to define a dependency from one asset to another, where the key is the element name, and the values are the asset's it depends on. This dependency will ensure that the `suzanneA` asset is loaded before `suzanneInstancer`.

An element can depend on multiple assets:

```yaml
assets:
  3d:
    character:
    - suzanneA: 
        departments:
          model: 
            - mesh
    - suzanneB: 
        departments:
          model: 
            - mesh
    - suzanneInstancer: 
        departments:
          layout: 
            - !depend(suzanneA;suzanneB): instancer
```