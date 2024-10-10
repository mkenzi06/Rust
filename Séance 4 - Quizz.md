- Quels sont les 4 thèmes abordés dans ce chapitre ?
		Les 4 themes abordes dans ce chapitre sont la possession, l'emprunt, les slices et comment rust agence les donnees en memoire.
- Quelles sont les deux approches traditionnelles de gestion de la mémoire ?
		Le ramasse miettes qui scrute constamment la memoire qui n'est plus utilisee oendant qu'elle s'execute la deuxieme c'est le developpeur qui alloue et libere de la memoire explicitement. 
- Est-il a priori rapide de se familiariser avec le concept de possession ?
		Non le concept de possession est un nouveau concept donc cela prend un certain temps pour s'y familiariser.
- Quelle structure de donnée est utilisé dans le chapitre pour exemplifier la possession ?
		La strcuture de donnee utiliser dans le chapitre sont les chaines de caracteres (String).
		
- Connaîssez-vous un autre langage qui demande la maitrise de la différence pile/tas ?
		Oui le langage assembleur et le C demande des connaissances sur la maitrise de la difference pile/tas.
- (Hors Livre) En gros, quelles instructions assembleurs permettent de manipuler la pile, et comment la pile est-elle gérée ? et par quoi ? processeur/système d'exploitation/bibliothèque ?
		En utilisant Push pour mettre un ellement au sommet de la pile, popq pour recuperer la valeur du sommet de la pile

- (Hors Livre) En gros, quelles instructions assembleurs permettent de manipuler le tas, et comment le tas est-il géré ? et par quoi ? processeur/système d'exploitation/bibliothèque ?
		il n'a ya pas de gestion sur le tas en assembleur.
- (Hors Livre) Pourquoi il est mieux de consulter des données qui sont proches les unes des autres pour un processeur moderne ?
		il est mieux de consulter des donnees qui sont proche les unes des autres car le gestionnaire prefere travailler sur des donnees proches les unes des autres que de plutot allouer une grande quantite de memoire qui prend beaucoup plus de temps(tas).
- Quel genre de données sont stockées sur la pile ?
		Tout type de donnees fixe variables locales, parametres de fonction 
- Quel est le but principale de la possession ?
		Le but principale de la possession est de gerer les donnees du tas (minimiser la quantite de donnees sur le tas etc...)
- Quand est-ce qu'une valeur est supprimée ?
		Une valeur est supprimee quand son proprietaire sort de la portée.
- Vrai, faux ou autre ? "Une variable est "en vigueur" de sa déclaration jusqu'à la fin de l'exécution de programme."
		Faux une variable est en vigueur jusqu'a la fin de sa portee.
- En terme de pile et de tas, quelle est la différence entre les types du chapitre 3 et le type String ? Pourquoi cette différence est-elle nécessaire ?
		La difference entre les types du chaptre 3 et le type string est que les autres types etaient stockes sur la pile et qui permet l'acces rapide sur ces donnees, contrairement aux string le type est plus complexe et est stocke sur le tas ce qui permet de gerer des quantite de texte dont on connait pas la taille et en terme de porte Rust nettoie automatiquement la memoire alloue sur le tas lors de l'arrivee a sa portee

- Donner une expression de type String.
		```rust
		let x = String::from("Hello");
		```
- Les littéraux comme ", world" sont-ils de type String ?
		Non les litteraux ne sont pas de type string, il faut reference a une chaine de caracteres immuable contrairement au string qui peut etre mutable.

- Vrai, faux ou autre ? "Les chaînes de caractères littérales et celles saisies par l'utilisateur sont stockés dans la même zone mémoire."
		Faux les chaines de caracteres saisies par l'utilisateur sont stockes sur le tas contrairement aux litterales qui elles sont stockes sur la pile.
- Quelle fonction est automatiquement appelée lorsque qu'un String sort de sa portée ?
		Lorsque au'un string sort de sa portee la fonction drop est appellee pour liberer de la memoire
- Une valeur de type String nécessite de la mémoire: uniquement dans le tas, uniquement sur la pile, ou sur les deux ?
		sur les deux, sur la pile on a le pointeur vers les donnees reelles de la chaine de caracteres stockes sur le tas
- Qu'est-ce qu'une "double libérations" (ou "double free")  et son rapport avec la sécurité informatique ?
		La double liberations peut mener a des corruptions de memoire ainsi influencer sur la vulnerabilite de la securite informatique
- Étant donnée la façon dont Rust gère la mémoire, pourquoi serait-il un problème que deux pointeurs possèdent la même mémoire dans la tas en même temps ?
		Car elle pourraient etre couteuse en terme de performance d'execution si les donnees sur le tas etaient volumineuses et en terme de securite sur la double liberations

- Vrai, faux ou autre ? "Une instruction telle que `let y = x;` peut parfois représentée une copie profonde couteuse."
		En rust la variable x serait neutralises au lieu d'appeller cela une copie on appelle ca un deplacement.
- Quelle est la différence entre un déplacement et une copie superficielle ?
		l'idée de copier le pointeur, la taille et la capacité sans copier les données peut vous faire penser à de la copie superficielle. Mais comme Rust neutralise aussi la première variable, au lieu d'appeler cela une copie superficielle, on appelle cela un déplacement
- Vrai, faux ou autre ? "Si on souhaite qu'un type fasse une copie il *suffit simplement* d'annoter ce type avec le trait Copy."
		Faux l'annotations copy ne suffit pas le type devrait etre sur la pile et donc avoir une taille fixe et rapide d'accees 
- Vrai, faux ou autre ? "Tout ce qui est alloué durant l'exécution d'une fonction est désallouée à la fin de l'exécution de la fonction."
		Concernant les string par exemple elle est desalloue a la fin de sa portee.
---
- Vrai, faux ou autre ? "Pour des raisons de sécurité et de cohérence, une fonction ne peut désallouer que ce qu'elle a alloué."
		Vrai

- Vrai, faux ou autre ? "Les fonctions n'introduisent aucunes subtilités sur la gestion de la possession: pour comprendre la possession il suffit comprendre ce qui se passe avec des variables."
		Faux les fonctions introduisent aussi de la possession sur les variables et leur portee.

- Dans le même style que l'encart 4-5, écrire une fonction `meme_longueur` qui reçoit deux String, et retourne un booléen indiquant s'ils ont la même taille, ainsi que les deux String pour rendre leur propriété. Écrire une fonction main similaire à celle de l'encart pour tester votre fonction.
  ```rust
	fn main() {
		let s1 = String::from("hello");
		let s2 = String::from("word");

		let same_length = meme_longueur(&s1, &s2);

		println!("alors les deux chaines ont la meme longueur? {}", same_length);
	}

	fn meme_longueur(s1: &String,s2: &String) -> (bool) {
		let same_length = s1.len() == s2.len();
		same_length
	}
	```
- Réécrire votre code précédent afin de profiter des références Rust.
  ```rust
				fn main() {
		let s1 = String::from("hello");
		let s2 = String::from("word");

		let same_length = meme_longueur(&s1, &s2);

		println!("alors les deux chaines ont la meme longueur? {}", same_length);
	}

	fn meme_longueur(s1: &String,s2: &String) -> (bool) {
		let same_length = s1.len() == s2.len();
		same_length
	}

	```
- Vrai, faux ou autre ? "Lorsque d'une fonction à une référence vers un String, cette fonction *possède* ce String, et par les règles de la possession, cette String est désalloué à la fin de la fonction."
		Faux elle fait reference a un string sans prendre la possession donc elle fait rien
- Vrai, faux ou autre ? "Lorsque d'une fonction à une référence vers un String peut également modifier le String".
		Faux Nous ne sommes pas autorisés à modifier une chose quand nous avons une référence vers un string
- Votre fonction `meme_longueur` précédente (version avec référence) peut-elle être appelé avec deux références identique, i.e. `meme_longueur(&s, &s)` ?
		Oui elle peut etre appeler avec 2 references identiques.
- Tentez d'écrire de mémoire les règles sur les emprunts.
		Les emprunts immuables, mutable et dangling pointer.

- (question volontairement flou) Donner différents exemples d'emprunts invalides et essayer d'imaginer, pour chacun d'eux, un contexte dans lequel cela provoquerait un bogue.
		l'utilisaton d'une seule reference plusieurs fois au meme moment ainsi que l'emprunt mutable et immuable en simultane

- Quels soucis pourraient survenir si on choisi de faire une fonction `second_mot(&String) -> (usize,usize)` qui se contente de retourner la taille du premier et laisse le programmeur utiliser le String d'origine et cette taille lorsqu'il veut travailler avec le premier mot ?
		le meilleur moyen est d'utiliser un slice pour garantir que les references sont calide


- Y aurait-il une façon de résoudre ces soucis sans utiliser le concept de slice ? Quel compromis faut-il faire ?
		Oui mais cela implique d'utiliser une nouvelle string
- Si on a un variable `s` de type `String`, quel est le type de `&s` ? Vers quoi pointe le pointeur sous-jacent de `&s` ? quel est le type de `&s[..]` ?  Vers quoi pointe le pointeur sous-jacent de `&s[..]` ?
		type de &s : &string reference immuable vers un string
		pointeur sou-jacent de &s pointeur vers la valeur original
		type de &s[..] : &str reference immuable vers une tranche de caracteres 
		pointeur sous-jacent pointeur vers le premier caractere de la chaine