# Delta modularity calculation (singleton)

> **🚨 This page is a work in progress. TODO: Maybe add a graphic here to explain the various variables.**

In the modularity optimization phase, we only rely on *local* information. Recalculating the global modularity value for every possible neighbor of each vertex would significantly degrade the performance of the Louvain algorithm. This is why [Blondel et al.](https://perso.uclouvain.be/vincent.blondel/publications/08BG.pdf) employ a delta modularity formula that can be applied to millions of nodes. In this section, we will explain and derive the formula and show how it can be simplified for usage in a program.

We have seen [here](../modularity/formula.md#alternative-formulation) that the modularity $Q(\Cs)$ for a partition $\Cs$ is given by:

$$
Q(\Cs) = \frac{1}{2m} \sum_{c\in \Cs} \left( \Sigma_c
- \frac{\left(\Sigma_{\hat{c}}\right)^2}{2m} \right)
$$

**We are only intrested in the contribution of a single community $c$ to the overall modularity**. With a slight abuse of notation, let $Q(c)$ denote modularity of community $c$[^q-notation-abuse], such that $Q(\Cs) = \sum_{c\in \Cs} Q(c)$. Therefore, for the modularity of a community $c$, we get:

$$
\boxed{
Q(c) = \frac{\Sigma_c}{2m}
- \left( \frac{\Sigma_{\hat{c}}}{2m} \right)^2
}
$$


---


Next, we look closely at the process of "moving" a vertex $u$ to the community of one of its neighbors $\Neigh(u)$. We assume the vertex to be located in a singleton community. The formula Blondel et al. presented is for exactly this case, where we remove $u$ from its singleton community and then insert it into the neighbor's community. This is actually only applicable for the first
iteration of the modularity optimization phase where in the beginning of every new pass we deal with singleton communities. The authors state that they use a similar formula to calculate the modularity change when $u$ is removed from any community (also those with more than one vertex in it), but do not reveal the expression used. We will therefore derive the generalized version in the next section.

For now, let us consider the case where we move a vertex $u$ from its singleton community that we will denote by $\singleton{c}_u$ to any other community $c$. The modularity change (delta) is then given by

$$
\Delta Q(u, \singleton{c}_u, c) = Q'(c) - Q(c) - Q(\singleton{c}_u)
$$

where $Q(c)$ is the quality of community $c$ *before* the merge (and hence without vertex $u$ in that community) and $Q'(c)$ is the quality of $c$ *after* the merge (and hence after vertex $u$ was integrated into community $c$). As the singleton community does not exist anymore after the merge, we also subtract the modularity of the singleton community $Q(\singleton{c}_u)$. In the following, let the prime symbol always denote the state of a variable *after* the merge.

With the above formula, this gives us:

$$
\begin{align}
\Delta Q(u, \singleton{c}_u, c)
&= Q'(c) - Q(c) - Q(\singleton{c}_u) \\
% 
&= \biggl[ \frac{\Sigma'_c}{2m}
- \left( \frac{\Sigma'_{\hat{c}}}{2m} \right)^2 \biggr]
- \biggl[ \frac{\Sigma_c}{2m}
- \left( \frac{\Sigma_{\hat{c}}}{2m} \right)^2 \biggr]
- \biggl[ \frac{\Sigma_{\singleton{c}_u}}{2m}
- \left( \frac{k_u}{2m} \right)^2 \biggr]
\end{align}
$$

which is the equation presented in the [original Louvain paper](https://perso.uclouvain.be/vincent.blondel/publications/08BG.pdf).
If we consider that after the merging process, the sum of the weights of the edges between node $u$ and community $c$ -- that we will denote[^notation-edges-between] by $k_u^{\rightarrow c}$ -- now adds to the community $c$ accommodating vertex $u$, we obtain

$$
\Sigma'_c = \Sigma_c + 2 k_u^{\rightarrow c}
$$

Likewise, we find that the weighted vertex degree $k_u$ adds to the total weighted vertex degree of $c$ after the merge, thus:

$$
\Sigma'_{\hat{c}} = \Sigma_{\hat{c}} + k_u
$$

With this, we can simplify our expression for the modularity change. Note that we use $\Sigma_{\singleton{c}_u} = 0$, as there are no edges inside a singleton community $\singleton{c}_u$ since we do not allow self-loops in the original graph.

$$
\begin{align}
\Delta Q(u, \singleton{c}_u, c)
% 
&= \biggl[ \frac{\Sigma_c + 2 k_u^{\rightarrow c}}{2m}
- \left( \frac{\Sigma_{\hat{c}} + k_u}{2m} \right)^2 \biggr]
- \biggl[ \frac{\Sigma_c}{2m}
- \left( \frac{\Sigma_{\hat{c}}}{2m} \right)^2 \biggr]
- \biggl[ \frac{\Sigma_{\singleton{c}_u}}{2m}
- \left( \frac{k_u}{2m} \right)^2 \biggr]\\
% 
&= \biggl[ \frac{\Sigma_c + 2 k_u^{\rightarrow c}}{2m}
- \left( \frac{\Sigma_{\hat{c}} + k_u}{2m} \right)^2 \biggr]
- \biggl[ \frac{\Sigma_c}{2m}
- \left( \frac{\Sigma_{\hat{c}}}{2m} \right)^2
- \left( \frac{k_u}{2m} \right)^2 \biggr]\\
% 
&= \biggl[ \frac{\Sigma_c + 2 k_u^{\rightarrow c}}{2m}
- \left( \frac{\Sigma_{\hat{c}} + k_u}{2m} \right)^2 \biggr]
- \biggl[ \frac{\Sigma_c}{2m}
- \left( \frac{\Sigma_{\hat{c}}}{2m} \right)^2
- \left( \frac{k_u}{2m} \right)^2 \biggr]\\
% 
&= \frac{2 k_u^{\rightarrow c}}{2m}
- \left( \frac{\Sigma_{\hat{c}}}{2m} \right)^2
- 2 \frac{\Sigma_{\hat{c}}}{2m} \frac{k_u}{2m}
- \left( \frac{k_u}{2m} \right)^2
+ \left( \frac{\Sigma_{\hat{c}}}{2m} \right)^2
+ \left( \frac{k_u}{2m} \right)^2 \biggr]\\
% 
&= \frac{k_u^{\rightarrow c}}{m}
- 2 \frac{\Sigma_{\hat{c}} k_u}{m\cdot 2m}\\
% 
&= \frac{1}{m} \biggl(
    k_u^{\rightarrow c} - \frac{\Sigma_{\hat{c}} k_u}{2m}
\biggr)
\end{align}
$$

Finally, we have

$$
\boxed{
    \Delta Q(u, \singleton{c}_u, c)
    \propto k_u^{\rightarrow c} - \frac{\Sigma_{\hat{c}} k_u}{2m}
}
$$

We can ignore the constant $\frac{1}{m}$ since we only compare different modularity increases $\Delta Q(u, \singleton{c}_u, c$ with each other and therefore merely require a relative measure. This saves us one division in the algorithm. After one complete pass, the new global modularity is calculated using the formula from [here](../modularity/formula.md#alternative-formulation)[^modularity-recalculation].


---


Our short formula is used in the algorithm to efficiently calculate the delta modularity gain $\Delta Q$. To remove a vertex $u$ from its previous community $c_u$ or to insert it into a new community $c$, only $\Sigma_c$ and $\Sigma_{\hat{c}}$ have to be updated. $\Sigma_c$ is adjusted for the global modularity calculation and not for the delta modularity $\Delta Q$. Note that we can precalculate the sum of the weights of edges between $u$ and community $c_u$ or $c$ ($k_u^{\rightarrow c_u}$ and $k_u^{\rightarrow c}$) before calling the `remove`- or `insert`-function. This is crucial for the speed of Louvain as $\Delta Q$ has to be computed frequently. As stated above, we also dropped
the factor $\frac{1}{m}$ to save one division. For the calculation of global modularity, we do not omit the factor $\frac{1}{2m}$ in order to obtain the absolute global modularity $Q$.




[^q-notation-abuse]: We can distinguish the two functions by looking at the argument, which is either a community $c$ or a partition $\Cs$, i.e. a set of communities $\Cs = \{c_1, \dots, c_k\}$.

[^notation-edges-between]: This is done in conformity with [these notes on modularity](https://www.hongliangjie.com/notes/modularity.pdf). The arrow does not indicate a direction; we still deal with an undirected graph.

[^modularity-recalculation]: In newer versions of the algorithm, this is not the case anymore and we only use the relative modularity calculation. TODO