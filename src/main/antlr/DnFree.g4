grammar DnFree;

@parser::header {
    import quantities.*;
}

@lexer::header {

}

@parser::members {

}

@lexer::members {

}

time_quantity returns [Quantity<Time> result]
    : time_component        { $result = $time_component.result; }
    | a=time_component bop=( '+' | '-' ) b=time_component
        { $result = new QuantityBinary<Time>(QuantityBinaryOp.fromString($bop.getText()), $a.result, $b.result); }
    | INSTANTANEOUS         { $result = TimeKeyword.INSTANTANEOUS; }
    | INDEFINITE            { $result = TimeKeyword.INDEFINITE; };

time_component returns [Quantity<Time> result]
    : e=expression u=time_unit  { $result = new QuantityComponent($e.result, $u.result); };

time_unit returns [Time result]
    : ACTION        { $result = Time.ACTION; }
    | BONUS_ACTION  { $result = Time.BONUS_ACTION; }
    | REACTION      { $result = Time.REACTION; }
    | SECOND        { $result = Time.SECOND; }
    | MINUTE        { $result = Time.MINUTE; }
    | HOUR          { $result = Time.HOUR; }
    | DAY           { $result = Time.DAY; }
    | SHORT_REST    { $result = Time.SHORT_REST; }
    | LONG_REST     { $result = Time.LONG_REST; };

distance_quantity returns [Quantity<Distance> result]
    : distance_component                        { $result = $distance_component.result; }
    | a=distance_component bop=( '+' | '-' ) b=distance_component
        { $result = new QuantityBinary<Distance>(QuantityBinaryOp.fromString($bop.getText()), $a.result, $b.result); }
    | TOUCH { $result = DistanceKeyword.TOUCH; };

distance_component returns [Quantity<Distance> result]
    : e=expression u=distance_unit  { $result = new QuantityComponent($e.result, $u.result); };

distance_unit returns [Distance result]
    : FOOT      { $result = Distance.FOOT; }
    | MILE      { $result = Distance.MILE; }
    | SPACE     { $result = Distance.SPACE; };

damage_quantity returns [Quantity<Damage> result]
    : damage_component                        { $result = $damage_component.result; }
    | a=damage_component bop=( '+' | '-' ) b=damage_component
        { $result = new QuantityBinary<Damage>(QuantityBinaryOp.fromString($bop.getText()), $a.result, $b.result); };

damage_component returns [Quantity<Damage> result]
    : e=expression u=damage_unit    { $result = new QuantityComponent($e.result, $u.result); };

damage_unit returns [Damage result]
    : ACID          { $result = Damage.ACID; }
    | BLUDGEONING   { $result = Damage.BLUDGEONING; }
    | COLD          { $result = Damage.COLD; }
    | FIRE          { $result = Damage.FIRE; }
    | FORCE         { $result = Damage.FORCE; }
    | LIGHTNING     { $result = Damage.LIGHTNING; }
    | NECROTIC      { $result = Damage.NECROTIC; }
    | PIERCING      { $result = Damage.PIERCING; }
    | POISON        { $result = Damage.POISON; }
    | PSYCHIC       { $result = Damage.PSYCHIC; }
    | RADIANT       { $result = Damage.RADIANT; }
    | SLASHING      { $result = Damage.SLASHING; }
    | THUNDER       { $result = Damage.THUNDER; };

expression returns [Expression result]
    : l=expression bop=( '*' | '/' | '/+' | '/-' ) r=expression
        { $result = new ExpressionBinary(ExpressionBinaryOp.fromString($bop.getText()), $l.result, $r.result); }
    | l=expression bop=( '+' | '-' ) r=expression
        { $result = new ExpressionBinary(ExpressionBinaryOp.fromString($bop.getText()), $l.result, $r.result); }
    | '(' e=expression ')'  { $result = $e.result; }
    | n=NUMBER { $result = new NumberLiteral($n.getText()); }
    | d=DICE
        {   String s = $d.getText();
            String[] parts = s.toLowerCase().split("d");
            $result = new Dice(parts[0], parts[1]); }
    | i=IDENTIFIER { $result = new Identifier($i.getText()); };

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
SECOND : S E C (O N D)? S?;
MINUTE : M I N (U T E)? S?;
HOUR : H O U R S? | H R S;
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

NUMBER : DIGIT+;
IDENTIFIER : LETTER (LETTER | DIGIT)*;
DICE : DIGIT+ D DIGIT+;


WS : [ \t\u000c]+ -> skip;
