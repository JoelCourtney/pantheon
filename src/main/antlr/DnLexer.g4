lexer grammar DnLexer;

fragment DIGIT : [0-9];
fragment LETTER : [A-Z] | [a-z] | '_';

fragment A : 'a' | 'A';
fragment B : 'b' | 'B';
fragment C : 'c' | 'C';
fragment D : 'd' | 'D';
fragment E : 'e' | 'E';
fragment F : 'f' | 'F';
fragment G : 'g' | 'G';
fragment H : 'h' | 'H';
fragment I : 'i' | 'I';
fragment J : 'j' | 'J';
fragment K : 'k' | 'K';
fragment L : 'l' | 'L';
fragment M : 'm' | 'M';
fragment N : 'n' | 'N';
fragment O : 'o' | 'O';
fragment P : 'p' | 'P';
fragment Q : 'q' | 'Q';
fragment R : 'r' | 'R';
fragment S : 's' | 'S';
fragment T : 't' | 'T';
fragment U : 'u' | 'U';
fragment V : 'v' | 'V';
fragment W : 'w' | 'W';
fragment X : 'x' | 'X';
fragment Y : 'y' | 'Y';
fragment Z : 'z' | 'Z';

ACTION : A C T I O N S?;
BONUS_ACTION
    : B O N U S ' ' A C T I O N S?
    | B O N L E S S ' ' A C T I O N S?;
REACTION : R E A C T I O N S?;
ROUND : R O U N D S?;
SECOND : S E C (O N D)? S?;
MINUTE : M I N (U T E)? S?;
HOUR : H O U R S? | H R S?;
DAY : D A Y S?;
LONG_REST : L O N G ' ' R E S T S?;
SHORT_REST : S H O R T ' ' R E S T S?;
ACID : A C I D;
BLUDGEONING : B L U D G E O N I N G;
COLD : C O L D;
FIRE : F I R E;
FORCE : F O R C E;
LIGHTNING : L I G H T N I N G;
NECROTIC : N E C R O T I C;
PIERCING : P I E R C I N G;
POISON : P O I S O N;
PSYCHIC : P S Y C H I C;
RADIANT : R A D I A N T;
SLASHING : S L A S H I N G;
THUNDER : T H U N D E R;
FOOT : F (E E | O O)? T;
MILE : M I (L E S?)?;
SPACE : S P (A C E S?)?;
INSTANTANEOUS : I N S T A N T (A N E O U S)?;
INDEFINITE : I N (D E)? F I N I T E;
TOUCH : T O U C H;
SELF : S E L F;

NUMBER : DIGIT+;
NAME : LETTER (LETTER | DIGIT)*;
DICE : DIGIT+ D DIGIT+;

PLUS : '+';
MINUS : '-';
TIMES : '*';
DIVIDE_DOWN : '/' | '/-';
DIVIDE_UP : '/+';
OPEN_PAREN : '(';
CLOSE_PAREN : ')';
EQUAL : '=';
COMMA : ',';
DOLLAR : '$';
SEMICOLON : ';';

WS : [ \t\u000c]+ -> skip;
