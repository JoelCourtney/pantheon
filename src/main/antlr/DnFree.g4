grammar DnFree;

@parser::header {
    import model.quantities.*;
}

time_quantity returns [Time result]
    : time_component        { $result = $time_component.result; }
    | a=time_component bop=( PLUS | MINUS ) b=time_component
        { $result = new QuantityBinary(QuantityBinaryOp.fromString($bop.getText()), $a.result, $b.result); }
    | INSTANTANEOUS         { $result = TimeKeyword.INSTANTANEOUS; }
    | INDEFINITE            { $result = TimeKeyword.INDEFINITE; };

time_component returns [Time result]
    : e=expression u=time_unit  { $result = new TimeComponent($e.result, $u.result); };

time_unit returns [TimeUnit result]
    : ACTION        { $result = TimeUnit.ACTION; }
    | BONUS_ACTION  { $result = TimeUnit.BONUS_ACTION; }
    | REACTION      { $result = TimeUnit.REACTION; }
    | SECOND        { $result = TimeUnit.SECOND; }
    | MINUTE        { $result = TimeUnit.MINUTE; }
    | HOUR          { $result = TimeUnit.HOUR; }
    | DAY           { $result = TimeUnit.DAY; }
    | SHORT_REST    { $result = TimeUnit.SHORT_REST; }
    | LONG_REST     { $result = TimeUnit.LONG_REST; };

distance_quantity returns [Distance result]
    : distance_component                        { $result = $distance_component.result; }
    | a=distance_component bop=( PLUS | MINUS ) b=distance_component
        { $result = new QuantityBinary(QuantityBinaryOp.fromString($bop.getText()), $a.result, $b.result); }
    | TOUCH { $result = DistanceKeyword.TOUCH; }
    | SELF { $result = DistanceKeyword.SELF; };

distance_component returns [Distance result]
    : e=expression u=distance_unit  { $result = new DistanceComponent($e.result, $u.result); };

distance_unit returns [DistanceUnit result]
    : FOOT      { $result = DistanceUnit.FOOT; }
    | MILE      { $result = DistanceUnit.MILE; }
    | SPACE     { $result = DistanceUnit.SPACE; };

damage_quantity returns [Damage result]
    : damage_component                        { $result = $damage_component.result; }
    | a=damage_component bop=( PLUS | MINUS ) b=damage_component
        { $result = new QuantityBinary(QuantityBinaryOp.fromString($bop.getText()), $a.result, $b.result); };

damage_component returns [Damage result]
    : e=expression u=damage_unit    { $result = new DamageComponent($e.result, $u.result); };

damage_unit returns [DamageUnit result]
    : ACID          { $result = DamageUnit.ACID; }
    | BLUDGEONING   { $result = DamageUnit.BLUDGEONING; }
    | COLD          { $result = DamageUnit.COLD; }
    | FIRE          { $result = DamageUnit.FIRE; }
    | FORCE         { $result = DamageUnit.FORCE; }
    | LIGHTNING     { $result = DamageUnit.LIGHTNING; }
    | NECROTIC      { $result = DamageUnit.NECROTIC; }
    | PIERCING      { $result = DamageUnit.PIERCING; }
    | POISON        { $result = DamageUnit.POISON; }
    | PSYCHIC       { $result = DamageUnit.PSYCHIC; }
    | RADIANT       { $result = DamageUnit.RADIANT; }
    | SLASHING      { $result = DamageUnit.SLASHING; }
    | THUNDER       { $result = DamageUnit.THUNDER; };

expression returns [Expression result]
    : l=expression bop=( TIMES | DIVIDE_DOWN | DIVIDE_UP ) r=expression
        { $result = new ExpressionBinary(ExpressionBinaryOp.fromString($bop.getText()), $l.result, $r.result); }
    | l=expression bop=( PLUS | MINUS ) r=expression
        { $result = new ExpressionBinary(ExpressionBinaryOp.fromString($bop.getText()), $l.result, $r.result); }
    | OPEN_PAREN e=expression CLOSE_PAREN  { $result = $e.result; }
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
SELF : S E L F;

NUMBER : DIGIT+;
IDENTIFIER : LETTER (LETTER | DIGIT)*;
DICE : DIGIT+ D DIGIT+;

PLUS : '+';
MINUS : '-';
TIMES : '*';
DIVIDE_DOWN : '/' | '/-';
DIVIDE_UP : '/+';
OPEN_PAREN : '(';
CLOSE_PAREN : ')';

WS : [ \t\u000c]+ -> skip;
