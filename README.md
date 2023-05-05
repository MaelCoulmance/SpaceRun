# Projet De Programmation Fonctionnelle Avancée

Le projet que j'ai réalisé est un space-shooter en deux dimmensions, écrit en Rust avec la biblotheque 
[`bevy`](https://bevyengine.org/). Le principe est le même que dans tout les spaces shooters: on doit tuer
le plus d'ennemies possible.

J'ai choisis d'utiliser la bibliothèque `bevy` car elle permet de créer des interfaces graphiques en utilisant
le modèle `ECS` de manière portable, tout en bénéficiant des avantages du language Rust.

## I. Organisation Haut Niveau du Projet

Le projet est divisé en deux modules: `components`, qui contient les 
composants et les entitées du jeu, et `plugins` qui contient les systèmes et les plugins.

Chaque module est en interne composé de plusieurs sous-modules, chacun contenant le code pour une fonctionalité du programme.

Dans l'API bevy, un plugin représente un sous-ensemble de l'application. L'idée est assez simple: au lieu d'avoir
une enorme fonction `main` qui initialise tout les éléments du jeu, on va répartir ces taches dans des genre
de modules, qui seront ensuite executés par l'application.

Le programme comporte au total 6 plugins, chacun responsable d'un élément du jeu.

-  `BackgroundPlugin`: gère l'affichage du fond d'écran et la prise en charge de la paralaxe.
- `ColisionsPlugin`: définie un événement déclenché lorsque deux entités du jeu entre en colision
- `EnemiesPlugin`: gère les ennemies, du spawn au déplacements en passant par le refresh lorsqu'une nouvelle frame est évaluée.
- `PewPewPlugin`: gère les projectiles tirés par le joueur, en principe il aurait aussi du gérer les projectiles tirés par les ennemies.
- `PlayerMovePlugin`: gère les entrées de l'utilisateur. Définie un événement qui est déclenché lorsque le joueur fait bouger son vaisseau. Il existe deux moyens de faire bouger son vaisseau et de tirer un projectile: les flèches du clavier et le bouton espace, ou la souris. L'avantage de cette solution est que les entrée effectuées par le joueur n'ont plus à être prise en compte une fois que l'événement correspondant est déclenché.
- `SpaceshipPlugin`: gère le vaisseau de l'utilisateur

## II. Difficultés Lié au Code

La principale difficulté que j'ai rencontré lors de la conception de mon projet à été de gérer les ennemis. J'ai du
m'y reprendre à plusieurs fois pour trouver le bon système. Le problème venait du fait que je voulais faire bouger
mes enemis selon une certaine corregraphie (par exemple lorsqu'ils arrivent à l'écran, au lieu de simplement les faire
spawn, je voulais les faire se déplacer un peu).

La solution que j'ai trouvé n'est pas optimale, mais elle marche. J'ai ajouter une structure `EnemyLocation` (dans le fichier
`sprun/components/enemies.rs`), qui contient la position initiale de l'enemie (lors du spawn) et sa position finale (lorsque
la "corrégraphie" est terminée). De cette manière, lorqu'on met à jour l'affichage, on peut faire se déplacer l'ennemi
vers son point d'arrivé. Cette solution empêche néanmoins de faire des corégraphies trop complexes; on doit se contenter 
d'un simple déplacement linéaire en abscisses et en ordonnées.

L'autre difficulté que j'ai rencontrée était que je voulais que les ennemies tirent eux aussi des projectiles en
direction du joueur. Le problème était ici de récupérer des coordonnées aléatoires qui correspondent à la position
d'un ennemi et d'y faire spawn un projectile dirigé vers le joueur.

## III. Compilation du programme

Pour compiler le programme, il faut avoir installer `cargo` et `rustc` (si possible dans leur dernières version).

### Pour compiler le programme:

```bash
cargo build --release
```

### Pour executer le programme depuis la console:

```bash
cargo run --release
```

### Pour faire un cleanup des fichiers de build:

```bash
cargo clean
```

### Note:
Il est bien sur possible d'executer le programme autrement qu'en passant par la commande cargo. 
Néanmoins dans ce cas, il faudras s'assurer que le dossier "assets" se trouve bien dans le même dossier 
que l'executable.