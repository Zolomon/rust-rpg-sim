rust-rpg-sim
============

An RPG simulator written in Rust


# Design
A Hero wakes up one day at his local town and has grown old enough to go exploring. 

The Hero leaves the town located at <0,0> in a random direction for exploration.

The Hero will meet random events while exploring: 

* Find empty rooms
* Find rooms with undesirable *Items*
* Find rooms with desirable *Items*
* Find rooms with NPCs
* Find rooms with *Enemies*

# Combat

During combat the entities will use different **effects**. 

# Equipment

Entities have slots to be filled with equipment. 

# Skills/Effects

Entities have slots to be filled with effects.

An effect is *instant* or *channeled* and can have a *cooldown* before it can be reused, otherwise it disappears.
