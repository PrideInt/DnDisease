# Dungeons 'n Disease
Disease combat game (as the disease like Plague Inc.) with 5e DnD rules.

> [!IMPORTANT]
> ## IN DEVELOPMENT
> [![Development](https://github.com/PrideInt/DnDisease/actions/workflows/rust.yml/badge.svg)](https://github.com/PrideInt/DnDisease/actions/workflows/rust.yml)

## Overview

In 2019, we've been plagued by SARS-CoV-2, rendering the globe vulnerable to an
ancient, pseudoscientific, probably magical pathogen that has been sleeping inside frozen glaciers.
With the planet warming up, this virus, or bacteria, whatever **it** is, has been awakened and
unleashed onto every living being on this planet. 

Where did it come from? Is it Arthurian? Older? Venetian? Assyrian? Well, however it came to be,
it has been forged with abilities to recombine its DNA, generate electrical fields, charm a population,
and so much more.

It can wipe out everyone. Everything.

So why did it choose you? Why do **you** get to play this game? **It** may beckon your call, but **we** will make 
your game ***completely fucking nauseating***.

## Initiation

Run `cargo run --bin dndisease` in terminal to begin the game.

## Map

Potential view of the game's map (when crossing over to other countries and nations) implemented by a graph.

<img src="https://github.com/user-attachments/assets/25dd187b-96da-48c0-be71-e67a43fa6ec9" width="75%">

**Nodes** are defined by country. Each node represents a country/nation.

**Edge weights** are defined by distance of player's current location. Can only move within one edge. Can only
return to and from node 1 and node 2.
