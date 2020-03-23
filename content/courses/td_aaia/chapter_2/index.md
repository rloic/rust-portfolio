---
title: Exercice 2
---

$$
\newcommand{\card}[1]{card(#1)}
\DeclareMathOperator*{\max}{max}
\require{cancel}
$$

## Exercice 2

### Définissez un graphe, et reformulez le problème en le ramenant à un problème vu en cours

Soit le graphe non orienté : \\(G = (S, A)\\), \\(S = \\{p \mid p \text{ est un scoot}\\}\\), \\(A = \\{\\{i, j\\} \in S^2 | i \text{ peut communiquer directement avec } j \\}\\).

![Graphe](/img/courses/td_aaia/exo2/graph.png)

Arthur souhaite savoir combien de temps il faudra pour que tout le monde ait reçu son message, et qui seront les derniers informés ?

Dans un graphe avec des poids uniformes : \\(\delta(s_i, s_j) = k \times \\) nombre d'arêtes entre \\(s_i\\) et \\(s_j\\).

On recherche :
$$
T = \max_{s_k \in S}{\delta(A, s_k)}
$$
Les derniers informés sont :
$$
D = \\{s \in S \mid \delta(A, s) = T \\}
$$

### Quel algorithme permet de résoudre ce problème ?

* Breadth First Search
  * Précondition : \\(\forall{\\{i, j\\}} \subseteq S, poids_i = poids_j\\).
  * Complexité \\(\mathcal{O}(\left|S \right| + \left|A\right|)\\)

### Résolution

**Étape 0:**

<img src="/img/courses/td_aaia/exo2/step_0.png" style="height: 400px;" />

\\(F = [A]\\)

|   | A | B | C | D | E | F | G | H | I |
|---|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
| \\(\delta(A, s_k)\\) | \\(0\\) | \\(\infty\\) | \\(\infty\\) | \\(\infty\\) | \\(\infty\\) | \\(\infty\\) | \\(\infty\\) | \\(\infty\\) | \\(\infty\\) |

**Étapes 1:**

<img src="/img/courses/td_aaia/exo2/step_1.png" style="height: 400px;" />

\\(F = [\cancel{A}, B, D]\\)

|   | A | B | C | D | E | F | G | H | I |
|---|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
|   |   | \\(1\\) |   | \\(1\\) |   |   |   |   |   |
| \\(\delta(A, s_k)\\) | \\(0\\) | \\(\color{red}{\cancel{\infty}}\\) | \\(\infty\\) | \\(\color{red}{\cancel{\infty}}\\) | \\(\infty\\) | \\(\infty\\) | \\(\infty\\) | \\(\infty\\) |\\(\infty\\)|

**Étape 2:**

<img src="/img/courses/td_aaia/exo2/step_2.png" style="height: 400px;" />

\\(F = [\cancel{A}, \cancel{B}, D, F]\\)

|   | A | B | C | D | E | F | G | H | I |
|---|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
|   |  | \\(1\\) |  | \\(1\\) |  | \\(2\\) |  |  |  |
| \\(\delta(A, s_k)\\) | \\(0\\) | \\(\cancel{\infty}\\) | \\(\infty\\) | \\(\cancel{\infty}\\) | \\(\infty\\) | \\(\color{red}{\cancel{\infty}}\\) | \\(\infty\\) | \\(\infty\\) |\\(\infty\\)|

**Étape 3:**

<img src="/img/courses/td_aaia/exo2/step_3.png" style="height: 400px;" />

\\(F = [\cancel{A}, \cancel{B}, \cancel{D}, F, G]\\)

|   | A | B | C | D | E | F | G | H | I |
|---|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
|   |  | \\(1\\) |  | \\(1\\) |  | \\(2\\) | \\(2\\) |  |  |
| \\(\delta(A, s_k)\\) | \\(0\\) | \\(\cancel{\infty}\\) | \\(\infty\\) | \\(\cancel{\infty}\\) | \\(\infty\\) | \\(\cancel{\infty}\\) | \\(\color{red}{\cancel{\infty}}\\) | \\(\infty\\) |\\(\infty\\)|

**Étape 4:**

<img src="/img/courses/td_aaia/exo2/step_4.png" style="height: 400px;" />

\\(F = [\cancel{A}, \cancel{B}, \cancel{D}, \cancel{F}, G, C, E, I]\\)

|   | A | B | C | D | E | F | G | H | I |
|---|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
|   |  | \\(1\\) | \\(3\\) | \\(1\\) | \\(3\\) | \\(2\\) | \\(2\\) |  | \\(3\\) |
| \\(\delta(A, s_k)\\) | \\(0\\) | \\(\cancel{\infty}\\) | \\(\color{red}{\cancel{\infty}}\\) | \\(\cancel{\infty}\\) | \\(\color{red}{\cancel{\infty}}\\) | \\(\cancel{\infty}\\) | \\(\cancel{\infty}\\) | \\(\infty\\) |\\(\color{red}{\cancel{\infty}}\\)|

**Étape 5:**

<img src="/img/courses/td_aaia/exo2/step_5.png" style="height: 400px;" />

\\(F = [\cancel{A}, \cancel{B}, \cancel{D}, \cancel{F}, \cancel{G}, C, E, I, H]\\)

|   | A | B | C | D | E | F | G | H | I |
|---|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
|   |  | \\(1\\) | \\(3\\) | \\(1\\) | \\(3\\) | \\(2\\) | \\(2\\) | \\(3\\) |\\(3\\)|
| \\(\delta(A, s_k)\\) | \\(0\\) | \\(\cancel{\infty}\\) | \\(\cancel{\infty}\\) | \\(\cancel{\infty}\\) | \\(\cancel{\infty}\\) | \\(\cancel{\infty}\\) | \\(\cancel{\infty}\\) | \\(\color{red}{\cancel{\infty}}\\) | \\(\cancel{\infty}\\) |

**Étape 6:**

<img src="/img/courses/td_aaia/exo2/step_6.png" style="height: 400px;" />

\\(F = [\cancel{A}, \cancel{B}, \cancel{D}, \cancel{F}, \cancel{G}, \cancel{C}, E, I, H]\\)

|   | A | B | C | D | E | F | G | H | I |
|---|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
|   |  | \\(1\\) | \\(3\\) | \\(1\\) | \\(3\\) | \\(2\\) | \\(2\\) | \\(3\\) |\\(3\\)|
| \\(\delta(A, s_k)\\) | \\(0\\) | \\(\cancel{\infty}\\) | \\(\cancel{\infty}\\) | \\(\cancel{\infty}\\) | \\(\cancel{\infty}\\) | \\(\cancel{\infty}\\) | \\(\cancel{\infty}\\) | \\(\cancel{\infty}\\) | \\(\cancel{\infty}\\) |

**Étape 7:**

<img src="/img/courses/td_aaia/exo2/step_7.png" style="height: 400px;" />

\\(F = [\cancel{A}, \cancel{B}, \cancel{D}, \cancel{F}, \cancel{G}, \cancel{C}, \cancel{E}, I, H]\\)

|   | A | B | C | D | E | F | G | H | I |
|---|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
|   |  | \\(1\\) | \\(3\\) | \\(1\\) | \\(3\\) | \\(2\\) | \\(2\\) | \\(3\\) |\\(3\\)|
| \\(\delta(A, s_k)\\) | \\(0\\) | \\(\cancel{\infty}\\) | \\(\cancel{\infty}\\) | \\(\cancel{\infty}\\) | \\(\cancel{\infty}\\) | \\(\cancel{\infty}\\) | \\(\cancel{\infty}\\) | \\(\cancel{\infty}\\) | \\(\cancel{\infty}\\) |

**Étape 8:**

<img src="/img/courses/td_aaia/exo2/step_8.png" style="height: 400px;" />

\\(F = [\cancel{A}, \cancel{B}, \cancel{D}, \cancel{F}, \cancel{G}, \cancel{C}, \cancel{E}, \cancel{I}, H]\\)

|   | A | B | C | D | E | F | G | H | I |
|---|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
|   |  | \\(1\\) | \\(3\\) | \\(1\\) | \\(3\\) | \\(2\\) | \\(2\\) | \\(3\\) |\\(3\\)|
| \\(\delta(A, s_k)\\) | 0 | \\(\cancel{\infty}\\) | \\(\cancel{\infty}\\) | \\(\cancel{\infty}\\) | \\(\cancel{\infty}\\) | \\(\cancel{\infty}\\) | \\(\cancel{\infty}\\) | \\(\cancel{\infty}\\) | \\(\cancel{\infty}\\) |

**Étape 9:**

<img src="/img/courses/td_aaia/exo2/step_9.png" style="height: 400px;" />

\\(F = [\cancel{A}, \cancel{B}, \cancel{D}, \cancel{F}, \cancel{G}, \cancel{C}, \cancel{E}, \cancel{I}, \cancel{H}]\\)

|   | A | B | C | D | E | F | G | H | I |
|---|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
|   |  | \\(1\\) | \\(3\\) | \\(1\\) | \\(3\\) | \\(2\\) | \\(2\\) | \\(3\\) |\\(3\\)|
| \\(\delta(A, s_k)\\) | \\(0\\) | \\(\cancel{\infty}\\) | \\(\cancel{\infty}\\) | \\(\cancel{\infty}\\) | \\(\cancel{\infty}\\) | \\(\cancel{\infty}\\) | \\(\cancel{\infty}\\) | \\(\cancel{\infty}\\) | \\(\cancel{\infty}\\) |

**Réponse**

|   | A | B | C | D | E | F | G | H | I |
|---|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
| \\(\delta(A, s_k)\\) | \\(0\\) | \\(1\\) | \\(3\\) | \\(1\\) | \\(3\\) | \\(2\\) | \\(2\\) | \\(3\\) |\\(3\\)|

Le temps de transmission du message est de 3 minutes.
Les derniers qui recevront le message seront : Céleste, Ernestine, Hugo et Isidore.
