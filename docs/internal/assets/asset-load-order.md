---
title: Asset Load Order
layout: default
parent: Assets
nav_order: 2
---

# Asset Load Order

{: .no_toc }

## Table of contents
{: .no_toc .text-delta }

1. TOC
{:toc}

---

## Configuring Load Order
The asset load order is an important part of configuration, as it determines which asset elements are available to eachother

The simplest form of a load order configuration is like so:

```yaml
load_order:
- mesh
- anim_cache
- shader_graph
```

Here, each element is explictly ordered, and are loaded from top to bottom. 

This allows `anim_cache` to affect `model`, and `shader_graph` to affect both, `model` and `anim_cache`.

While this is a simple configuration, it doesn't allow us to change load order based on the current context, such as specifying different elements to load based on which department is loading the asset. In order to do this, we need Load Order Operators

## Load Order Operators

A load order operator is simply a piece of extra configuration in the load order, which allows us to have a bit more logic in determining which assets are loaded

One possible use case for this, is in a 3D animation context. When the `animation` department loads an asset, they likely want to load the rig required for animating the character, however most other departments likely dont want the rig, and just want to load the final animation provided by the `animation` department

This can be achieved using `department_switch` operator:

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