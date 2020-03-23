---
title: Exercice 3
---

$$
\newcommand{\card}[1]{card(#1)}
\DeclareMathOperator*{\max}{max}
\require{cancel}
$$

## Exercice 3

### Définissez un graphe

* Soit un graphe orienté, \\(G = (S, A)\\),

  \\(S = \\{ s \mid s \text{ est l'action de mettre un vêtement} \\}\\)

  \\(A = \\{ (i, j) \in S^2 \mid i \text{ doit être effectuée avant } j \\}\\)

### Quel algorithme permet de résoudre ce problème ?

* Tri topologique
  * Précondition : Le graphe doit être orienté et acyclique (DAG)
  * Complexité : \\(\mathcal{O}(\left|S\right| + \left|A\right|)\\)

### Résolution

**Étape 0 :**

<img src="/img/courses/td_aaia/exo3/step_0.png" style="height: 400px;" />

|   | C | H | K | M | P | R | T | U | V |
|---|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
| ordre | \\(\infty\\) | \\(\infty\\) | \\(\infty\\) | \\(\infty\\) | \\(\infty\\) | \\(\infty\\) | \\(\infty\\) | \\(\infty\\) | \\(\infty\\) |

\\(succ(C) = \emptyset\\)

**Étape 1 :**

<img src="/img/courses/td_aaia/exo3/step_1.png" style="height: 400px;" />

|   | C | H | K | M | P | R | T | U | V |
|---|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
|   | \\(0\\) | | | | | | | | |
| ordre | \\(\color{red}{\cancel{\infty}}\\) | \\(\infty\\) | \\(\infty\\) | \\(\infty\\) | \\(\infty\\) | \\(\infty\\) | \\(\infty\\) | \\(\infty\\) | \\(\infty\\) |

**Étape 2 :**

<img src="/img/courses/td_aaia/exo3/step_2.png" style="height: 400px;" />

|   | C | H | K | M | P | R | T | U | V |
|---|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
|   | \\(0\\) | | | | | | | | |
| ordre | \\(\cancel{\infty}\\) | \\(\infty\\) | \\(\infty\\) | \\(\infty\\) | \\(\infty\\) | \\(\infty\\) | \\(\infty\\) | \\(\infty\\) | \\(\infty\\) |

\\(succ(H) = \\{R, U\\}\\)

**Étape 3 :**

<img src="/img/courses/td_aaia/exo3/step_3.png" style="height: 400px;" />

|   | C | H | K | M | P | R | T | U | V |
|---|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
|   | \\(0\\) | | | | | | | | |
| ordre | \\(\cancel{\infty}\\) | \\(\infty\\) | \\(\infty\\) | \\(\infty\\) | \\(\infty\\) | \\(\infty\\) | \\(\infty\\) | \\(\infty\\) | \\(\infty\\) |

\\(succ(R) = \\{V\\}\\)

**Étape 4 :**

<img src="/img/courses/td_aaia/exo3/step_4.png" style="height: 400px;" />

|   | C | H | K | M | P | R | T | U | V |
|---|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
|   | \\(0\\) | | | | | | | | |
| ordre | \\(\cancel{\infty}\\) | \\(\infty\\) | \\(\infty\\) | \\(\infty\\) | \\(\infty\\) | \\(\infty\\) | \\(\infty\\) | \\(\infty\\) | \\(\infty\\) |

\\(succ(V) = \emptyset\\)

**Étape 5 :**

<img src="/img/courses/td_aaia/exo3/step_5.png" style="height: 400px;" />

|   | C | H | K | M | P | R | T | U | V |
|---|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
|   | \\(0\\) | | | | | | | | \\(1\\) |
| ordre | \\(\cancel{\infty}\\) | \\(\infty\\) | \\(\infty\\) | \\(\infty\\) | \\(\infty\\) | \\(\infty\\) | \\(\infty\\) | \\(\infty\\) | \\(\color{red}{\cancel{\infty}}\\) |

**Étape 6 :**

<img src="/img/courses/td_aaia/exo3/step_6.png" style="height: 400px;" />

|   | C | H | K | M | P | R | T | U | V |
|---|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
|   | \\(0\\) | | | | |  | | | \\(1\\) |
| ordre | \\(\cancel{\infty}\\) | \\(\infty\\) | \\(\infty\\) | \\(\infty\\) | \\(\infty\\) | \\(\infty\\) | \\(\infty\\) | \\(\infty\\) | \\(\cancel{\infty}\\) |

\\(succ(R) = \\{\cancel{V}\\}\\)

**Étape 7 :**

<img src="/img/courses/td_aaia/exo3/step_7.png" style="height: 400px;" />

|   | C | H | K | M | P | R | T | U | V |
|---|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
|   | \\(0\\) | | | | | \\(2\\) | | | \\(1\\) |
| ordre | \\(\cancel{\infty}\\) | \\(\infty\\) | \\(\infty\\) | \\(\infty\\) | \\(\infty\\) | \\(\color{red}{\cancel{\infty}}\\) | \\(\infty\\) | \\(\infty\\) | \\(\cancel{\infty}\\) |

**Étape 8 :**

<img src="/img/courses/td_aaia/exo3/step_8.png" style="height: 400px;" />

|   | C | H | K | M | P | R | T | U | V |
|---|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
|   | \\(0\\) | | | | | \\(2\\) | | | \\(1\\) |
| ordre | \\(\cancel{\infty}\\) | \\(\infty\\) | \\(\infty\\) | \\(\infty\\) | \\(\infty\\) | \\(\cancel{\infty}\\) | \\(\infty\\) | \\(\infty\\) | \\(\cancel{\infty}\\) |

\\(succ(H) = \\{\cancel{R}, U\\}\\)

**Étape 9 :**

<img src="/img/courses/td_aaia/exo3/step_9.png" style="height: 400px;" />

|   | C | H | K | M | P | R | T | U | V |
|---|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
|   | \\(0\\) | | | | | \\(2\\) | | | \\(1\\) |
| ordre | \\(\cancel{\infty}\\) | \\(\infty\\) | \\(\infty\\) | \\(\infty\\) | \\(\infty\\) | \\(\cancel{\infty}\\) | \\(\infty\\) | \\(\infty\\) | \\(\cancel{\infty}\\) |

\\(succ(U) = \\{\cancel{V}\\}\\)

**Étape 10 :**

<img src="/img/courses/td_aaia/exo3/step_10.png" style="height: 400px;" />

|   | C | H | K | M | P | R | T | U | V |
|---|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
|   | \\(0\\) | | | | | \\(2\\) | | \\(3\\) | \\(1\\) |
| ordre | \\(\cancel{\infty}\\) | \\(\infty\\) | \\(\infty\\) | \\(\infty\\) | \\(\infty\\) | \\(\cancel{\infty}\\) | \\(\infty\\) | \\(\color{red}{\cancel{\infty}}\\) | \\(\cancel{\infty}\\) |

**Étape 11 :**

<img src="/img/courses/td_aaia/exo3/step_11.png" style="height: 400px;" />

|   | C | H | K | M | P | R | T | U | V |
|---|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
|   | \\(0\\) | | | | | \\(2\\) | | \\(3\\) | \\(1\\) |
| ordre | \\(\cancel{\infty}\\) | \\(\infty\\) | \\(\infty\\) | \\(\infty\\) | \\(\infty\\) | \\(\cancel{\infty}\\) | \\(\infty\\) | \\(\cancel{\infty}\\) | \\(\cancel{\infty}\\) |

\\(succ(H) = \\{\cancel{R}, \cancel{U}\\}\\)

**Étape 12 :**

<img src="/img/courses/td_aaia/exo3/step_12.png" style="height: 400px;" />

|   | C | H | K | M | P | R | T | U | V |
|---|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
|   | \\(0\\) | \\(4\\) | | | | \\(2\\) | | \\(3\\) | \\(1\\) |
| ordre | \\(\cancel{\infty}\\) | \\(\color{red}{\cancel{\infty}}\\) | \\(\infty\\) | \\(\infty\\) | \\(\infty\\) | \\(\cancel{\infty}\\) | \\(\infty\\) | \\(\cancel{\infty}\\) | \\(\cancel{\infty}\\) |

**Étape 13 :**

<img src="/img/courses/td_aaia/exo3/step_13.png" style="height: 400px;" />

|   | C | H | K | M | P | R | T | U | V |
|---|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
|   | \\(0\\) | \\(4\\) | | | | \\(2\\) | | \\(3\\) | \\(1\\) |
| ordre | \\(\cancel{\infty}\\) | \\(\infty\\) | \\(\infty\\) | \\(\infty\\) | \\(\infty\\) | \\(\cancel{\infty}\\) | \\(\infty\\) | \\(\cancel{\infty}\\) | \\(\cancel{\infty}\\) |

\\(succ(K) = \\{\cancel{C}, P\\}\\)

**Étape 14 :**

<img src="/img/courses/td_aaia/exo3/step_14.png" style="height: 400px;" />

|   | C | H | K | M | P | R | T | U | V |
|---|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
|   | \\(0\\) | \\(4\\) | | | | \\(2\\) | | \\(3\\) | \\(1\\) |
| ordre | \\(\cancel{\infty}\\) | \\(\infty\\) | \\(\infty\\) | \\(\infty\\) | \\(\infty\\) | \\(\cancel{\infty}\\) | \\(\infty\\) | \\(\cancel{\infty}\\) | \\(\cancel{\infty}\\) |

\\(succ(P) = \\{\cancel{C}, \cancel{U}\\}\\)

**Étape 15 :**

<img src="/img/courses/td_aaia/exo3/step_15.png" style="height: 400px;" />

|   | C | H | K | M | P | R | T | U | V |
|---|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
|   | \\(0\\) | \\(4\\) | | | \\(5\\) | \\(2\\) | | \\(3\\) | \\(1\\) |
| ordre | \\(\cancel{\infty}\\) | \\(\infty\\) | \\(\infty\\) | \\(\infty\\) | \\(\color{red}{\cancel{\infty}}\\) | \\(\cancel{\infty}\\) | \\(\infty\\) | \\(\cancel{\infty}\\) | \\(\cancel{\infty}\\) |

**Étape 16 :**

<img src="/img/courses/td_aaia/exo3/step_16.png" style="height: 400px;" />

|   | C | H | K | M | P | R | T | U | V |
|---|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
|   | \\(0\\) | \\(4\\) | | | \\(5\\) | \\(2\\) | | \\(3\\) | \\(1\\) |
| ordre | \\(\cancel{\infty}\\) | \\(\infty\\) | \\(\infty\\) | \\(\infty\\) | \\(\cancel{\infty}\\) | \\(\cancel{\infty}\\) | \\(\infty\\) | \\(\cancel{\infty}\\) | \\(\cancel{\infty}\\) |

\\(succ(K) = \\{\cancel{C}, \cancel{P}\\}\\)

**Étape 17 :**

<img src="/img/courses/td_aaia/exo3/step_17.png" style="height: 400px;" />

|   | C | H | K | M | P | R | T | U | V |
|---|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
|   | \\(0\\) | \\(4\\) | \\(6\\) | | \\(5\\) | \\(2\\) | | \\(3\\) | \\(1\\) |
| ordre | \\(\cancel{\infty}\\) | \\(\infty\\) | \\(\color{red}{\cancel{\infty}}\\) | \\(\infty\\) | \\(\cancel{\infty}\\) | \\(\cancel{\infty}\\) | \\(\infty\\) | \\(\cancel{\infty}\\) | \\(\cancel{\infty}\\) |

**Étape 18 :**

<img src="/img/courses/td_aaia/exo3/step_18.png" style="height: 400px;" />

|   | C | H | K | M | P | R | T | U | V |
|---|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
|   | \\(0\\) | \\(4\\) | \\(6\\) | | \\(5\\) | \\(2\\) | | \\(3\\) | \\(1\\) |
| ordre | \\(\cancel{\infty}\\) | \\(\infty\\) | \\(\cancel{\infty}\\) | \\(\infty\\) | \\(\cancel{\infty}\\) | \\(\cancel{\infty}\\) | \\(\infty\\) | \\(\cancel{\infty}\\) | \\(\cancel{\infty}\\) |

\\(succ(M) = \emptyset\\)

**Étape 19 :**

<img src="/img/courses/td_aaia/exo3/step_19.png" style="height: 400px;" />

|   | C | H | K | M | P | R | T | U | V |
|---|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
|   | \\(0\\) | \\(4\\) | \\(6\\) | \\(7\\) | \\(5\\) | \\(2\\) | | \\(3\\) | \\(1\\) |
| ordre | \\(\cancel{\infty}\\) | \\(\infty\\) | \\(\cancel{\infty}\\) | \\(\color{red}{\cancel{\infty}}\\) | \\(\cancel{\infty}\\) | \\(\cancel{\infty}\\) | \\(\infty\\) | \\(\cancel{\infty}\\) | \\(\cancel{\infty}\\) |

**Étape 20 :**

<img src="/img/courses/td_aaia/exo3/step_20.png" style="height: 400px;" />

|   | C | H | K | M | P | R | T | U | V |
|---|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
|   | \\(0\\) | \\(4\\) | \\(6\\) | \\(7\\) | \\(5\\) | \\(2\\) | | \\(3\\) | \\(1\\) |
| ordre | \\(\cancel{\infty}\\) | \\(\infty\\) | \\(\cancel{\infty}\\) | \\(\cancel{\infty}\\) | \\(\cancel{\infty}\\) | \\(\cancel{\infty}\\) | \\(\infty\\) | \\(\cancel{\infty}\\) | \\(\cancel{\infty}\\) |

\\(succ(T) = \\{\cancel{C}\\}\\)

**Étape 21 :**

<img src="/img/courses/td_aaia/exo3/step_21.png" style="height: 400px;" />

|   | C | H | K | M | P | R | T | U | V |
|---|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
|   | \\(0\\) | \\(4\\) | \\(6\\) | \\(7\\) | \\(5\\) | \\(2\\) | \\(8\\) | \\(3\\) | \\(1\\) |
| ordre | \\(\cancel{\infty}\\) | \\(\infty\\) | \\(\cancel{\infty}\\) | \\(\cancel{\infty}\\) | \\(\cancel{\infty}\\) | \\(\cancel{\infty}\\) | \\(\color{red}{\cancel{\infty}}\\) | \\(\cancel{\infty}\\) | \\(\cancel{\infty}\\) |

**Résultat :**

|   | C | H | K | M | P | R | T | U | V |
|---|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
| ordre | \\(0\\) | \\(4\\) | \\(6\\) | \\(7\\) | \\(5\\) | \\(2\\) | \\(8\\) | \\(3\\) | \\(1\\) |
| ordre inverse | \\(8\\) | \\(4\\) | \\(2\\) | \\(1\\) | \\(3\\) | \\(6\\) | \\(0\\) | \\(5\\) | \\(7\\) |

L'ordre des tâches à effectuer est : T, M, K, P, H, U, R, V, C.
