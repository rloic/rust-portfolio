---
title: Exercice 6
---

## Exercice 6

1. Définissez un graphe, et reformulez le problème en le ramenant à un problème vu en cours.
  
2. Quel algorithme vu en cours permet de résoudre ce problème ? Comment l’adapter pourminimiser le nombre de calculs à faire ?

3. Résolvez le problème en détaillant le déroulement de l’algorithme.

### Définissez un graphe, et reformulez le problème en le ramenant à un problème vu en cours.

Nous définissons le graphe simple non orienté \\(G = (S, A)\\),

$$
S = \\{ i \\mid i \text{ est une intersection } \\}
$$
$$
A = \\{ \\{i, j \\} \subseteq S \mid \text{ il existe un tronçon de route allant de } i \text { à } j \\}
$$

### Quel algorithme vu en cours permet de résoudre ce problème ? Comment l’adapter pour minimiser le nombre de calculs à faire ?

Disjktra
- Complexité : \\(\mathcal{O}(~\left| A \right| + \left| S \right| log \left| S \right|~)\\)
- Précondition : Le poids des arêtes doit être positif

### Résolvez le problème en détaillant le déroulement de l’algorithme.

![bg right:40% 80%](/img/courses/td_aaia/exo6/etape1/step_0.png)

$$F = \left[{(A, 0)}\right]$$

| A | B | C | D | E | F | G | H | I | T |
|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
| A | \\(\bot\\) | \\(\bot\\) | \\(\bot\\) | \\(\bot\\) | \\(\bot\\) | \\(\bot\\) | \\(\bot\\) | \\(\bot\\) | \\(\bot\\) |

![bg right:40% 80%](/img/courses/td_aaia/exo6/etape1/step_1.png)

$$F = \left[\cancel{{(A, 0)}}, (B, 1)\right]$$
$$C = (A, 0)$$

| A | B | C | D | E | F | G | H | I | T |
|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
|   | A |   |   |   |   |   |   |   |   |
| A | \\(\cancel{\bot}\\) | \\(\bot\\) | \\(\bot\\) | \\(\bot\\) | \\(\bot\\) | \\(\bot\\) | \\(\bot\\) | \\(\bot\\) | \\(\bot\\) |

![bg right:40% 80%](/img/courses/td_aaia/exo6/etape1/step_2.png)

$$F = \left[\cancel{{(A, 0)}}, (B, 1), (G, 2)\right]$$
$$C = (A, 0)$$

| A | B | C | D | E | F | G | H | I | T |
|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
|   | A |   |   |   |   | A |   |   |   |
| A | \\(\cancel{\bot}\\) | \\(\bot\\) | \\(\bot\\) | \\(\bot\\) | \\(\bot\\) | \\(\cancel{\bot}\\) | \\(\bot\\) | \\(\bot\\) | \\(\bot\\) |

![bg right:40% 80%](/img/courses/td_aaia/exo6/etape1/step_3.png)

$$F = \left[\cancel{{(A, 0)}}, (B, 1), (G, 2), (H, 6)\right]$$
$$C = (A, 0)$$

| A | B | C | D | E | F | G | H | I | T |
|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
|   | A |   |   |   |   | A | A |   |   |
| A | \\(\cancel{\bot}\\) | \\(\bot\\) | \\(\bot\\) | \\(\bot\\) | \\(\bot\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\bot\\) | \\(\bot\\) |

![bg right:40% 80%](/img/courses/td_aaia/exo6/etape1/step_4.png)

$$F = \left[\cancel{{(A, 0)}}, \cancel{(B, 1)}, (G, 2), (C, 4), (H, 6)\right]$$
$$C = (B, 1)$$

| A | B | C | D | E | F | G | H | I | T |
|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
|   | A | B |   |   |   | A | A |   |   |
| A | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\bot\\) | \\(\bot\\) | \\(\bot\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\bot\\) | \\(\bot\\) |

![bg right:40% 80%](/img/courses/td_aaia/exo6/etape1/step_5.png)

$$F = \left[\cancel{{(A, 0)}}, \cancel{(B, 1)}, \cancel{(G, 2)}, (D, 3), (C, 4), (H, 6)\right]$$
$$C = (G, 2)$$

| A | B | C | D | E | F | G | H | I | T |
|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
|   | A | B | G |   |   | A | A |   |   |
| A | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\bot\\) | \\(\bot\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\bot\\) | \\(\bot\\) |

![bg right:40% 80%](/img/courses/td_aaia/exo6/etape1/step_6.png)

$$F = \left[\cancel{{(A, 0)}}, \cancel{(B, 1)}, \cancel{(G, 2)}, \cancel{(D, 3)}, (C, 4), (H, 5)\right]$$
$$C = (D, 3)$$

| A | B | C | D | E | F | G | H | I | T |
|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
|   |   |   |   |   |   |   | D |   |   |
|   | A | B | G |   |   | A | \\(\cancel{A}\\) |   |   |
| A | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\bot\\) | \\(\bot\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\bot\\) | \\(\bot\\) 

![bg right:40% 80%](/img/courses/td_aaia/exo6/etape1/step_7.png)

$$F = \left[\cancel{{(A, 0)}}, \cancel{(B, 1)}, \cancel{(G, 2)}, \cancel{(D, 3)}, (C, 4), (T, 4), (H, 5)\right]$$
$$C = (D, 3)$$

| A | B | C | D | E | F | G | H | I | T |
|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
|   |   |   |   |   |   |   | D |   |   |
|   | A | B | G |   |   | A | \\(\cancel{A}\\) |   | D |
| A | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\bot\\) | \\(\bot\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\bot\\) | \\(\cancel{\bot}\\) |

![bg right:40% 80%](/img/courses/td_aaia/exo6/etape1/step_8.png)

$$F = \left[\cancel{{(A, 0)}}, \cancel{(B, 1)}, \cancel{(G, 2)}, \cancel{(D, 3)}, \cancel{(C, 4)}, (T, 4), (H, 5), (I, 8)\right]$$
$$C = (C, 4)$$

| A | B | C | D | E | F | G | H | I | T |
|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
|   |   |   |   |   |   |   | D |   |   |
|   | A | B | G |   |   | A | \\(\cancel{A}\\) | C | D |
| A | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\bot\\) | \\(\bot\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) |

![bg right:40% 80%](/img/courses/td_aaia/exo6/etape1/step_9.png)

$$F = \left[\cancel{{(A, 0)}}, \cancel{(B, 1)}, \cancel{(G, 2)}, \cancel{(D, 3)}, \cancel{(C, 4)}, \cancel{(T, 4)}, (H, 5), (E, 6), (I, 8)\right]$$
$$C = (T, 4)$$

| A | B | C | D | E | F | G | H | I | T |
|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
|   |   |   |   |   |   |   | D |   |   |
|   | A | B | G | T |   | A | \\(\cancel{A}\\) | C | D |
| A | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\bot\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) |

![bg right:40% 80%](/img/courses/td_aaia/exo6/etape1/step_10.png)

$$F = \left[\cancel{{(A, 0)}}, \cancel{(B, 1)}, \cancel{(G, 2)}, \cancel{(D, 3)}, \cancel{(C, 4)}, \cancel{(T, 4)}, (H, 5), (E, 6), (I, 8), (F, 10)\right]$$
$$C = (T, 4)$$

| A | B | C | D | E | F | G | H | I | T |
|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
|   |   |   |   |   |   |   | D |   |   |
|   | A | B | G | T | T | A | \\(\cancel{A}\\) | C | D |
| A | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) |

![bg right:40% 80%](/img/courses/td_aaia/exo6/etape1/step_11.png)

$$F = \left[\cancel{{(A, 0)}}, \cancel{(B, 1)}, \cancel{(G, 2)}, \cancel{(D, 3)}, \cancel{(C, 4)}, \cancel{(T, 4)}, \cancel{(H, 5)}, (E, 6), (F, 6), (I, 8)\right]$$
$$C = (H, 5)$$

| A | B | C | D | E | F | G | H | I | T |
|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
|   |   |   |   |   | H |   | D |   |   |
|   | A | B | G | T | \\(\cancel{T}\\) | A | \\(\cancel{A}\\) | C | D |
| A | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) |

![bg right:40% 80%](/img/courses/td_aaia/exo6/etape1/step_12.png)

$$F = \left[\cancel{{(A, 0)}}, \cancel{(B, 1)}, \cancel{(G, 2)}, \cancel{(D, 3)}, \cancel{(C, 4)}, \cancel{(T, 4)}, \cancel{(H, 5)}, \cancel{(E, 6)}, (F, 6), (I, 7)\right]$$
$$C = (E, 6)$$	

| A | B | C | D | E | F | G | H | I | T |
|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
|   |   |   |   |   | H |   | D | E |   |
|   | A | B | G | T | \\(\cancel{T}\\) | A | \\(\cancel{A}\\) | \\(\cancel{C}\\) | D |
| A | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) |

![bg right:40% 80%](/img/courses/td_aaia/exo6/etape1/step_13.png)

$$F = \left[\cancel{{(A, 0)}}, \cancel{(B, 1)}, \cancel{(G, 2)}, \cancel{(D, 3)}, \cancel{(C, 4)}, \cancel{(T, 4)}, \cancel{(H, 5)}, \cancel{(E, 6)}, \cancel{(F, 6)}, (I, 7)\right]$$
$$C = (F, 6)$$	

| A | B | C | D | E | F | G | H | I | T |
|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
|   |   |   |   |   | H |   | D | E |   |
|   | A | B | G | T | \\(\cancel{T}\\) | A | \\(\cancel{A}\\) | \\(\cancel{C}\\) | D |
| A | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) |

![bg right:40% 80%](/img/courses/td_aaia/exo6/etape1/step_14.png)

$$F = \left[\cancel{{(A, 0)}}, \cancel{(B, 1)}, \cancel{(G, 2)}, \cancel{(D, 3)}, \cancel{(C, 4)}, \cancel{(T, 4)}, \cancel{(H, 5)}, \cancel{(E, 6)}, \cancel{(F, 6)}, \cancel{(I, 7)}\right]$$
$$C = (I,7)$$	

| A | B | C | D | E | F | G | H | I | T |
|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
|   |   |   |   |   | H |   | D | E |   |
|   | A | B | G | T | \\(\cancel{T}\\) | A | \\(\cancel{A}\\) | \\(\cancel{C}\\) | D |
| A | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) |

![bg right:40% 80%](/img/courses/td_aaia/exo6/etape2/step_15.png)

$$F = \left[\cancel{(T, 0)}\right]$$
$$C = (T, 0)$$	

| A | B | C | D | E | F | G | H | I | T |
|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
| \\(\bot\\) | \\(\bot\\) | \\(\bot\\) | \\(\bot\\) | \\(\bot\\) | \\(\bot\\) | \\(\bot\\) | \\(\bot\\) | \\(\bot\\) | T |

![bg right:40% 80%](/img/courses/td_aaia/exo6/etape2/step_16.png)

$$F = \left[\cancel{(T, 0)}, (D, 1)\right]$$
$$C = (T, 0)$$	

| A | B | C | D | E | F | G | H | I | T |
|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
|  |  |  | T |  |  |  |  |  |  |
| \\(\bot\\) | \\(\bot\\) | \\(\bot\\) | \\(\cancel{\bot}\\) | \\(\bot\\) | \\(\bot\\) | \\(\bot\\) | \\(\bot\\) | \\(\bot\\) | T |

![bg right:40% 80%](/img/courses/td_aaia/exo6/etape2/step_17.png)

$$F = \left[\cancel{(T, 0)}, (D, 1), (E, 2)\right]$$
$$C = (T, 0)$$	

| A | B | C | D | E | F | G | H | I | T |
|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
|   |   |   | T | T |   |   |   |   |   |
| \\(\bot\\) | \\(\bot\\) | \\(\bot\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\bot\\) | \\(\bot\\) | \\(\bot\\) | \\(\bot\\) | T |

![bg right:40% 80%](/img/courses/td_aaia/exo6/etape2/step_18.png)

$$F = \left[\cancel{(T, 0)}, (D, 1), (E, 2), (F, 6)\right]$$
$$C = (T, 0)$$	

| A | B | C | D | E | F | G | H | I | T |
|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
|   |   |   | T | T | T |   |   |   |   |
| \\(\bot\\) | \\(\bot\\) | \\(\bot\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\bot\\) | \\(\bot\\) | \\(\bot\\) | T |

![bg right:40% 80%](/img/courses/td_aaia/exo6/etape2/step_19.png)

$$F = \left[\cancel{(T, 0)}, \cancel{(D, 1)}, (E, 2), (G, 2), (F, 6)\right]$$
$$C = (D, 1)$$	

| A | B | C | D | E | F | G | H | I | T |
|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
|   |   |   | T | T | T | D |   |   |   |
| \\(\bot\\) | \\(\bot\\) | \\(\bot\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\bot\\) | \\(\bot\\) | T |

![bg right:40% 80%](/img/courses/td_aaia/exo6/etape2/step_20.png)

$$F = \left[\cancel{(T, 0)}, \cancel{(D, 1)}, (E, 2), (G, 2), (H, 3), (F, 6)\right]$$
$$C = (D, 1)$$	

| A | B | C | D | E | F | G | H | I | T |
|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
|   |   |   | T | T | T | D | D |   |   |
| \\(\bot\\) | \\(\bot\\) | \\(\bot\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\bot\\) | T |

![bg right:40% 80%](/img/courses/td_aaia/exo6/etape2/step_21.png)

$$F = \left[\cancel{(T, 0)}, \cancel{(D, 1)}, \cancel{(E, 2)}, (G, 2), (H, 3), (I, 3), (F, 6)\right]$$
$$C = (E, 2)$$	

| A | B | C | D | E | F | G | H | I | T |
|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
|   |   |   | T | T | T | D | D | E |   |
| \\(\bot\\) | \\(\bot\\) | \\(\bot\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | T |

![bg right:40% 80%](/img/courses/td_aaia/exo6/etape2/step_22.png)

$$F = \left[\cancel{(T, 0)}, \cancel{(D, 1)}, \cancel{(E, 2)}, \cancel{(G, 2)}, (H, 3), (I, 3), (A, 4), (F, 6)\right]$$
$$C = (G, 2)$$	

| A | B | C | D | E | F | G | H | I | T |
|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
| G |   |   | T | T | T | D | D | E |   |
| \\(\cancel{\bot}\\) | \\(\bot\\) | \\(\bot\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | T |

![bg right:40% 80%](/img/courses/td_aaia/exo6/etape2/step_23.png)

$$F = \left[\cancel{(T, 0)}, \cancel{(D, 1)}, \cancel{(E, 2)}, \cancel{(G, 2)}, \cancel{(H, 3)}, (I, 3), (A, 4), (F, 6)\right]$$
$$C = (H, 3)$$	

| A | B | C | D | E | F | G | H | I | T |
|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
| G |   |   | T | T | T | D | D | E |   |
| \\(\cancel{\bot}\\) | \\(\bot\\) | \\(\bot\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | T |

![bg right:40% 80%](/img/courses/td_aaia/exo6/etape2/step_24.png)

$$F = \left[\cancel{(T, 0)}, \cancel{(D, 1)}, \cancel{(E, 2)}, \cancel{(G, 2)}, \cancel{(H, 3)}, (I, 3), (A, 4), (F, 4)\right]$$
$$C = (H, 3)$$	

| A | B | C | D | E | F | G | H | I | T |
|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
|   |   |   |   |   | H |   |   |   |   |
| G |   |   | T | T | \\(\cancel{T}\\) | D | D | E |   |
| \\(\cancel{\bot}\\) | \\(\bot\\) | \\(\bot\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | T |

![bg right:40% 80%](/img/courses/td_aaia/exo6/etape2/step_25.png)

$$F = \left[\cancel{(T, 0)}, \cancel{(D, 1)}, \cancel{(E, 2)}, \cancel{(G, 2)}, \cancel{(H, 3)}, \cancel{(I, 3)}, (A, 4), (F, 4), (C, 7)\right]$$
$$C = (I, 3)$$	

| A | B | C | D | E | F | G | H | I | T |
|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
|   |   |   |   |   | H |   |   |   |   |
| G |   | I | T | T | \\(\cancel{T}\\) | D | D | E |   |
| \\(\cancel{\bot}\\) | \\(\bot\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | T |

![bg right:40% 80%](/img/courses/td_aaia/exo6/etape2/step_26.png)

$$F = \left[\cancel{(T, 0)}, \cancel{(D, 1)}, \cancel{(E, 2)}, \cancel{(G, 2)}, \cancel{(H, 3)}, \cancel{(I, 3)}, (A, 4), (F, 4), (C, 7)\right]$$
$$C = (I, 3)$$	

| A | B | C | D | E | F | G | H | I | T |
|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
|   |   |   |   |   | H |   |   |   |   |
| G |   | I | T | T | \\(\cancel{T}\\) | D | D | E |   |
| \\(\cancel{\bot}\\) | \\(\bot\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | T |

![bg right:40% 80%](/img/courses/td_aaia/exo6/etape2/step_27.png)

$$F = \left[\cancel{(T, 0)}, \cancel{(D, 1)}, \cancel{(E, 2)}, \cancel{(G, 2)}, \cancel{(H, 3)}, \cancel{(I, 3)}, \cancel{(A, 4)}, (F, 4), (B, 5), (C, 7)\right]$$
$$C = (A, 4)$$	

| A | B | C | D | E | F | G | H | I | T |
|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
|   |   |   |   |   | H |   |   |   |   |
| G | A | I | T | T | \\(\cancel{T}\\) | D | D | E |   |
| \\(\cancel{\bot}\\) | \\(\bot\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | T |

![bg right:40% 80%](/img/courses/td_aaia/exo6/etape2/step_28.png)

$$F = \left[\cancel{(T, 0)}, \cancel{(D, 1)}, \cancel{(E, 2)}, \cancel{(G, 2)}, \cancel{(H, 3)}, \cancel{(I, 3)}, \cancel{(A, 4)}, \cancel{(F, 4)}, \cancel{(B, 5)}, (C, 7)\right]$$
$$C = (B, 5)$$	

| A | B | C | D | E | F | G | H | I | T |
|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
|   |   |   |   |   | H |   |   |   |   |
| G | A | I | T | T | \\(\cancel{T}\\) | D | D | E |   |
| \\(\cancel{\bot}\\) | \\(\bot\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | T |

![bg right:40% 80%](/img/courses/td_aaia/exo6/etape2/step_29.png)

$$F = \left[\cancel{(T, 0)}, \cancel{(D, 1)}, \cancel{(E, 2)}, \cancel{(G, 2)}, \cancel{(H, 3)}, \cancel{(I, 3)}, \cancel{(A, 4)}, \cancel{(F, 4)}, \cancel{(B, 5)}, \cancel{(C, 7)}\right]$$
$$C = (C, 7)$$	

| A | B | C | D | E | F | G | H | I | T |
|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
|   |   |   |   |   | H |   |   |   |   |
| G | A | I | T | T | \\(\cancel{T}\\) | D | D | E |   |
| \\(\cancel{\bot}\\) | \\(\bot\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | \\(\cancel{\bot}\\) | T |

![bg right:40% 80%](/img/courses/td_aaia/exo6/solution.png)

|                                     | A | B | C | D | E | F | G | H | I | T |
|:------------------------------------|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
| \\(\delta(A, s_k)\\)                    | 0 | 1 | 4 | 3 | 6 | 6 | 2 | 5 | 7 | 4 |
| \\(pred(s_k, A \rightsquigarrow s_k)\\) | A | A | B | G | T | H | A | D | E | D |

<br />

|                                     | A | B | C | D | E | F | G | H | I | T |
|:------------------------------------|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
| \\(\delta(s_k, T)\\)                    | 4 | 5 | 7 | 1 | 2 | 4 | 2 | 3 | 3 | 0 |
| \\(succ(s_k, s_k \rightsquigarrow T)\\) | G | A | I | T | T | H | D | D | E | T |
