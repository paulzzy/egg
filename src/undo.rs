use std::{
    collections::HashSet,
    iter::{repeat, zip},
};

use log::{debug, info, trace};
use symbol_table::GlobalSymbol;

use crate::{Analysis, AstSize, EClass, EGraph, ENodeOrVar, Id, Instant, Language, Rewrite, Subst};

/// TODO: good docs and an example
///
/// Appliers in `rewrites` must return `Some` for `get_pattern_ast`. In practice
/// this means that the applier should be a `Pattern`.
pub fn undo_rewrites<'a, L: Language + 'a, N: Analysis<L> + 'a>(
    egraph: &mut EGraph<L, N>,
    rewrites_with_substs: impl IntoIterator<Item = (&'a Rewrite<L, N>, &'a Vec<Subst>)>,
    roots: &Vec<Id>,
) -> Result<(), String> {
    if egraph.are_explanations_enabled() {
        todo!("Undoing rewrites with explanations enabled is not supported");
    }

    // let allowed_rewrites: [GlobalSymbol; 1] = ["associate-*r*".into()];

    // let rewrites_with_substs = rewrites_with_substs
    //     .into_iter()
    //     .filter(|(rewrite, _)| allowed_rewrites.contains(&rewrite.name))
    //     .collect::<Vec<_>>();

    info!("Undoing rewrites with roots {roots:?}");

    trace!("E-Graph before undoing: {:?}", egraph.dump());

    let mut enode_counter: u32 = 0;
    for (rewrite, all_substs) in rewrites_with_substs {
        info!("Undoing rewrite {}", rewrite.name);

        let pattern_ast = rewrite
            .applier
            .get_pattern_ast()
            .expect("applier must support `get_pattern_ast`, such as `Pattern`");

        // if pattern_ast.len()
        //     == rewrite
        //         .searcher
        //         .get_pattern_ast()
        //         .expect("searcher must support `get_pattern_ast`, such as `Pattern`")
        //         .len()
        // {
        //     // Avoid undoing rewrites where the RHS is contained in the LHS, because it may result
        //     // in a term with no leaf e-nodes (so the term never "terminates")
        //     log::warn!("Skip undoing rewrite {} because the RHS is shorter than and may be contained in the LHS", rewrite.name);
        //     continue;
        // }

        info!("Undoing rewrite {}", rewrite.name);
        let undo_rewrite_time = Instant::now();

        let total_len = all_substs.len();

        for subst in all_substs.iter().skip(1) {
            let removed = remove_top_enode(egraph, pattern_ast.as_ref(), subst)?;
            enode_counter += removed as u32;

            if removed {
                {
                    // Check that all roots have at least one ground term (i.e. have a best e-class)
                    for root in roots {
                        info!("checking root {:?}", root);
                        use crate::Extractor;
                        let _ = Extractor::new(egraph, AstSize).find_best(*root);
                    }
                }
            }
        }

        info!(
            "Finished undoing rewrite {} in {} seconds with {total_len} matches",
            rewrite.name,
            undo_rewrite_time.elapsed().as_secs_f64()
        );
    }
    info!("Removed {enode_counter} e-nodes");

    let eclass_counter = remove_unreachable(egraph, roots.clone());
    info!("Removed {eclass_counter} e-classes");

    trace!("E-Graph after undoing: {:?}", egraph.dump());

    Ok(())
}

fn remove_top_enode<L: Language, N: Analysis<L>>(
    egraph: &mut EGraph<L, N>,
    pattern_ast: &[ENodeOrVar<L>],
    subst: &Subst,
) -> Result<bool, String> {
    let (top_enode, children) = match pattern_ast.split_last() {
        Some((top_enode, children)) => (top_enode, children),
        None => return Err("pattern_ast should not be empty".to_string()),
    };

    let mut id_buf: Vec<Id> = vec![0.into(); children.len()];
    for (i, enode_or_var) in children.iter().enumerate() {
        let id = match enode_or_var {
            ENodeOrVar::Var(var) => *subst
                .get(*var)
                .expect("substitution should contain variable"),
            ENodeOrVar::ENode(enode) => {
                let instantiated_enode = enode
                    .clone()
                    .map_children(|child| id_buf[usize::from(child)]);

                match egraph.lookup(instantiated_enode) {
                    Some(eclass_id) => eclass_id,
                    None => {
                        return Ok(false);
                    }
                }
            }
        };
        id_buf[i] = id;
    }

    let mut top_enode_instantiated = match top_enode {
        ENodeOrVar::Var(_var) => {
            // Rewrite is of the form "(...) => (?a)", which cannot be undone because there is no
            // e-node to undo, only two e-classes that were unioned. Since a union does not affect
            // the cost of e-matching, there is no need to undo it. TODO: is this right?
            return Ok(false);
        }
        ENodeOrVar::ENode(enode) => enode
            .clone()
            .map_children(|child| id_buf[usize::from(child)]),
    };

    let eclass_id = match egraph.lookup(&mut top_enode_instantiated) {
        Some(eclass_id) => eclass_id,
        None => {
            return Ok(false);
        }
    };

    fn grounded<L: Language, N: Analysis<L>>(
        eclass: &EClass<L, N::Data>,
        excluded: &L,
        egraph: &EGraph<L, N>,
    ) -> bool {
        fn helper<L: Language, N: Analysis<L>>(
            eclass: &EClass<L, N::Data>,
            excluded: &L,
            egraph: &EGraph<L, N>,
            visited: &mut HashSet<Id>,
        ) -> bool {
            if visited.contains(&eclass.id) {
                // Avoid following cycles
                return false;
            }

            let mut iterator = eclass.nodes.iter().filter(|&enode| enode != excluded);

            if iterator.any(|enode| enode.is_leaf()) {
                return true;
            }

            visited.insert(eclass.id);

            iterator.any(|enode| {
                enode
                    .children()
                    .iter()
                    .all(|id| helper(&egraph[*id], excluded, egraph, visited))
            })
        }

        helper(eclass, excluded, egraph, &mut HashSet::new())
    }

    // Return early if undoing the top e-node would result in its e-class containing no ground term
    // (i.e. cannot be extracted)
    if !grounded(&egraph[eclass_id], &top_enode_instantiated, egraph) {
        return Ok(false);
    }

    let eclass = &mut egraph[eclass_id];
    match eclass.nodes.binary_search(&top_enode_instantiated) {
        Ok(idx) => {
            eclass.nodes.remove(idx);
            let id = egraph.memo.remove(&top_enode_instantiated);
            debug_assert_eq!(egraph.find(id.unwrap()), eclass_id);
            debug!("Removed e-node {top_enode_instantiated:?} from e-class {eclass_id:?} because of pattern {pattern_ast:?}");
        }
        Err(_) => {
            debug!(
                "Already removed top e-node {top_enode_instantiated:?}: {top_enode:?} from e-class {eclass_id:?}"
            );
        }
    };

    Ok(true)
}

pub fn remove_unreachable<L: Language, N: Analysis<L>>(
    egraph: &mut EGraph<L, N>,
    roots: impl IntoIterator<Item = Id>,
) -> u32 {
    let mut visited_eclasses: HashSet<Id> = HashSet::new();
    let mut visited_enodes: HashSet<Id> = HashSet::new();
    let mut dfs_stack: Vec<(Id, &L)> = roots
        .into_iter()
        .flat_map(|id| zip(repeat(egraph.find(id)), &egraph[id].nodes))
        .collect();

    while let Some((eclass_id, enode)) = dfs_stack.pop() {
        visited_eclasses.insert(eclass_id);
        visited_enodes.insert(*egraph.memo.get(enode).unwrap());

        let children = enode
            .children()
            .iter()
            // Avoid following cycles
            .filter(|id| !visited_eclasses.contains(id))
            .flat_map(|id| zip(repeat(*id), &egraph[*id].nodes));

        dfs_stack.extend(children);
    }

    let mut eclass_counter = 0;
    let classes = &mut egraph.classes;
    let memo = &mut egraph.memo;

    classes.retain(|id, eclass| {
        if visited_eclasses.contains(id) {
            eclass.parents.retain(|id| visited_enodes.contains(id));
            true
        } else {
            for enode in &eclass.nodes {
                debug!("Removing e-node {enode:?} because it's in unreachable e-class {id:?}");
                memo.remove(enode);
            }
            eclass_counter += 1;
            debug!(
                "Removing unreachable e-class {id:?} with {} e-nodes",
                eclass.nodes.len()
            );
            false
        }
    });
    for classes in egraph.classes_by_op.values_mut() {
        classes.retain(|id| visited_eclasses.contains(id));
    }

    eclass_counter
}
