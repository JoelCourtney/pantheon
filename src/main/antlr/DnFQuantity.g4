grammar DnFQuantity;

import DnFLexer;

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
