---
title: Shots
layout: default
nav_order: 4
---

# Shots

The shot system provides a simple way to manage different sequences in your project. In a film / TV context, a sequence represents a series of shots which together create a narrative element. 

The shot system is flexible, allowing you to divide your project in to as many different sequences and sub-sequences as you desire

{: .note }
> For more insight in to how some studios name shots in the VFX industry, see Netflix's [VFX Shot and Version Naming Recommendations](https://partnerhelp.netflixstudios.com/hc/en-us/articles/360057627473-VFX-Shot-and-Version-Naming-Recommendations)

## Example Config

Following and episode / sequence / shot pattern you could configure your shots like so:

```dart
shots:
  "103": 
    DE:
      - "0010"
      - "0020"
      - "0030"
  "104":
    TCC:
      - "0010"
      - "0020"
      - "0030"
      - "0040"
    GK:
      - "0010"
      - "0020"
      - "0030"
      - "0040"
```

{: .important }
> Note that entries which consist of only numbers are entered as strings


This configuration will result in the following shot list:

```json
"103/DE/0010"
"103/DE/0020"
"103/DE/0030"
"104/TCC/0010"
"104/TCC/0020"
"104/TCC/0030"
"104/TCC/0040"
"104/GK/0010"
"104/GK/0020"
"104/GK/0030"
"104/GK/0040"
```