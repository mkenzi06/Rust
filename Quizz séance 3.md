- Voici une question pour vous montrer le format d'une réponse:
		Voici une réponse correctement indentée pour être replier avec la question.
		Merci d'ajouter vos réponses aux questions ci-dessous en respectant ce format.
		Répondez avec concision, mais avec le niveau de détail que vous donneriez à l'oral.
- Selon la page d'introduction du chapitre, combien de concepts y sont abordés ?
        Plusieurs concept ont etaient abordes comme : l'installation de Rust et comment lire le book, La communaute vise (developpeurs, etudiant, entreprises et Développeurs de logiciel libre), Fonctionnalités de Rust similaires à d'autres langages ainsi que les domaines d'utilisation.

- Qu'est-ce qu'un _mot clé_ ?
        Un mot-clé est un terme réservé dans un langage de programmation, comme Rust, qui a une fonction spécifique et ne peut pas être utilisé pour nommer des variables ou des fonctions. Par exemple, en Rust, des mots comme if, else, fn, ou let sont des mots-clés
- Tous les mots clés sont-ils associés à des fonctionnalités ?
        Non tous les mots-clés ne sont pas encore associés à des fonctionnalités dans Rust. Certains sont réservés pour des usages futurs, afin d'éviter qu'ils soient utilisés comme identificateurs.

- Où peut-on trouver la liste de tous les mots clés ?
        En annexe 1 du Rust Book present dans la documentation officielle du langage. 

- Vrai, faux, ou autres ? "Une variable correspond à toujours une "mémoire" pouvant prendre différentes 
valeurs durant une seule et même exécution."
        Faux En Rust, une variable peut être immuable par defaut, ce qui signifie que sa valeur ne peut pas changer après son initialisation, saud si on utilise le mot cle mut lors de la declaration.

- Comment déclarer une variable correspond à une "mémoire" pouvant prendre différentes valeurs durant une seule et même exécution ?
        En declarant la variable avec le mot cle `mut`.

- Vrai, faux, ou autres ? "Si un variable ne prends qu'une seule valeur au cours de l'exécution d'un programme, alors il est toujours possible de la déclarer comme une constante."
        Faux car une variable immuable ne permets pas d'etre consideree comme une constante ainsi il faudrait l'a declaree en tant que Constante avec une valeur determinee avant l'execution contrairement a une variable immuable.

- Vrai, faux, ou autres ? "Comme en Java, deux variables Rust différentes ont forcément des noms différents au sein d'un seul et même bloc de portée."
        en Rust on peut utiliser le masquage c'est a dire pouvoir donner un nom a  2 variables et les utiliser les 2 cependant seulement la derniere declaree sera utilisee (on peut meme changer de type) et cela ne provoque pas d'erreur.
- Vrai, faux, ou autres ? "Au sein d'un bloc de portée, un _nom_ de variable correspond à un unique et même type tout au long du bloc de portée."
        Faux grace au masquage on peut changer le type de la variable en utilisant le meme nom sauf si on utilise le mot cle let a la declaration de la variable du meme nom que celle dispo sur le bloc.




- Donner la liste des principaux types scalaires de Rust.
        les entiers, les nombres à virgule flottante, booleen ansi que les caracters.

- Combien y a-t-il de types de nombres entiers en Rust ?
        il y'a 6 taille pour les entiers chaque taille est pour un entier signe ou unsigned. donc en gros il existe 12.

- Combien de types de nombres entiers ont un nombre de bit indépendant de l'architecture vers laquelle le programme est compilé ?
        un seul type il s'agit du type archi qui donc depend de l'architecture donc 8 types sont independant de l'archi.


- Quelle est le type de nombres entiers par défaut ?
        le type d'entiers par defaut est i32.

- Quelles types de nombres entiers utilise-t-on pour indexer un tableau en Rust ?
        les types usize et isize sont utilises pour indexer un tableau.
- Quelle est la représentation binaire machine de la valeur -3 de type i16 ?
        la représentation binaire de -3 en i16 est 1111 1111 1111 1101.
- Donner le littéral correspond à -3 de type i16.
        Le littéral pour -3 en i16 est -3i16.
- Comment écrire le littéral correspond à "cent millions" en Rust de sorte à rendre facilement lisible que le nombre de zéro est le bon ?
        d'apres la doc c'est comme ceci 1_000_000
- Vrai, faux, ou autres ? "En Rust, si on incrémente une variable de type u8 dans une boucle infini, alors toutes les valeurs de 0 à 255 s'affiche en boucle et le programme ne s'arrête jamais."
        Vrai, si on incrémente une variable de type u8 dans une boucle infinie, elle va reboucler de 0 à 255 sans s'arrêter
- Donner la liste des types à virgule flottante.
        Les types à virgule flottante sont f32 et f64
- Vrai, faux, ou autres ? "Rust est un langage bas niveau, donc une variable à virgule flottante a une représentation différente en fonction de l'architecture vers laquelle on compile notre programme."
        Faux, les nombres à virgule flottante ont la même représentation sur toutes les architectures (IEEE-754).
- Vrai, faux, ou autres ? "Le type flottant par défaut a une taille de 32 bits."
        Faux, le type flottant par défaut est f64, qui a une taille de 64 bits.
- Où peut-on consulter la liste de tous les opérateurs numériques ?
        On peut consulter la liste des operateurs dans l'annexe B de la documentation officielle
- Vrai, faux, ou autres ? "Rust profite d'une panoplie de près de 200 opérateurs numériques."
        Faux, Rust ne possède pas 200 opérateurs numériques
- Vrai, faux, ou autres ? "En Rust, un test qui échoue (comme 10 > 30) renvoie 0 alors qu'un test qui réussi comme (10 < 30) renvoie 1."
        Faux, un test en Rust renvoie true ou false, pas 0 ou 1
- Vrai, faux, ou autres ? "Le type char représente 8 octets et représente un caractère ASCII."
        Faux, le type char représente un caractère Unicode, pas ASCII, et il occupe 4 octets

- Vrai, faux, ou autres ? "Il est possible d'ajouter des éléments dans un tableau Rust, modifiant ainsi sa taille, au cours de l'exécution d'un programme."
        Faux, un tableau a une taille fixe, mais on peut utiliser des vecteurs pour ajouter des éléments
- Si on a écrit un programme avec une variable x de type `(i32, i32, i32)`, décrire informellement comment le réécrire pour que son type soit maintenant `[i32; 3]` ? Le programme résultat est-il globalement plus court ? plus long ? de même longueur ?
        on declare un tableau de 3 element de type plus court mais la meme longueur.

- Si on a écrit un programme avec une variable x de type `[i32; 3]`, décrire informellement comment le réécrire pour que son type soit maintenant  `(i32, i32, i32)` ? Le programme résultat est-il globalement plus court ? plus long ? de même longueur ?
        Pour transformer un tableau [i32; 3] en tuple (i32, i32, i32), on change la declaration. Le programme peut être de même longueur ou un peu plus long.


- Que se passe-t-il en Rust si on tente d'accéder à la case 10 d'un tableau de taille 5 ? En quoi cela diffère de ce qui se passe en C ?
        une erreur qui dit que l'index ne correspond pas a la taille du tableau, le C n'a pas ce comportememnt et peut le faire indefiniment 

- Existe-t-il d'autres types Rust fonctionnant comme les tableaux ?
        oui il existe des vecteurs et des arrays
- Vrai, faux, ou autres ? "Comme en Java, Rust a besoin d'allouer de la mémoire et d'utiliser des pointeurs en interne pour faire fonctionner ces types composés."
         rust ne nécessite pas toujours l'utilisation de pointeurs de la même manière que Java. En Rust, la gestion de la mémoire se fait par le biais de la possession.
- Proposer une expression à mettre à la place des points d'intérrogation afin que le programme suivant compile et affiche "Yes" à l'exécution.

  ```rust
	fn main() {
	    let x = ???;
		if x.2[x.1].0 {
			println!("Yes");
		} else {
			println!("No");
		}
	}
	```
- Transformer le code précédent en une fonction recevant en paramètre la valeur de x.
  ```rust
	fn main() {
	    let x = ???;
		if x.2[x.1].0 {
			println!("Yes");
		} else {
			println!("No");
		}
	}
	```
- Modifier la fonction pour qu'elle prenne un paramètre la valeur de x et retourne un booléen au lieu d'afficher un message. Faites le bien "à la Rust", en suivant le style indiqué dans le chapitre.
    ```rust
	fn main() {
	    let x = x = (
        0, // Un entier quelconque
        1, // Index pour accéder à l'élément dans le tableau
        vec![
            (false,), // Premier élément du vecteur
            (true,),  // Deuxième élément du vecteur
        ],
    ); 
		if x.2[x.1].0 {
			println!("Yes");
		} else {
			println!("No");
		}
	}
	```
- Quelle convention utilise-t-on en Rust pour nommer les variables et les fonctions ?
        utiliser le snake_case tout en miniscule separe d'un underscore.
- Quel est normalement la différence technique entre "paramètre" et "argument" ?
        le parametre c'est les variables qu'on declare sur une fonction par exemple et l'argument c'est la valeur passee a la fonction 
- Le bout de code `let x = 10` est une expression ou une instruction ?
        let x = 10 est une instruction. En Rust, une instruction est une opération qui effectue quelque chose, mais qui n'a pas de valeur retournée. Dans ce cas, l'instruction crée une variable x et l'initialise avec la valeur 10
- Le bout de code `let x = 10` contient il une expression ?
        Oui, let x = 10 contient une expression. La valeur 10 est une expression qui produit un résultat, et cette valeur est utilisée pour initialiser x. En Rust, une expression est une unité de code qui produit une valeur.
- Vrai, faux, ou autres ? "Lorsqu'on déclare une variable, il est optionnel d'indiquer le type."
        Vrai. Lorsqu'on déclare une variable, il est optionnel d'indiquer le type en Rust
- Vrai, faux, ou autres ? "Lorsqu'on déclare un paramètre, il est optionnel d'indiquer le type."
        Lorsqu'on déclare un paramètre dans une fonction, il est nécessaire d'indiquer le type
- Vrai, faux, ou autres ? "En Rust, il n'existe qu'une seule façon de faire des commentaires."
        non il existe plusieurs manieres de faire des commentaires.
- Dans le code suivant, que faut-il mettre à la place des parenthèses pour que le code de la fonction compile (on ignorera les warnings):
	```rust
	fn blabla(x: i32) -> ??? {
		x + x
	}
	```
- Les `if/else` sont ils des instructions ou des expressions ?
        Les if statemenmt en Rust sont des expressions. Cela signifie qu'ils peuvent retourner une valeur
- Les `if` sans `else` sont ils des instructions ou des expressions ?
        Un if sans else est également une expression. Bien qu'il ne retourne pas de valeur par défaut s'il ne se produit pas, il peut toujours être utilisé dans un contexte d'expression
- Remanier le code suivant afin de n'avoir qu'un seul appel à `println!` et aucune variable supplémentaire.
    ```rust
	fn main() {
	    let x = ???;
		if x.2[x.1].0 {
			println!("Yes");
		} else {
			println!("No");
		}
	}
	```
- A quoi servent les étiquettes de boucle ?
        Les étiquettes de boucle servent à nommer une boucle pour pouvoir y faire référence depuis une instruction break ou continue.
- Une boucle `loop` est une expression. Quels types peuvent être retournés par un loop ?
        Une boucle loop peut retourner n'importe quel type, mais pour ce faire, vous devez utiliser une instruction break pour sortir de la boucle
- Une boucle `while` est une expression. Quels types peuvent être retournés par un while ?
        Comme pour les boucles loop, une boucle while peut également retourner n'importe quel type, à condition qu'une instruction break soit utilisée pour sortir de la boucle
- Les boucles `for` de Rust ressemble au boucle `for` de quels languages que vous connaissez ?
        Les boucles for de Rust ressemblent aux boucles for en Python et en JavaScript, car elles itèrent sur des collections ou des plages de valeurs
- Comme suggéré à la fin du chapitre, réaliser les programmes suivants:
	- Convertir des températures entre les degrés Fahrenheit et Celsius.
        ```rust
                fn main() {
                    let fahrenheit = 100.0; // Exemple de température
                    let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
                    println!("{} Fahrenheit est égal à {} Celsius", fahrenheit, celsius);
                    }   
            ```
	- Générer le _n_-ième nombre de Fibonacci.
		```rust
        fn fibonacci(n: u32) -> u32 {
            if n <= 1 {
                return n;
            }
            fibonacci(n - 1) + fibonacci(n - 2)
        }

        fn main() {
            let n = 10; // Exemple
            println!("Le {}-ième nombre de Fibonacci est {}", n, fibonacci(n));
        }
		```
	- Afficher les paroles de la chanson de Noël _The Twelve Days of Christmas_ en profitant de l'aspect répétitif de la chanson (https://www.lyricsforchristmas.com/christmas-carols/the-twelve-days-of-christmas/).
		```rust
            fn main() {
            let gifts = [
                "
                On the first day of Christmas
                my true love sent to me
                a partridge in a pear tree.",

                "On the second day of Christmas,
                my true love sent to me
                Two turtle doves,
                and a partridge in a pear tree.",

                "On the third day of Christmas,
                my true love sent to me
                Three French hens,
                Two turtle doves,
                And a partridge in a pear tree.",

                "On the fourth day of Christmas,
                my true love sent to me
                Four calling birds,
                Three French hens,
                Two turtle doves,
                And a partridge in a pear tree.",

                "On the fifth day of Christmas,
                my true love sent to me
                Five golden rings,
                Four calling birds,
                Three French hens,
                Two turtle doves,
                And a partridge in a pear tree."

                ,"On the sixth day of Christmas,
                my true love sent to me
                Six geese a-laying,
                Five golden rings,
                Four calling birds,
                Three French hens,
                Two turtle doves,
                And a partridge in a pear tree.",


                "On the seventh day of Christmas,
                my true love sent to me
                Seven swans a-swimming,
                Six geese a-laying,
                Five golden rings,
                Four calling birds,
                Three French hens,
                Two turtle doves,
                And a partridge in a pear tree.",

                "On the eighth day of Christmas,
                my true love sent to me
                Eight maids a-milking,
                Seven swans a-swimming,
                Six geese a-laying,
                Five golden rings,
                Four calling birds,
                Three French hens,
                Two turtle doves,
                And a partridge in a pear tree.",

                "On the ninth day of Christmas,
                my true love sent to me
                Nine ladies dancing,
                Eight maids a-milking,
                Seven swans a-swimming,
                Six geese a-laying,
                Five golden rings,
                Four calling birds,
                Three French hens,
                Two turtle doves,
                And a partridge in a pear tree.",

                "On the tenth day of Christmas,
                my true love sent to me
                Ten lords a-leaping,
                Nine ladies dancing,
                Eight maids a-milking,
                Seven swans a-swimming,
                Six geese a-laying,
                Five golden rings,
                Four calling birds,
                Three French hens,
                Two turtle doves,
                And a partridge in a pear tree.",

                "On the eleventh day of Christmas,
                my true love sent to me
                Eleven pipers piping,
                Ten lords a-leaping,
                Nine ladies dancing,
                Eight maids a-milking,
                Seven swans a-swimming,
                Six geese a-laying,
                Five golden rings,
                Four calling birds,
                Three French hens,
                Two turtle doves,
                And a partridge in a pear tree.",

                "On the twelfth day of Christmas,
                my true love sent to me
                Twelve drummers drumming,
                Eleven pipers piping,
                Ten lords a-leaping,
                Nine ladies dancing,
                Eight maids a-milking,
                Seven swans a-swimming,
                Six geese a-laying,
                Five golden rings,
                Four calling birds,
                Three French hens,
                Two turtle doves,
                And a partridge in a pear tree!"
            ];

            for i in 0..gifts.len() {
                println!("{}", gifts[i]);
                println!(); 
            }
        }
		
		```