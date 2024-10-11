# Chapitre 5

- Les structures sont similaires à une famille de types vue au chapitre 3, laquelle ?

		Les structures sont similaires aux tuples, car tout les deux portes des valeurs associees.
- Quelle est la différence entre cette famille de types du chapitre 3 et les structures ?

		La différence est qu'on doit nommer chaque élement aux données d'une structure afin de clarifier le role de chaque valeur

- Comment appelle-t-on les différentes parties qui composent une structure ?

		On commence par le nom de la structure ensuite la partie accolades qui contient le nom et le type de chaque élement donnée on appelle ca un champ.

- Deviner si le code suivant compile ou non (en ignorant les warnings):

  ```rust
			struct A { n: i32, m: f64 }
			fn f (x: A) {
				 println!("{} {}", x.n, x.m);
				  }
			
			struct B { n: i32, m: f64 }
			fn main() {
			let x = B { n: 0, m: 0.0 };
			println!("{} {}", x.n, x.m);
			f(x);
			}
	```

		Normalement la fonction f attend en parametre d'entree une instance de type A comme defini sur la fonction et x est de type B donc le code ne devrait pas compiler avec une erreur Expected A found B.
- Vrai, faux ou autre ? "Lorsqu'on instancie une structure, il faut donner les champs dans le même ordre que lors de la déclaration de la structure."


		Faux nous n'avons pas à préciser les champs dans le même ordre qu'on les a déclarés dans la structure.

- Vrai, faux ou autre ? "Il est possible de ne pas donner de nom au champs des structures Rust."


		Faux, il est nécessaire de donner un nom a chaque champs.

- Vrai, faux ou autre ? "Dans le code suivant, les deux instances `x` et `y` des structures unités `X` et `Y` sont égales, car les instances de structures unités sont toutes égales."


  ```rust
	struct X;
	struct Y;
	fn main() {
		let x = X;
		let y = Y;
	}
	``` 

		Faux les instances x et y ne sont pas égales car elle sont des instances de 2 types différents.

	
- Déclarer une structure décrivant un livre par son titre, sa description et son nombre de chapitre.


	```rust
	struct Book {
    title: String,
    description: String,
    nbr_chapter: u32,
	}
	```

- Donner l'*expression* (et uniquement elle sans rien autour) créant une instance de votre structure pour le Rust Book.


	```rust
	let bookrust = Book {
		title: String::from("Rust Book"),
		description: String::from(""),
		nbr_chapter: 1,
	}
	```

- Comment indiquer que le champ description de votre structure est mutable mais pas le titre ni le nombre de chapitre ?


		Rust ne nous permet pas de marquer seulement certains champs comme mutables donc l'instance toute entiere doit etre mutable


- Donner le code d'une fonction `creer_livre` qui créer un livre avec un titre et un nombre de chapitre reçu en paramètre et retourne ce livre avec une description vide. Donner deux versions de votre code: l'une sans raccourci d'initialisation de champ et l'autre avec.


		```rust
		fn creer_livre(title: String, nbr_chapter: u32) -> Book {
			Book {
				title: title,
				description: String::from("Je suis la description de rust"),
				nbr_chapter: nbr_chapter,
			}
		}
		```

		```rust
		fn creer_livre(title: String, nbr_chapter: u32) -> Book {
			Book {
				title,
				description: String::from(""),
				nbr_chapter,
			}
		}
		```
	
- En combinant la syntaxe de mise à jour de structure et un raccourci d'initialisation de champ, créer une fonction `nouvelle_description` qui reçoit un livre et une description et retourne un livre identique à celui en premier paramètre hormis pour sa description est celle reçue en second paramètre. **⚠ Ne pas utiliser de référence ! ⚠** 


	```rust
		fn nouvelle_description(Book: book, description: String) -> Book
		{
			Book{
				description,
				..book
			}

		}
	```

- Écrire une fonction `main` qui instance un livre `l1` via la fonction `creer_livre`, demande à l'utilisateur une description que l'on va stocker dans une variable `description_utilisateur` et instancie un autre livre `l2` via la fonction `nouvelle_description` appelée avec `l1` et `description_utilisateur`. Votre fonction `main` doit afficher le contenu de toutes les variables **encore en vigueur** juste avant de se terminer, en utilisant la macro `dbg!`.



		```rust
		fn main(){
			let l1 = creer_livre(("Rust Book"),10);
			print!("Entrez une description pour le livre: ");
			let mut description_utilisateur = String::new();
			io::stdin().read_line(&mut description_utilisateur).expect("Échec de la lecture de l'entrée");
			let l2 = nouvelle_description(l1, description_utilisateur);
    		dbg!(&l2);
			
		}
		```


- En utilisant la méthode clone vue au chapitre 3, écrire une fonction `dupliquer_livre` qui reçoit une référence vers un livre et retourne un nouveau livre avec le même titre, la même description et le même nombre de chapitre.


		```rust
			fn dupliquer_livre(book: &Book) - > Book{
			Book {
			titre: book.title.clone(), // On clone chaque attribut
			description: book.description.clone(),
			nombre_chapitres: book.nbr_chapter.clone(),
		}
			}
		```

- Écrire une nouvelle version de `nouvelle_description` appelée `changer_description` qui reçoit en paramètre une *référence mutable* vers un livre ainsi qu'une description et modifie le champ description du livre pour y mettre la nouvelle description.



		```rust
		fn changer_description(livre:&mut Book, description : String){
			livre.description = description;
		}
		```


- On souhaiterait éviter de cloner inutilement des chaînes de caractères qui vont en réalité rester identiques. Le type `&str` permet justement à plusieurs variable de pointé vers la même zone mémoire de chaîne de caractère, mais pas le type `String`. Quelle fonctionnalité de Rust faudrait-il utiliser pour remplacer les `String` de votre structure livre par des `&str` ?


		on devrait utiliser des durees de vie pour mettre des references a des champs dans une structure.

- Remanier votre code de sorte à ce que `creer_livre`, `nouvelle_description` et `changer_description` soient des fonctions associées à votre structure. Mettre à jour votre fonction `main` en conséquence et utiliser la syntaxe d'appel de méthode lorsque c'est possible.


		```rust
			struct Book {
		title: String,
		description: String,
		nbr_chapter: u32,
	}

	impl Book {
		
		fn creer_livre(title: String, nbr_chapter: u32) -> Book {
			Book {
				title: title,
				description: String::from("Je suis la description de rust"),
				nbr_chapter: nbr_chapter,
			}
		}

		
		fn changer_description(&mut self , nouvelle_description: String) {
			livre.description = nouvelle_description;
		}

		
		fn nouvelle_description(Book: book, description: String) -> Book
		{
			Book{
				description,
				..book
			}

		}

	}

		```


- Donner un exemple de votre nouveau `main` où _référencement et déréférencement automatiques_ de Rust lors rend l'appel de méthodes plus simple.


		```rust
		let mut livre = Book::creer_livre(String::from("Rust Programming"), 12);

    
    livre.changer_description(String::from("Un guide complet sur Rust."));

  
    let livre2 = livre.nouvelle_description(String::from("Un manuel avancé sur Rust."));
		```




- Qu'est-ce qu'un bloc `impl` ? Est-il possible d'en avoir plusieurs pour une seule structure ?


		Dans un bloc impl on peut definir les methodes associees a une structure et on peut utiliser plusieurs bloc impl dans une seule structure

- La notation `&self` en premier paramètre d'un déclaration de méthode est un raccourci. Quelle est la forme complète ?


		La forme complete est self: &Self.


- Vrai ou faux ? "Comme les méthodes et les champs d'une structure s'invoquent tous les deux avec la notation pointé `x.nom`, une méthode ne peut pas porter le même nom d'un champ."
		
		Faux une methode peut porter le meme nom d'un champs.
- Donner un exemple d'attribut externe.
		
		#[derive(Debug)]
# Chapitre 6
	

- Définir une énumération permettant de décrire une couleur qui peut soit être en RGB ou en CMYK, et faire une fonction `main` de teste qui les instancie et affiche une instance de chaque variante en utilisant `dbg!`.



			```rust
	enum Color {
		RGB(u8, u8, u8),        
		CMYK(u8, u8, u8, u8),   
	}
		fn main() {
	
		let couleur_rgb = Color::RGB(0, 0, 0); 
		
		let couleur_cmyk = Color::CMYK(0, 100, 100, 0); 

		// Affichage des couleurs avec dbg!
		dbg!(&couleur_rgb);  
		dbg!(&couleur_cmyk); 
	}
		```


- Définir une structure permettant de décrire un carte par sa "couleur" (qui est une énumération dont les variantes sont cœur, trèfle, carreau et pique) et sa "valeur" (qui est une énumération dans la première variante est muni d'un `u8` précisant la valeur en 1 et 10 et trois variantes supplémentaires correspondent à valet, dame et roi).



		```rust
			enum Couleur {
				Coeur,
				Trefle,
				Carreau,
				Pique,
			}

			
			
			enum Valeur {
				Numerique(u8), // Valeurs de 1 à 10
				Valet,
				Dame,
				Roi,
			}

			
			struct Carte {
				couleur: Couleur,
				valeur: Valeur,
			}
		```
	

- Faire un programme qui instancie toutes les cartes possibles et les affiche avec `dbg!`.



		```rust
	fn main() {
		
		let carte1 = Carte {
			couleur: Couleur::Coeur,
			valeur: Valeur::Numerique(10),
		};

		let carte2 = Carte {
			couleur: Couleur::Trefle,
			valeur: Valeur::Valet,
		};

		// Affichage des cartes avec dbg!
		dbg!(&carte1); 
		dbg!(&carte2); 
	}
		```



- Vrai ou faux ? "Dans un langage utilisé le concept de valeur nulle, n'importe quelle variable peut potentiellement contenir null et c'est au programmeur de savoir si cette possibilité est effectivement ou non, ce qui pose un problème si le programmeur pense que ce n'est pas possible alors qu'en fait la valeur nulle est possible"


		Vrai et il appartient au programmeur de gérer cette possibilite. Cela peut entraîner des erreurs d'exécution si le programmeur n'est pas vigilant.


- Vrai ou faux ? "En Rust, une valeur pouvant être invalide ou non-disponible à pour type `Option<T>` alors qu'une valeur qui est toujours valide a pour type `T` directement, donc le programmeur ne peut pas se tromper et doit travailler différent dans un cas et dans l'autre."


		Vrai cela oblige au programmeur de travailler differement pour traiter les valeurs

- Donner le code d'une fonction `seulement_positif` qui reçoit une valeur `i32` et retourne un `Option<i32>` qui est `None` si la valeur est strictement négative, et `Some(v)` sinon.


	```rust 
	fn seulement_positif(v: i32) -> Option<i32> {
		if v < 0 {
			None
		} else {
			Some(v)
		}
	}
	```

- En utilisant `match`, faire une fonction `valeur_non_atout` qui reçoit une carte (voir question précédente) et retourne sa valeur à la belote (https://fr.wikipedia.org/wiki/Belote#Valeur_des_cartes) en supposant que les cartes de couleur cœur sont atout et les autres couleurs non.


	```rust
	#[derive(Debug)]
		enum Couleur {
			Coeur,
			Trèfle,
			Carreau,
			Pique,
		}
		#[derive(Debug)]
		enum Figure {
			Valet,
			Dame,
			Roi,
		}
		#[derive(Debug)]
		struct Valeur {
			numerique: Option<u8>, 
			figure: Option<Figure>, 
		}

		#[derive(Debug)]
		struct Carte {
			couleur: Couleur,
			valeur: Valeur,
		}

		fn valeur_non_atout(carte: &Carte) -> Option<i32> {
			match &carte.couleur {
				Couleur::Coeur => {
					// Atout
					match &carte.valeur {
						Valeur { numerique: Some(1), figure: None } => Some(11), 
						Valeur { numerique: Some(10), figure: None } => Some(10), 
						Valeur { numerique: None, figure: Some(Figure::Roi) } => Some(4), 
						Valeur { numerique: None, figure: Some(Figure::Dame) } => Some(3), 
						Valeur { numerique: None, figure: Some(Figure::Valet) } => Some(2), 
						Valeur { numerique: Some(_) , figure: None } => Some(0), 
						Valeur { numerique: None, figure: None } => None,
					}
				},
				_ => {
					
					match &carte.valeur {
						Valeur { numerique: Some(1), figure: None } => Some(11), 
						Valeur { numerique: Some(10), figure: None } => Some(10), 
						Valeur { numerique: None, figure: Some(Figure::Roi) } => Some(4), 
						Valeur { numerique: None, figure: Some(Figure::Dame) } => Some(3), 
						Valeur { numerique: None, figure: Some(Figure::Valet) } => Some(2), 
						Valeur { numerique: Some(_) , figure: None } => Some(0), 
						Valeur { numerique: None, figure: None } => None, 
					}
				}
			}
		}
	```
- Faire une version avec `match` et une version avec `if let` d'une fonction recevant deux `Option<i32>` et retournant leur somme quand c'est possible et `None` sinon.



		```rust
			fn somme_let(a: Option<i32>, b: Option<i32>) -> Option<i32> {
				if let(Some(x), Some(y))= (a,b) {
					Some(x+y)	
				} else {
					None
				}
			}
		```