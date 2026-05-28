# Relation Editing

This PR will add the ability to create and delete points and edges.

## Considerations
- The main question is how to make the sidebar and the graph interact and communicate with each other
- We will also think about how the GraphTheoryRelationManager fits into all of this, because we are adding new nodes and points
    - Points can't just be held logically, their positions need to be stateful so that we keep the same visual layout

## Changes
- `GraphTheoryRelationManager`
    - Make point edits in `<DigraphCanvas/>` persist, so we need to call an actual setter on the `GraphTheoryRelationManager`
    - Add a `create_point(x, y)` method that assigns the point to a certain layer
    - On edge connection, we can reset the nodes, but we don't want to reset point positions