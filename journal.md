# Rust
## Pourquoi Rust ?

### Performance
- Comment Rust parvient-il à être rapide sans environnement d'exécution ?
- Dans quels types de systèmes embarqués Rust peut-il être utilisé ?

### Fiabilité
- Quels types d'erreur (Semantique ?) peuvent être éliminés dès la compilation grâce à Rust ?
- Comment Rust assure-t-il la sécurité des threads ?

### Productivité
- Comment les messages d'erreur du compilateur Rust sont-ils utiles pour les développeurs ?
- Quels sont les types d'outils fournis avec Rust pour améliorer la productivité ?

## Faites le en Rust
### Ligne de Commande
- Quels sont les avantages de créer des outils en ligne de commande avec Rust ?
- Comment Rust facilite-t-il la distribution des applications en ligne de commande ?
### Web Assmbly
- Quels sont les avantages de publier des modules Rust sur npm ?

### Réseau
- Quels sont les avantages de Rust pour les services réseau ?

### Embarqué
- Quels sont les avantages de Rust pour le développement embarqué ?


## Le langage de programmation Rust
- Pourquoi la communautee rust est elle investie a ce point ?

## Avant-propos
### Paragraphe 1
- Rust est un langage de programmation puissant et permet de coder dans la diversite.
**Question :** Comment Rust permet-il de programmer en toute confiance dans divers domaines ?
### Paragraphe 2
- Rust simplifie la gestion des éléments au niveau système, traditionnellement jugée mysterieux et complexe.
**Question :** Comment Rust simplifie-t-il la gestion des éléments au niveau système ?

### Paragraphe 3
- Rust élimine les pièges traditionnels de la programmation bas niveau en offrant des outils conviviaux et fiables.
**Question :** Quels outils Rust offre-t-il pour éviter les pièges de la programmation bas niveau ?

### Paragraphe 4
- Rust permet d'introduire du parallélisme à faible risque grâce à son compilateur qui détecte les erreurs classiques.
**Question :** Comment Rust facilite-t-il l'introduction du parallélisme dans le code ?

### Paragraphe 5
- Rust est expressif et ergonomique, rendant agréable l'écriture d'applications en ligne de commande, de serveurs web, etc.
**Question :** En quoi Rust est-il expressif et ergonomique pour différents types d'applications ?

### Paragraphe 6
- Cette documentation exploite le potentiel de Rust pour améliorer les compétences et l'assurance des développeurs.
**Question :** Comment ce livre aide-t-il les développeurs à améliorer leurs compétences et leur assurance en Rust ?


## Introduction
 Rust remet en cause le conflit entre l'ergonomie de haut-niveau et bas-niveau grace a l'equilibre entres ses capacites techniques.
### À qui s'adresse Rust
**Résumé :** Rust est idéal pour de nombreuses personnes, y compris les équipes de développeurs, les étudiants, les entreprises, les développeurs de logiciel libre, et ceux qui recherchent rapidité et stabilité.
**Question :** Quels sont les principaux groupes de personnes pour lesquels Rust est idéal et pourquoi ?
### Équipes de développeurs
**Résumé :** Rust est productif pour les grandes équipes de développeurs en prévenant les bogues subtils grâce à son compilateur et en offrant des outils modernes comme Cargo, Rustfmt, et le Rust Language Server.
**Question :** Comment Rust aide-t-il les grandes équipes de développeurs à prévenir les bogues subtils et à être plus productives ?
### Étudiants
**Résumé :** Rust est conçu pour les étudiants et ceux qui veulent apprendre les concepts système, avec une communauté accueillante et des ressources pédagogiques.
**Question :** Comment Rust facilite-t-il l'apprentissage des concepts système pour les étudiants ?

### Entreprises
**Résumé :** De nombreuses entreprises utilisent Rust en production pour diverses missions, allant des outils en ligne de commande aux systèmes embarqués et à l'apprentissage automatique.
**Question :** Pourquoi de nombreuses entreprises choisissent-elles d'utiliser Rust en production pour diverses missions ?

### Développeurs de logiciel libre
**Résumé :** Rust est ouvert aux contributions pour le développement du langage, de la communauté, des outils de développement et des bibliothèques.
**Question :** Comment les développeurs de logiciel libre peuvent-ils contribuer à l'écosystème Rust ?

### Les personnes qui recherchent la rapidité et la stabilité
**Résumé :** Rust offre rapidité et stabilité grâce à des vérifications du compilateur et des abstractions sans coût, permettant un code sûr et rapide.
**Question :** Comment Rust assure-t-il rapidité et stabilité dans le développement de logiciels ?

### À qui est destiné ce livre
**Résumé :** Ce livre est destiné à ceux qui ont déjà écrit du code dans un autre langage de programmation et vise à être accessible à un large éventail d'expériences de programmation.
**Question :** À qui ce livre sur Rust est-il destiné et comment est-il structuré pour être accessible ?

### Comment utiliser ce livre
**Résumé :** Ce livre est prévu pour être lu dans l'ordre, avec chaque chapitre s'appuyant sur les notions des chapitres précédents.
**Question :** Pourquoi est-il recommandé de lire ce livre dans l'ordre ?

**Résumé :** Le livre contient des chapitres théoriques et de projet pour apprendre et appliquer les concepts de Rust.
**Question :** Quels types de chapitres trouverez-vous dans ce livre et comment sont-ils structurés ?

**Résumé :** Le chapitre 1 couvre l'installation de Rust et les bases, tandis que le chapitre 2 offre une initiation pratique.
**Question :** Que couvrent les chapitres 1 et 2 de ce livre ?

**Résumé :** Les chapitres 3 à 6 couvrent les fonctionnalités de base de Rust, comme les structures, les méthodes, et les énumérations.
**Question :** Quels sujets sont abordés dans les chapitres 3 à 6 ?

**Résumé :** Le chapitre 7 traite des modules et de la visibilité, et le chapitre 8 des collections de données.
**Question :** Que couvrent les chapitres 7 et 8 ?

**Résumé :** Le chapitre 9 explore la gestion des erreurs en Rust.
**Question :** Quel est le sujet principal du chapitre 9 ?

**Résumé :** Le chapitre 10 couvre la généricité, les traits, et les durées de vie, et le chapitre 11 les techniques de test.
**Question :** Quels sujets sont abordés dans les chapitres 10 et 11 ?

**Résumé :** Le chapitre 12 est un projet pratique pour implémenter un programme en ligne de commande.
**Question :** Quel type de projet est abordé dans le chapitre 12 ?

**Résumé :** Le chapitre 13 explore les fermetures et les itérateurs, et le chapitre 14 approfondit Cargo et les bonnes pratiques.
**Question :** Quels sujets sont abordés dans les chapitres 13 et 14 ?

**Résumé :** Le chapitre 15 parle des pointeurs intelligents et des traits associés.
**Question :** Que couvre le chapitre 15 ?

**Résumé :** Le chapitre 16 traite de la programmation concurrente, et le chapitre 17 compare Rust à la programmation orientée objet.
**Question :** Quels sujets sont abordés dans les chapitres 16 et 17 ?

**Résumé :** Le chapitre 18 est une référence sur les motifs et le filtrage de motif.
**Question :** Quel est le sujet principal du chapitre 18 ?
**Résumé :** Le chapitre 19 couvre des sujets avancés comme le code non sécurisé et les macros.
**Question :** Quels sujets avancés sont abordés dans le chapitre 19 ?

**Résumé :** Le chapitre 20 est un projet pour implémenter un serveur web multitâches.
**Question :** Quel type de projet est abordé dans le chapitre 20 ?

**Résumé :** Les annexes fournissent des informations supplémentaires sur les mots-clés, les opérateurs, les traits dérivables, les outils de développement, et les éditions de Rust.
**Question :** Que contiennent les annexes de ce livre ?

**Résumé :** Il n'y a pas de mauvaise manière de lire ce livre, et vous pouvez sauter des étapes si nécessaire.
**Question :** Est-il possible de lire ce livre de manière non linéaire ?

**Résumé :** Comprendre les messages d'erreur du compilateur est crucial pour apprendre Rust, et le livre inclut des exemples de code qui ne compilent pas pour illustrer ces messages.
**Question :** Pourquoi est-il important de comprendre les messages d'erreur du compilateur en apprenant Rust ?

## Installation

**Résumé :** La première étape consiste à installer Rust via rustup, un outil en ligne de commande pour gérer les versions de Rust et les outils associés.
**Question :** Quelle est la première étape pour installer Rust et quel outil est utilisé ?


**Résumé :** Installer la dernière version stable de Rust garantit que les exemples du livre continueront à se compiler avec les nouvelles versions.
**Question :** Pourquoi est-il important d'installer la dernière version stable de Rust ?

**Résumé :** Les commandes en ligne de commande dans le livre commencent par $, et les résultats de commande ne commencent pas par $.
**Question :** Comment sont formatées les commandes en ligne de commande et leurs résultats dans ce livre ?

**Résumé :** Sous Windows, vous devez suivre les instructions sur le site officiel de Rust et installer les outils de compilation C++ pour Visual Studio.
**Question :** Quelles sont les étapes pour installer Rust sous Windows ?

**Résumé :** Mettre à jour Rust avec rustup est facile en utilisant la commande `rustup update`.
**Question :** Comment pouvez-vous mettre à jour Rust après l'avoir installé avec rustup ?

**Résumé :** Pour désinstaller Rust et rustup, utilisez la commande `rustup self uninstall`.
**Question :** Quelle commande permet de désinstaller Rust et rustup ?

**Résumé :** Pour vérifier l'installation de Rust, utilisez la commande `rustc --version` pour afficher la version installée.
**Question :** Comment pouvez-vous vérifier si Rust est correctement installé ?

**Résumé :** Si vous rencontrez des problèmes, vous pouvez obtenir de l'aide sur le canal #beginners du Discord officiel de Rust, le forum d'utilisateurs, ou Stack Overflow.
**Question :** Où pouvez-vous obtenir de l'aide si vous rencontrez des problèmes avec l'installation de Rust ?

**Résumé :** Rust inclut une copie locale de la documentation, accessible via la commande `rustup doc`.
**Question :** Comment pouvez-vous accéder à la documentation locale de Rust ?
## Hello, World!

**Résumé :** Écrire un programme "Hello, world!" est une tradition pour apprendre un nouveau langage, et Rust n'impose pas d'exigences sur les outils ou l'éditeur utilisés.
**Question :** Pourquoi est-il traditionnel d'écrire un programme "Hello, world!" en apprenant un nouveau langage ?

**Résumé :** Créez un dossier pour ranger le code Rust, comme un dossier projects dans votre dossier utilisateur.
**Question :** Où est-il suggéré de créer un dossier pour ranger le code Rust pour les exercices de ce livre ?

**Résumé :** Sous Windows, utilisez CMD pour créer un dossier de projet et naviguer dedans.
**Question :** Quelles commandes utilisez-vous sous Windows CMD pour créer et naviguer dans un dossier de projet ?

**Résumé :** Créez un fichier source main.rs et écrivez le code pour afficher "Hello, world!".
**Question :** Quel est le nom du fichier source à créer pour écrire le programme "Hello, world!" en Rust ?

**Résumé :** Compilez et exécutez le fichier main.rs pour afficher "Hello, world!" dans le terminal.
**Question :** Quelles commandes utilisez-vous sous Windows pour compiler et exécuter le fichier main.rs ?

**Résumé :** Si "Hello, world!" s'affiche, vous avez écrit votre premier programme Rust avec succès.
**Question :** Que signifie le fait de voir "Hello, world!" s'afficher dans le terminal après avoir exécuté le programme ?

**Résumé :** La fonction main est le point d'entrée de tout programme Rust, et son corps est entouré d'accolades.
**Question :** Quelle est la fonction spéciale qui sert de point d'entrée dans un programme Rust ?

**Résumé :** Utilisez rustfmt pour formater automatiquement le code de vos projets Rust.
**Question :** Quel outil pouvez-vous utiliser pour formater automatiquement le code de vos projets Rust ?

**Résumé :** La ligne println!("Hello, world!"); affiche le texte à l'écran et utilise une macro.
**Question :** Quelle est la différence entre println! et println en Rust ?

**Résumé :** La compilation et l'exécution sont des étapes séparées en Rust, produisant un binaire exécutable.
**Question :** Pourquoi la compilation et l'exécution sont-elles des étapes séparées en Rust ?

**Résumé :** Après compilation, vous pouvez exécuter le fichier binaire pour voir le résultat.
**Question :** Quelle commande utilisez-vous pour exécuter le fichier binaire compilé en Rust sous Windows ?

**Résumé :** Rust est un langage à compilation anticipée, permettant de distribuer des exécutables sans nécessiter Rust.
**Question :** Quelle est la différence entre un langage à compilation anticipée comme Rust et un langage dynamique comme Python ?

**Résumé :** Pour des programmes plus grands, utilisez Cargo pour gérer les options de compilation et le partage de code.
**Question :** Pourquoi est-il recommandé d'utiliser Cargo pour des programmes Rust plus grands ?
## Hello, Cargo!

**Résumé :** Cargo est le système de compilation et de gestion de paquets de Rust, facilitant la compilation et la gestion des dépendances.
**Question :** Quelles tâches Cargo gère-t-il pour les projets Rust ?

**Résumé :** Les programmes Rust simples n'ont pas de dépendances, mais Cargo facilite l'ajout de dépendances pour des projets plus complexes.
**Question :** Pourquoi est-il avantageux d'utiliser Cargo pour des projets Rust plus complexes ?

**Résumé :** La majorité des projets Rust utilisent Cargo, et il est installé avec Rust via l'installateur officiel.
**Question :** Comment pouvez-vous vérifier si Cargo est installé sur votre système ?

**Résumé :** Créez un nouveau projet avec Cargo en utilisant `cargo new` et naviguez dans le dossier créé.
**Question :** Quelle commande utilisez-vous pour créer un nouveau projet avec Cargo ?

**Résumé :** Cargo génère un fichier Cargo.toml et un dossier src avec un fichier main.rs, ainsi qu'un dépôt Git.
**Question :** Quels fichiers et dossiers Cargo génère-t-il lors de la création d'un nouveau projet ?

**Résumé :** Le fichier Cargo.toml contient des informations de configuration pour le projet, comme le nom, la version, et l'édition de Rust.
**Question :** Que contient le fichier Cargo.toml généré par Cargo ?

**Résumé :** Cargo génère un programme “Hello, world!” par défaut dans le fichier src/main.rs.
**Question :** Quel code Cargo génère-t-il par défaut dans le fichier src/main.rs ?

**Résumé :** Cargo aide à structurer les projets en séparant les fichiers sources dans le dossier src.
**Question :** Comment Cargo aide-t-il à structurer les projets Rust ?

**Résumé :** Vous pouvez transformer un projet existant en projet Cargo en déplaçant le code dans un dossier src et en créant un fichier Cargo.toml.
**Question :** Comment transformer un projet Rust existant en projet Cargo ?

**Résumé :** Compilez un projet Cargo avec `cargo build`, ce qui crée un exécutable dans le dossier target/debug.
**Question :** Quelle commande utilisez-vous pour compiler un projet Cargo et où se trouve l'exécutable généré ?

**Résumé :** Utilisez `cargo run` pour compiler et exécuter un projet en une seule commande.
**Question :** Quelle commande permet de compiler et d'exécuter un projet Cargo en une seule étape ?

**Résumé :** La commande `cargo check` vérifie rapidement si le code est compilable sans produire d'exécutable.
**Question :** Dans quel cas utiliseriez-vous la commande `cargo check` au lieu de `cargo build` ?

**Résumé :** Résumé des commandes Cargo : `cargo new`, `cargo build`, `cargo run`, et `cargo check`.
**Question :** Quelles sont les principales commandes Cargo et leurs fonctions ?

**Résumé :** Utilisez `cargo build --release` pour compiler un projet en mode optimisé pour la diffusion.
**Question :** Quelle commande utilisez-vous pour compiler un projet Cargo en mode optimisé pour la diffusion ?

**Résumé :** Cargo est particulièrement utile pour les projets complexes avec plusieurs crates, facilitant la coordination de la compilation.
**Question :** Pourquoi Cargo est-il particulièrement utile pour les projets Rust complexes ?

**Résumé :** Pour travailler sur un projet Rust existant, clonez le dépôt Git, naviguez dans le dossier projet, et compilez avec Cargo.
**Question :** Quelles étapes suivez-vous pour travailler sur un projet Rust existant avec Cargo ?

**Résumé :** Résumé des apprentissages : installation de Rust, mise à jour, documentation locale, programme “Hello, world!”, et utilisation de Cargo.
**Question :** Quels sont les principaux apprentissages de ce chapitre sur l'installation et l'utilisation de Rust et Cargo ?
## Programmer le jeu du plus ou du moins

**Résumé :** Ce chapitre présente des concepts courants en Rust à travers la création d'un jeu du plus ou du moins.
**Question :** Quels concepts courants en Rust seront abordés dans ce chapitre ?

**Résumé :** Le jeu consiste à deviner un nombre entre 1 et 100, avec des indications si le nombre est trop grand ou trop petit.
**Question :** Quel est le principe du jeu du plus ou du moins ?

**Résumé :** Utilisez Cargo pour créer un nouveau projet nommé jeu_du_plus_ou_du_moins.
**Question :** Quelle commande utilisez-vous pour créer un nouveau projet avec Cargo ?

**Résumé :** Le fichier Cargo.toml généré contient les informations de configuration du projet.
**Question :** Que contient le fichier Cargo.toml généré par Cargo ?


**Résumé :** Compilez et exécutez le programme “Hello, world!” avec la commande cargo run.
**Question :** Quelle commande permet de compiler et d'exécuter le programme “Hello, world!” en une seule étape ?

**Résumé :** Ouvrez le fichier src/main.rs pour écrire le code du jeu du plus ou du moins.
**Question :** Dans quel fichier allez-vous écrire le code du jeu du plus ou du moins ?

**Résumé :** Importez la bibliothèque std::io pour gérer la saisie utilisateur.
**Question :** Quelle bibliothèque devez-vous importer pour gérer la saisie utilisateur en Rust ?

**Résumé :** La fonction main est le point d'entrée du programme et affiche des messages à l'écran.
**Question :** Quelle est la fonction spéciale qui sert de point d'entrée dans un programme Rust ?

**Résumé :** Créez une variable mutable pour stocker la saisie de l'utilisateur avec let mut supposition = String::new().
**Question :** Comment créez-vous une variable mutable pour stocker la saisie de l'utilisateur en Rust ?

**Résumé :** Utilisez io::stdin().read_line(&mut supposition) pour lire la saisie utilisateur.
**Question :** Quelle méthode utilisez-vous pour lire la saisie utilisateur en Rust ?

**Résumé :** La méthode read_line retourne un io::Result, qui peut être Ok ou Err.
**Question :** Quelle est la valeur de retour de la méthode read_line en Rust ?

**Résumé :** Utilisez la méthode expect pour gérer les erreurs potentielles lors de la lecture de la saisie utilisateur.
**Question :** Quelle méthode utilisez-vous pour gérer les erreurs potentielles lors de la lecture de la saisie utilisateur ?

**Résumé :** Utilisez println! pour afficher la saisie utilisateur avec des espaces réservés {}.
**Question :** Comment affichez-vous des valeurs dynamiques dans une chaîne de caractères en Rust ?


**Résumé :** Testez le programme en utilisant la commande cargo run pour vérifier que la saisie utilisateur est correctement affichée.

**Question :** Quelle commande utilisez-vous pour tester le programme et vérifier que la saisie utilisateur est correctement affichée ?

### Génération d'un Nombre Secret
**Résumé :** Ce paragraphe explique qu'il faut générer un nombre secret aléatoire entre 1 et 100 pour un jeu, et comment la crate rand de Rust, qui n'est pas incluse dans la bibliothèque standard, permet de le faire.
**Question :** Quelle crate Rust est utilisée pour générer un nombre aléatoire dans ce jeu ?

### Extension des Fonctionnalités avec une Crate
**Résumé :** Ce paragraphe décrit ce qu'est une crate en Rust et comment rand est une crate de bibliothèque qui ne peut pas être exécutée seule. Il mentionne aussi la coordination des crates externes via Cargo.
**Question :** Quelle est la différence entre une crate binaire et une crate de bibliothèque en Rust ?

### Ajout de Dépendance dans Cargo.toml
**Résumé :** Ce paragraphe montre comment ajouter rand comme dépendance dans le fichier Cargo.toml et précise la version exacte à utiliser pour éviter les problèmes de compatibilité.
**Question :** Pourquoi on ajoute la crate rand au fichier Cargo.toml ?

### Processus de Compilation avec Cargo
**Résumé :** Après avoir ajouté une dépendance externe, Cargo télécharge et compile les crates nécessaires. Ce paragraphe décrit le processus de compilation avec la sortie typique de cargo build.
**Question :** Que fait Cargo après avoir ajouté une nouvelle dépendance et exécuté cargo build pour la première fois ?

### Répétabilité des Compilations avec Cargo.lock
**Résumé :** Le fichier Cargo.lock est créé pour garantir la reproductibilité des compilations en conservant les versions exactes des dépendances utilisées.
**Question :** Quel est le rôle du fichier Cargo.lock dans la gestion des dépendances de Rust ?

### Mise à Jour des Crates
**Résumé :** Ce paragraphe explique comment mettre à jour les crates en utilisant la commande cargo update, qui ignore le fichier Cargo.lock et cherche des versions plus récentes qui respectent les critères spécifiés dans Cargo.toml.
**Question :** Que se passe-t-il lorsque vous exécutez cargo update sur un projet Rust ?

### Mise à Jour vers une Nouvelle Version Majeure
**Résumé :** Ce paragraphe indique que pour mettre à jour vers une nouvelle série de versions majeures d'une crate, vous devez modifier le fichier Cargo.toml. Il suggère que des informations supplémentaires sur Cargo seront couvertes plus tard.
**Question :** Comment mettre à jour une crate vers une nouvelle série de versions majeures en Rust ?
### Générer un nombre aléatoire
**Résumé :** Ce paragraphe explique comment utiliser la crate rand pour générer un nombre aléatoire dans le jeu.
**Question :** Quelle crate Rust est utilisée pour générer un nombre aléatoire dans ce jeu ?

### Ajout du code pour générer un nombre aléatoire
**Résumé :** Ce paragraphe montre comment modifier src/main.rs pour inclure la génération d'un nombre aléatoire.
**Question :** Quelle méthode de la crate rand est utilisée pour générer un nombre aléatoire ?

### Utilisation de rand::Rng
**Résumé :** Le trait Rng définit les méthodes pour les générateurs de nombres aléatoires, et il doit être importé pour être utilisé.
**Question :** Pourquoi devons-nous importer le trait Rng dans notre code ?

### Génération du nombre aléatoire
**Résumé :** La fonction rand::thread_rng génère un générateur de nombres aléatoires pour le fil d'exécution courant.
**Question :** Quelle fonction de la crate rand génère un générateur de nombres aléatoires pour le fil d'exécution courant ?

### Affichage du nombre secret
**Résumé :** Afficher le nombre secret est utile pour les tests, mais sera supprimé dans la version finale.
**Question :** Pourquoi affichons-nous le nombre secret pendant le développement ?

### Tester le programme
**Résumé :** Ce paragraphe montre comment tester le programme pour vérifier que des nombres aléatoires différents sont générés.
**Question :** Quelle commande utilisez-vous pour tester le programme et vérifier que des nombres aléatoires différents sont générés ?

### Comparer le nombre saisi au nombre secret
**Résumé :** Ce paragraphe explique comment comparer la saisie utilisateur au nombre secret en utilisant std::cmp::Ordering.
**Question :** Quelle énumération est utilisée pour comparer la saisie utilisateur au nombre secret ?

### Utilisation de std::cmp::Ordering
**Résumé :** Le type Ordering a trois variantes : Less, Greater, et Equal, pour comparer deux valeurs.
**Question :** Quelles sont les trois variantes de l'énumération Ordering ?

### Expression match
**Résumé :** Une expression match compare une valeur à plusieurs motifs et exécute le code correspondant au motif correspondant.
**Question :** Comment une expression match fonctionne-t-elle en Rust ?

### Erreur de compilation
**Résumé :** Le code ne compile pas encore car Rust ne peut pas comparer une chaîne de caractères à un nombre.
**Question :** Pourquoi le code ne compile-t-il pas encore ?

### Conversion de la chaîne en nombre
**Résumé :** Ce paragraphe explique comment convertir la chaîne de caractères en un nombre pour la comparaison.
**Question :** Pourquoi devons-nous convertir la chaîne de caractères en un nombre ?

### Masquage des variables
**Résumé :** Le masquage permet de réutiliser le nom de variable supposition pour la conversion en nombre.
**Question :** Qu'est-ce que le masquage (shadowing) en Rust ?

### Utilisation de trim et parse
**Résumé :** La méthode trim enlève les espaces et parse convertit la chaîne en nombre.
**Question :** Que font les méthodes trim et parse sur une chaîne de caractères ?

### Gestion des erreurs avec parse
**Résumé :** La méthode parse retourne un Result, et expect gère les erreurs potentielles.
**Question :** Que retourne la méthode parse et comment gérons-nous les erreurs potentielles ?

### Test du programme
**Résumé :** Ce paragraphe montre comment tester le programme après avoir ajouté la conversion de la chaîne en nombre.
**Question :** Quelle commande utilisez-vous pour tester le programme après avoir ajouté la conversion de la chaîne en nombre ?

### Ajout d'une boucle pour plusieurs suppositions
**Résumé :** Ce paragraphe explique comment ajouter une boucle pour permettre plusieurs suppositions.
**Question :** Quel mot-clé crée une boucle infinie en Rust ?

### Problème de la boucle infinie
**Résumé :** Le programme demande un nombre à l'infini, ce qui pose un problème.
**Question :** Quel est le problème avec la boucle infinie dans le programme ?

### Arrêter le programme avec ctrl-c
**Résumé :** L'utilisateur peut interrompre le programme avec ctrl-c ou en saisissant une valeur non numérique.
**Question :** Comment l'utilisateur peut-il interrompre le programme ?

### Arrêter le programme après avoir gagné
**Résumé :** Ce paragraphe explique comment arrêter le programme après que l'utilisateur a deviné le bon nombre en utilisant break.
**Question :** Quelle instruction permet de sortir de la boucle après avoir deviné le bon nombre ?

### Gérer les saisies invalides
**Résumé :** Ce paragraphe montre comment gérer les saisies invalides en utilisant une expression match.
**Question :** Comment le programme gère-t-il les saisies invalides ?

### Test du programme avec saisies invalides
**Résumé :** Ce paragraphe montre comment tester le programme pour vérifier qu'il gère correctement les saisies invalides.
**Question :** Quelle commande utilisez-vous pour tester le programme après avoir ajouté la gestion des saisies invalides ?

### Code final du jeu du plus ou du moins
**Résumé :** Ce paragraphe présente le code final du jeu du plus ou du moins sans afficher le nombre secret.
**Question :** Quelle est la dernière étape pour finaliser le jeu du plus ou du moins ?

## Les Concepts courant de programmation

**Résumé :** Les concepts présentés ne sont pas spécifiques à Rust, mais nous les appliquerons à Rust et expliquerons les conventions associées.
**Question :** Les concepts présentés dans ce chapitre sont-ils spécifiques à Rust ?

**Résumé :** Vous allez apprendre les concepts de variables, types de base, fonctions, commentaires, et structures de contrôle.
**Question :** Quels concepts fondamentaux allez-vous apprendre dans ce chapitre ?

## Les variables et la mutabilité

**Résumé :** Par défaut, les variables en Rust sont immuables, ce qui aide à garantir la sécurité et la concurrence, mais elles peuvent être rendues mutables si nécessaire.
**Question :** Pourquoi Rust favorise-t-il l'immuabilité des variables par défaut ?

### Immuabilité des variables

**Résumé :** Lorsqu'une variable est immuable, sa valeur ne peut pas être changée après avoir été assignée.
**Question :** Que signifie l'immuabilité d'une variable en Rust ?

### Création d'un nouveau projet

**Résumé :** Créez un nouveau projet appelé variables pour explorer l'immutabilité et la mutabilité des variables.
**Question :** Quelle commande utilisez-vous pour créer un nouveau projet en Rust ?

### Exemple de code immuable

**Résumé :** Ce code montre une tentative de modification d'une variable immuable, ce qui entraîne une erreur de compilation.
**Question :** Que se passe-t-il lorsque vous essayez de modifier une variable immuable en Rust ?

### Message d'erreur de compilation

**Résumé :** Le message d'erreur indique qu'il est impossible d'assigner deux fois une variable immuable.
**Question :** Que signifie l'erreur de compilation "cannot assign twice to immutable variable" ?

### Importance des erreurs de compilation

**Résumé :** Les erreurs de compilation aident à identifier les problèmes de sécurité et de logique dans le code.
**Question :** Pourquoi les erreurs de compilation sont-elles importantes en Rust ?

### Mutabilité des variables

**Résumé :** La mutabilité peut être utile et facilite la rédaction du code, mais elle doit être explicitement déclarée avec mut.
**Question :** Comment rendez-vous une variable mutable en Rust ?

### Exemple de code mutable

**Résumé :** Ce code montre comment rendre une variable mutable et modifier sa valeur.
**Question :** Comment modifiez-vous la valeur d'une variable mutable en Rust ?

### Exécution du programme avec une variable mutable

**Résumé :** En utilisant mut, la valeur de x peut être modifiée de 5 à 6.
**Question :** Quelle est la sortie du programme lorsque la variable x est mutable et sa valeur est modifiée ?

### Compromis entre immuabilité et mutabilité

**Résumé :** La mutabilité peut prévenir les bogues et améliorer les performances, mais l'immutabilité peut rendre le code plus clair.
**Question :** Quels sont les compromis entre l'immutabilité et la mutabilité des variables en Rust ?

### Les constantes

**Résumé :** Les constantes sont des valeurs immuables déclarées avec le mot-clé const et doivent toujours avoir un type spécifié.
**Question :** Comment déclare-t-on une constante en Rust ?

### Immuabilité des constantes

**Résumé :** Les constantes sont toujours immuables et ne peuvent pas être modifiées après leur déclaration.
**Question :** Les constantes en Rust peuvent-elles être rendues mutables ?

### Déclaration des constantes

**Résumé :** Les constantes peuvent être déclarées n'importe où dans le code, y compris dans la portée globale.
**Question :** Où peut-on déclarer des constantes en Rust ?

### Expressions constantes

**Résumé :** Les constantes doivent être définies par des expressions constantes et non par des valeurs calculées à l'exécution.
**Question :** Par quoi doivent être définies les constantes en Rust ?

### Exemple de déclaration de constante

**Résumé :** Cet exemple montre comment déclarer une constante en utilisant une expression constante.
**Question :** Comment déclare-t-on une constante représentant trois heures en secondes en Rust ?

### Utilité des constantes

**Résumé :** Les constantes sont utiles pour des valeurs partagées par plusieurs parties du programme, comme des limites ou des constantes physiques.
**Question :** Pourquoi les constantes sont-elles utiles dans un programme Rust ?

### Avantages des constantes

**Résumé :** Déclarer des valeurs codées en dur comme constantes rend le code plus compréhensible et facilite les mises à jour.
**Question :** Quels sont les avantages de déclarer des valeurs codées en dur comme constantes ?

### Le masquage

**Résumé :** Le masquage permet de réutiliser le nom d'une variable en déclarant une nouvelle variable avec le même nom.
**Question :** Qu'est-ce que le masquage en Rust ?

### Exemple de masquage

**Résumé :** Cet exemple montre comment le masquage fonctionne en réutilisant le nom d'une variable pour effectuer des transformations.
**Question :** Comment fonctionne le masquage dans l'exemple donné ?

### Différence entre masquage et mutabilité

**Résumé :** Le masquage est différent de la mutabilité car il permet de réutiliser le nom de la variable sans la rendre mutable.
**Question :** Quelle est la différence entre le masquage et la mutabilité en Rust ?

### Changement de type avec le masquage

**Résumé :** Le masquage permet de changer le type d'une variable tout en réutilisant le même nom.
**Question :** Comment le masquage permet-il de changer le type d'une variable en Rust ?

### Exemple de changement de type

**Résumé :** Cet exemple montre comment utiliser le masquage pour changer le type d'une variable de chaîne de caractères à nombre.
**Question :** Comment le masquage est-il utilisé pour changer le type d'une variable dans l'exemple donné ?

### Erreur de compilation avec mutabilité

**Résumé :** Utiliser mut pour changer le type d'une variable entraîne une erreur de compilation.
**Question :** Que se passe-t-il si vous essayez de changer le type d'une variable mutable en Rust ?

### Types de données

**Résumé :** Maintenant que nous avons vu comment fonctionnent les variables, nous allons étudier les types de données qu'elles peuvent prendre.
**Question :** Quelle est la prochaine section après avoir étudié les variables en Rust ?
#### nombre entiers
**Résumé :**  il existe 12 types de nombres entiers
**Question :**  Combien de nombre d'entiers existe t-il ?

#### Types scalaires
***Résumé*** : Un type scalaire représente une seule valeur. Rust possède quatre types principaux de scalaires : les entiers, les nombres à virgule flottante, les booléens et les caractères.
 ***Question*** : Quels sont les quatre types principaux de scalaires en Rust ?

#### Types de nombres entiers
***Résumé*** : Un entier est un nombre sans partie décimale. Rust offre plusieurs types d'entiers, signés et non signés, avec des tailles allant de 8 bits à 128 bits, ainsi que des types dépendant de l'architecture (isize et usize). 
***Question*** : Quelle est la différence entre un entier signé et un entier non signé en Rust ?

#### Dépassement d'entier
***Résumé*** : En cas de dépassement d'entier, Rust peut paniquer en mode débogage ou effectuer un rebouclage en mode publication. Il existe des méthodes pour gérer explicitement les dépassements d'entiers. 
***Question*** : Que se passe-t-il en cas de dépassement d'entier en mode débogage et en mode publication en Rust ?

#### Types de nombres à virgule flottante
***Résumé*** : Rust possède deux types de nombres à virgule flottante : f32 et f64. Le type par défaut est f64 car il est plus précis et presque aussi rapide que f32 sur les processeurs modernes. 
***Question*** : Quels sont les deux types de nombres à virgule flottante en Rust et lequel est utilisé par défaut ?

#### Les opérations numériques
***Résumé*** : Rust offre des opérations mathématiques de base pour tous les types de nombres : addition, soustraction, multiplication, division et modulo.
 ***Question*** : Quelles sont les opérations mathématiques de base disponibles en Rust ?

#### Le type booléen
***Résumé*** : Le type booléen en Rust, désigné par bool, peut avoir deux valeurs : true et false. Il est principalement utilisé dans les structures conditionnelles. 
***Question ***: Quelles sont les deux valeurs possibles pour un type booléen en Rust ?

#### Le type caractère
***Résumé***: Le type char en Rust représente une valeur scalaire Unicode et prend quatre octets en mémoire. Il peut représenter plus de caractères que l'ASCII, y compris les emoji et les caractères accentués. 
***Question** : Quelle est la taille en mémoire d'un type char en Rust et que peut-il représenter ?

#### Le type tuple
***Résumé*** : Un tuple regroupe plusieurs valeurs de types différents en un seul type composé. Les tuples ont une taille fixe et peuvent être déstructurés pour accéder à leurs éléments. 
***Question*** : Comment accède-t-on aux éléments d'un tuple en Rust ?

#### Le type tableau
**Résumé*** : Un tableau en Rust contient plusieurs valeurs du même type et a une taille fixe. Les tableaux sont utiles pour des données allouées sur la pile ou lorsque le nombre d'éléments est fixe. 
***Question*** : Quelle est la différence entre un tableau et un vecteur en Rust ?

#### Accéder aux éléments d'un tableau
***Résumé** : Vous pouvez accéder aux éléments d'un tableau en utilisant l'indexation. Rust vérifie que l'indice est valide pour éviter les erreurs d'accès mémoire.
***Question*** : Comment accède-t-on aux éléments d'un tableau en Rust et que se passe-t-il si l'indice est invalide ?

#### Accès incorrect à un élément d'un tableau
***Résumé** : Si vous essayez d'accéder à un élément en dehors des limites du tableau, Rust va paniquer et arrêter l'exécution du programme pour éviter des erreurs d'accès mémoire. 
***Question*** : Que se passe-t-il si vous essayez d'accéder à un élément en dehors des limites d'un tableau en Rust ?

### Les fonctions
***Résumé*** : Les fonctions sont très utilisées en Rust. La fonction main est le point d'entrée de nombreux programmes. Les fonctions sont définies avec le mot-clé fn et utilisent le style de nommage snake case. ***Question*** : Quel mot-clé est utilisé pour définir une fonction en Rust ?

### Exemple de définition de fonction
***Résumé*** : Les fonctions sont définies avec fn suivi du nom de la fonction et d'une paire de parenthèses. Les accolades délimitent le corps de la fonction. Les fonctions peuvent être appelées depuis n'importe où dans le code. Question : Comment appelle-t-on une fonction définie dans un programme Rust ?

### Les paramètres
***Résumé*** : Les fonctions peuvent avoir des paramètres, qui sont des variables spéciales faisant partie de la signature de la fonction. Les paramètres doivent avoir un type explicitement déclaré.***Question*** : Pourquoi est-il nécessaire de déclarer le type des paramètres dans une fonction en Rust ?

### Exemple de fonction avec paramètres
***Résumé*** : Les paramètres sont déclarés dans la signature de la fonction. Lors de l'appel de la fonction, des valeurs concrètes (arguments) sont passées aux paramètres. ***Question*** : Comment passe-t-on des valeurs concrètes à une fonction avec des paramètres en Rust ?

### Instructions et expressions
***Résumé*** : Les instructions effectuent des actions et ne retournent aucune valeur, tandis que les expressions sont évaluées pour retourner une valeur. Rust est un langage basé sur des expressions.
***Question*** : Quelle est la différence entre une instruction et une expression en Rust ?

### Exemple d'instruction et d'expression
***Résumé***: La création d'une variable avec let est une instruction. Les expressions peuvent faire partie d'une instruction. Les expressions n'ont pas de point-virgule de fin de ligne. 
***Question*** : Pourquoi les expressions en Rust n'ont-elles pas de point-virgule de fin de ligne ?

### Les fonctions qui retournent des valeurs
***Résumé*** : Les fonctions peuvent retourner des valeurs. La valeur de retour est la même que la valeur de l'expression finale dans le corps de la fonction. Le type de retour est déclaré après une flèche (->). ***Question*** : Comment déclare-t-on le type de retour d'une fonction en Rust ?

### Exemple de fonction qui retourne une valeur
***Résumé*** : Une fonction peut retourner une valeur sans utiliser le mot-clé return. La dernière expression du corps de la fonction est la valeur de retour. 
***Question*** : Comment une fonction en Rust retourne-t-elle une valeur sans utiliser le mot-clé return ?

### Erreur de types inadéquats
***Résumé*** : Si une fonction déclare un type de retour mais ne retourne pas de valeur (ou retourne une valeur du mauvais type), Rust génère une erreur de types inadéquats.
***Question*** : Que se passe-t-il si une fonction en Rust déclare un type de retour mais ne retourne pas de valeur ?

### Les commentaires
Résumé : Les commentaires en Rust commencent par // et continuent jusqu'à la fin de la ligne. Pour les commentaires multi-lignes, chaque ligne doit commencer par //. Les commentaires peuvent être ajoutés à la fin d'une ligne de code ou sur une ligne séparée. Question : Comment écrit-on un commentaire en Rust ?