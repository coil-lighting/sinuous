/* Placeholder for mixer and mixdown algorithm

In past LD50 versions, mixdown went something like this:

- The mixer object hold the master copy of the device tree.

- Mixer layers are arranged in a tree, like in Photoshop.

- Each layer has a private copy of the device (sub)tree of interest. Maybe one layer works on just the focus rail on just one Technobeam, and another layer works on all the fixtures.

- For each layer, the mixer sets input parameters (or at least modified params).

- At this point the Mixer might copy the master device tree into a background state to seed the blending process below. (This is an optional debug-mode step, and we could blend directly into the master buffer, but it can help during development to be able to compare the results to the original at every point during mixdown.)

- Then all the layers are commanded to animate(). In our case, they should animate in parallel, while the Mixer idles or blocks.

- As animation completes, each layer emits an output and then halts. A layer performs no processing until its next event arrives.

- As outputs arrive, the Mixer then applies blenders between pairs of outputs, always in reverse order of dependency, rolling up results from the leaves up the tree to the root.

- Along with blending, there may be post-effects applied; this might be where submaster fader scaling happened, or it might have happened in the layer. I can't remember how I did it last time, but this time it ought to be a generic, Mixer-level function so that it only has to be implemented once, and a layer can't cheat its submaster.

-  When the tree is reduced to a single resulting new master node, the master fader scaling is applied, and any top-level effects. (Since the Mixer is a tree, this step is actually implied by the previous step.)

- The mixer's owner then renders the master copy of the device tree to DMX using the now-familiar algorithm.

- The mixer's owner does anything else required, like sending a copy of the device tree or the dmx buffers to a logger, recorder, debugger, or bookmarking service.

- If you optionally copied the device tree in step 5 to assist debugging, then copy it back down. (If you have no external references into it, you might just swap references and dispose of the old one.)

Come to think of it, if you do copy the results back into the master, then you could hand off ownership of the temp result tree to a logging or bookmarking service running in a downstream task. Easy to do in C... who knows what tribute Rust requires.

When the device tree changes because you repatch the show, the change event is processed between frames in the main runloop. The mixer distributes copies of the new device (sub)trees to the appropriate layers. The layers are then responsible for transferring state from their old device trees (and any other internal data structures) into a new copy. It's pub/sub, with a single-threaded event-driven main animation loop built around the mixer, with multi-threaded fanout for the animation step.

(In the old LD50, a second pass was then made to animate the GUI, but it'll happen differently this time.)

*/
