# Delta modularity calculation (generalized)

> **🚨 This page is a work in progress. TODO: explain why this is needed.**

Here we generalize the delta modularity calculation presented in the [previous section](./delta-modularity.md) for the case where we move a vertex $u$ from any community (not just a singleton community) to any other community $c$. We proceed by removing $u$ from the old community and putting it into its own singleton community. The latter is done, so that the derived Louvain modularity difference formula can be reused, which is only applicable to the case where one moves a vertex from a singleton community to any other community. With this more generalized equation, we fill the gap caused by the absence of this formula in the [original Louvain paper](https://perso.uclouvain.be/vincent.blondel/publications/08BG.pdf). While we do not employ this generalized formula in the actual implementation, it still might be useful for other applications.

First, we calculate the modularity difference when removing a vertex $u$ from its old community $c_u$ (not required to be a singleton community) and inserting it into its own singleton community $\singleton{c}_u$. As before, we prohibit self-loops in the original graph, hence $\Sigma_{\singleton{c}_u}$. We won't be as elaborate this time due to the rationale being very similar:

$$
\begin{align}
\Delta Q(u, c_u, \singleton{c}_u)
% 
&= Q'(c_u) - Q(c_u) + Q(\singleton{c}_u)\\
% 
&= \biggl[ \frac{\Sigma'_{c_u}}{2m}
- \left( \frac{\Sigma'_{\hat{c}_u}}{2m} \right)^2 \biggr]
- \biggl[ \frac{\Sigma_{c_u}}{2m}
- \left( \frac{\Sigma_{\hat{c}_u}}{2m} \right)^2 \biggr]
+ \biggl[ \frac{\Sigma_{\singleton{c}_u}}{2m}
- \left( \frac{k_u}{2m} \right)^2 \biggr]\\
% 
&= \biggl[ \frac{\Sigma_{c_u} - 2 k_u^{\rightarrow c_u}}{2m}
- \left( \frac{\Sigma_{\hat{c}_u} - k_u}{2m} \right)^2 \biggr]
- \biggl[ \frac{\Sigma_{c_u}}{2m}
- \left( \frac{\Sigma_{\hat{c}_u}}{2m} \right)^2 \biggr]
- \left( \frac{k_u}{2m} \right)^2\\
% 
&= - \frac{k_u^{\rightarrow c_u}}{m}
- \left( \frac{\Sigma_{\hat{c}_u}}{2m} \right)^2
+ 2 \frac{\Sigma_{\hat{c}_u}}{2m} \frac{k_u}{2m}
- \left( \frac{k_u}{2m} \right)^2
+ \left( \frac{\Sigma_{\hat{c}_u}}{2m} \right)^2
- \left( \frac{k_u}{2m} \right)^2\\
% 
&= - \frac{k_u^{\rightarrow c_u}}{m}
+ \frac{2 k_u \Sigma_{\hat{c}_u}}{(2m^2)}
- \frac{2 k_u^2}{(2m)^2}\\
% 
&= \frac{1}{m} \biggl(
    - k_u^{\rightarrow c_u}
    + \frac{k_u (\Sigma_{\hat{c}_u} - k_u)}{2m}
\biggr)
\end{align}
$$

We bring together this equation with the one from the [previous section](./delta-modularity.md) and obtain the following generalized formula for the modularity difference when moving a vertex $u$ from its previous community $c_u$ to any other community $c$. This is done by first moving $u$ from $c_u$ into its own singleton community $\singleton{c}_u$, then moving it from there into the target community $c$:

$$
\begin{align}
\Delta Q(u, c_u, c)
% 
&= \Delta Q(u, c_u, \singleton{c}_u)
+ \Delta Q(u, \singleton{c}_u, c)\\
% 
&= \frac{1}{m} \biggl(
    - k_u^{\rightarrow c_u}
    + \frac{k_u (\Sigma_{\hat{c}_u} - k_u)}{2m}
\biggr)
+
\frac{1}{m} \biggl(
    k_u^{\rightarrow c}
    - \frac{\Sigma_{\hat{c}} k_u}{2m}
\biggr)\\
% 
&= \frac{1}{m} \biggl(
    k_u^{\rightarrow c} - k_u^{\rightarrow c_u}
    + \frac{k_u (\Sigma_{\hat{c}_u} - k_u)
    - \Sigma_{\hat{c}} k_u}{2m}
\biggr)\\
% 
&= \frac{1}{m} \biggl(
    k_u^{\rightarrow c} - k_u^{\rightarrow c_u}
    + \frac{k_u (\Sigma_{\hat{c}_u} - \Sigma_{\hat{c}} - k_u)}{2m}
\biggr)\\
\end{align}
$$

Finally, for the generalized formula, we have

$$
\boxed{
    \Delta Q(u, c_u, c) \propto
    (k_u^{\rightarrow c} - k_u^{\rightarrow c_u})
    + \frac{k_u (\Sigma_{\hat{c}_u} - \Sigma_{\hat{c}} - k_u)}{2m}
}
$$


---


The delta modularity calculation for removing from a singleton community can be derived from this formula by assuming that $c_u$ is a singleton community. As we do not allow self-loops, $k_u^{\rightarrow c_u} = 0$. Moreover, the sum of the weights of edges incident to vertices in $c_u$ is equal to the vertex degree: $\sum_{\hat{c}_u} = k_u$, which gives us:

$$
\Delta Q(u, \singleton{c}_u, c)
\propto (k_u^{\rightarrow c} - 0)
+ \frac{k_u (k_u  - \Sigma_{\hat{c}} - k_u)}{2m}
= k_u^{\rightarrow c} - \frac{\Sigma_{\hat{c}} k_u}{m}
$$

which is indeed the formula derived [here](./delta-modularity.md).
