---
title: Exercice 1
---

$$
\newcommand{\card}[1]{card(#1)}
\DeclareMathOperator*{\max}{max}
\require{cancel}
$$

## Exercice 1

### Définissez un graphe et reformulez ces deux propriétés par rapport à ce graphe.

On définit le graphe simple non orienté \\(G = (S, A)\\), avec \\(S = \\{ i \mid i \text{ est un membre du reseau} \\}\\) et \\(A = \\{ \\{i, j\\} \subseteq S \mid i \text{ est ami avec } j \\}\\).

**Propriétés 1 :** Il existe au moins deux membres du réseau qui ont le même nombre d'amis :
$$
\exists{\\{i, j\\}} \subseteq S, deg(i) = deg(j)
$$
**Propriété 2 :** Il existe un nombre pair de membres ayant un nombre impair d'amis :
$$
\text{Soit } I = \\{i \in S \mid k \in \mathbb{Z}, deg(i) = 2k + 1 \\}, \exists{k'} \in \mathbb{Z}, \card{I} = 2k'.
$$

### Démontrez les deux propriétés par rapport à ce graphe.

#### Propriété 1

$$
\exists{\{i, j\}} \subseteq S, deg(i) = deg(j)
$$

Nous utilisons le raisonnement par l'absurde :
$$
\forall{\{i, j\}} \subseteq S, deg(i) \ne \deg(j)
$$

Soit un graphe simple non orienté \\(G = (S, A)\\), avec \\(\card{S} = n\\).
Comme la graphe est un graphe simple, chaque nœud peut se connecter au maximum avec \\(n-1\\) autre nœuds.

Nous avons donc un graphe de la forme : \\(deg(s_0) = 0, deg(s_1) = 1, ..., deg(s_{n - 1}) = n - 1\\).

Or \\(deg(s) = \card{adj(s)}\\), d'où \\(adj(s_0)  = \emptyset\\) et \\(adj(s_{n - 1}) = N \setminus s_{n - 1} \implies s_0 \in adj(s_{n - 1}) \wedge s_{n - 1} \notin adj(s_0)\\). Ce qui est en contradiction avec la définition de la symétrie des arêtes dans les graphes non orientés.

#### Propriété 2

$$
\text{Soit: } I = \\{i \in S \mid \exists{k} \in \mathbb{Z}, deg(i) = 2k + 1 \\}
$$

$$
\text{Nous voulons montrer: } \exists{k'} \in \mathbb{Z}, \card{I} = 2k'.
$$

Soit :
$$
P = \\{ p \in S \mid \exists{k} \in \mathbb{Z}, deg(p) = 2k \\} \\
$$
Nous avons :
$$
\sum_{s \in S}{deg(s)}= 2\left| A\right|
$$

$$
\sum_{s \in S}{deg(s)} = deg(P) + deg(I), \text{car } P \cup I = G \wedge P \cap I = \emptyset.
$$

$$
\sum_{s \in S}{deg(s)} = \sum_{p \in P}{deg(p)} + \sum_{i \in I}{deg(i)}
$$
$$
\sum_{s \in S}{deg(s)} = \sum_{p \in P}{2k_p} + \sum_{i \in I}{\left(2k_i + 1\right)}
$$
$$
\sum_{s \in S}{deg(s)} = 2\sum_{p \in P}{k_p} + 2\sum_{i \in I}{k_i} + \sum_{i \in I}{1} = 2(\sum_{p \in P}{k_p} + \sum_{i \in I}{k_i}) + \card{I}
$$

Ce qui implique de \\(\card{I}\\) est pair.