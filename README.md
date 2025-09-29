Sketch of destructive e-graph rewrites, forked from
[egg](https://github.com/egraphs-good/egg).

See Paul's [EGRAPHS workshop talk](https://pldi25.sigplan.org/details/egraphs-2025-papers/7/Destructive-E-Graph-Rewrites)
at PLDI 2025 for more details.

## Future Work

- Currently, when a destructive rewrite is applied, we only support removing the
  corresponding e-node from the e-graph. This means that unrelated terms may be
  affected. Ideally a destructive rewrite would only remove terms that are
  directly involved. This would probably require more information in the
  e-graph, e.g. tracking the origin of each term. `egg` contains a second
  e-graph for explanations which apparently has this information.
- [`egglog`](https://github.com/egraphs-good/egglog) has a `delete` function.
- Destructive rewrites could take cost information into account to prioritize
  rewrites.
- Prof. Pavel Panchekha (from a discussion at PLDI 2025):
  - Try removing nodes older than a certain number of iterations.
  - Extracting is `(egraph -> term)`, while destructive rewrites are
    `(egraph -> egraph)`. Can be thought of as garbage collection.
