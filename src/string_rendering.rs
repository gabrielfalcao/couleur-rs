#[cfg(test)]
mod tests  {

    #[test]
    pub fn test_cluster_together_non_renderable_color_nodes() -> Result<()>{
        // Given a list of N random Node variants In order to render
        // colored ANSI test I want to agglutinate together all nodes
        // into `Node::RenderableColor` nodes while advancing cursor
        // which keeps track of nodes from 0 to N
        //
        // If the current cursor position is already a
        // `Node:RenderableColor` that precedes other non-text nodes,
        // a few outcomes are possible:
        //
        // 1. The RenderableNode is whole (contains all non-required
        // fields such as Contrast) already set to non-null value.
        //
        // 2. The RenderableNode is not whole and, coincidentally does
        // not have a Layer node defined.
        //
        // 3. Given that the cursor is currently pointing at said
        // RenderableNode at position 4 and that the array of nodes
        // has 7 items where item at position 5 is a Text and position
        // 6 is a Layer::BG node, then once the cursor moves to
        // position 5 the RenderableNode "absorbs" the Layer node at
        // position 5, shortening the final array of nodes to length
        // 6.
        //
        // A Final Array of nodes can only have the following few
        // types of nodes: `RenderableColor`, `Text` and `Reset`
        //
        Ok(())
    }

}
