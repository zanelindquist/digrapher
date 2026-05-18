# Point Movement

## Description
This is part of the feature/pointInteraction PR that seeks to let a user select points on the graph and drag them to other locations.

## Overview
A user should be able to click on a point to select it and drag a selected point to a new location on the canvas

## External component changes
| Component | Issue | Solution |
|-----------|-------|----------|
| `<Graph/>`| Freezing canvas movement when a point is being clicked and dragged | Pass a prop from `<Graph/>` to `<DigraphCanvas/>` that digraph canvas can use to freeze movement when a point is in the process of being clicked and dragged|
| `Point` | Tell if the pointer is on a point | Create a new `pointer_by(pointer_x: i32, pointer_y: i32)` method that uses the radius and `distance_to()` method to return a boolean |

## Logical vs Visual points
`Point` objects will now use logical coordinates. Then, at render time, the visual coordinates will be produced using a `CanvasPosition` instance.

- Logical coordinates
    - Are centered on the canvas, and scaled by the SCALING_CONSTANT which is defined by the screen width and stuff
- Visual coordinates
    - Are the x, y on the actual canvas
- Pointer coordinates
    - Are based on the absolute position of the pointer in the window in pixels

## Algorithm
- `<DigraphCanvas/>` Variable Changes
    - Variable changes
        + Add `selected_point: UseStateHandle<<Option<&Point>>` for tracking
        - Modify `points: PointVector` -> `points: UseStateHandle<PointsVector>`
    - New functions
        - `onpointerdown`
        - `onpointerup`
        - `onpointermove`
- Logic
    - => user pointer down
        - For every point, call `pointer_by(pointer_x, pointer_y)` to see if a point is being selected
            - True
                - Set the selected point to this point
                - Set the `<Graph/>`'s passed interrupt prop to freeze canvas movement
            - False: do nothing
    - => user pointer up
        - Set the `<Graph/>`'s passed interrupt to unfreeze canvas movement
    - => use pointer move
        - If a point is being selected
            - Get the mouse's coordinates
            - Calculate the point's new logical position using the `canvas_pos` class instance
            - Relpace the old point with the new point
        - Check if a point is being hovered
            
