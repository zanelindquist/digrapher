# Point Movement

## Description
This is part of the feature/pointInteraction PR that seeks to let a user select points on the graph and drag them to other locations.

## Overview
A user should be able to click on a point to select it and drag a selected point to a new location on the canvas

## External component changes
| Component | Issue | Solution |
|-----------|-------|----------|
| `<Graph/>`| Freezing canvas movement when a point is being clicked and dragged | Pass a prop from `<Graph/>` to `<DigraphCanvas/>` that digraph canvas can use to freeze movement when a point is in the process of being clicked and dragged|
| `Point` | Tell if the pointer is on a point | Create a new `in_proximity(x: i32, y: i32)` method that uses the radius and `distance_to()` method to return a boolean |


## Challenges
| Component | Challenge | Solution |
| --------- | -------- | --------|
| `<DigraphCanvas/>` | 

## Logical vs Visual points
`Point` objects will now use logical coordinates. Then, at render time, the visual coordinates will be produced using the `CanvasPosition`.

- Logical coordinates
    - Are normalized in a -1 to 1 x, y plane

## Algorithm
- `<DigraphCanvas/>` Variable Changes
    - Variable changes
        + Add `selected_point: UseStateHandle<<Option<&Point>>` for tracking
        + Add `last_pos: UseStateHandle<(i32, i32)>` for keeping track of pointer movements
        - Modify `points: PointVector` -> `points: UseStateHandle<PointsVector>`
    - New functions
        - `onpointerdown`
        - `onpointerup`
        - `onpointermove`
- Logic
    - => user pointer down
        - For every point, call `in_proximity(pointer_x, pointer_y)` to see if a point is being selected
            - True
                - Set the selected point to this point
                - Set the `<Graph/>`'s passed interrupt prop to freeze canvas movement
            - False: do nothing
    - => user pointer up
        - Set the `<Graph/>`'s passed interrupt to unfreeze canvas movement
    - => use pointer move
        - If a point is being selected
            - Use the `last_pos` variable and the pointer position to create a change offset
            - Translate the visual pixels into logical pixels based on the `props.position.zoom`'s current scale
            - Set a new `last_pos`
            - Add the change offset to the selected point's coordinates
