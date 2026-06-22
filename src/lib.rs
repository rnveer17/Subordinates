// ============================================================
// Subordinates – Two Distinct Implementations
// ============================================================

// ============================================================
// Approach 1: Recursive DFS
// ============================================================
// Traverses the tree from the root, computes subtree sizes.
// Each node's subordinate count is its subtree size - 1.
//
// Time complexity: O(n) | Space complexity: O(n)
// ============================================================
pub fn subordinates_dfs(children: &[Vec<usize>]) -> Vec<usize> {
    let n = children.len();
    let mut subordinates = vec![0; n];

    fn dfs(node: usize, children: &[Vec<usize>], subordinates: &mut [usize]) -> usize {
        let mut count = 0;
        for &child in &children[node] {
            count += dfs(child, children, subordinates) + 1;
        }
        subordinates[node] = count;
        count
    }

    dfs(1, children, &mut subordinates);
    subordinates
}

// ============================================================
// Approach 2: Iterative DFS (Stack-based) – Safe for deep trees
// ============================================================
// Uses an explicit stack to avoid recursion overhead.
// Processes nodes in post-order to compute subtree sizes.
//
// Time complexity: O(n) | Space complexity: O(n)
// ============================================================
pub fn subordinates_iterative(children: &[Vec<usize>]) -> Vec<usize> {
    let n = children.len();
    let mut subordinates = vec![0; n];
    let mut order = Vec::new();
    let mut stack = vec![1];

    // Step 1: Traverse tree (pre-order)
    while let Some(node) = stack.pop() {
        order.push(node);
        for &child in &children[node] {
            stack.push(child);
        }
    }

    // Step 2: Process in reverse order (post-order)
    for &node in order.iter().rev() {
        let mut count = 0;
        for &child in &children[node] {
            count += subordinates[child] + 1;
        }
        subordinates[node] = count;
    }

    subordinates
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let mut children = vec![Vec::new(); 6];
        children[1].push(2);
        children[1].push(3);
        children[2].push(4);
        children[3].push(5);

        let result = subordinates_dfs(&children);
        assert_eq!(result[1], 4);
        assert_eq!(result[2], 1);
        assert_eq!(result[3], 1);
        assert_eq!(result[4], 0);
        assert_eq!(result[5], 0);
    }

    #[test]
    fn test_star_tree() {
        let mut children = vec![Vec::new(); 5];
        children[1].push(2);
        children[1].push(3);
        children[1].push(4);

        let result = subordinates_dfs(&children);
        assert_eq!(result[1], 3);
        assert_eq!(result[2], 0);
        assert_eq!(result[3], 0);
        assert_eq!(result[4], 0);
    }

    #[test]
    fn test_chain() {
        let mut children = vec![Vec::new(); 5];
        children[1].push(2);
        children[2].push(3);
        children[3].push(4);

        let result = subordinates_dfs(&children);
        assert_eq!(result[1], 3);
        assert_eq!(result[2], 2);
        assert_eq!(result[3], 1);
        assert_eq!(result[4], 0);
    }

    #[test]
    fn both_algorithms_give_same_result() {
        let mut children = vec![Vec::new(); 6];
        children[1].push(2);
        children[1].push(3);
        children[2].push(4);
        children[3].push(5);

        let rec = subordinates_dfs(&children);
        let iter = subordinates_iterative(&children);
        assert_eq!(rec, iter);
    }
}