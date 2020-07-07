grammar DnF;

import DnLexer;

@parser::header {
    import model.quantities.time.*;
    import model.quantities.distance.*;
    import model.quantities.damage.*;
    import model.quantities.amounts.*;
    import model.quantities.*;
}

time returns [Time result]
    : time_amount EOF { $result = $time_amount.result; };

time_amount returns [Time result]
    : time_component        { $result = $time_component.result; }
    | a=time_amount SEMICOLON b=time_amount
        { $result = new QuantityBinary($a.result, $b.result); }
    | INSTANTANEOUS         { $result = TimeKeyword.INSTANTANEOUS; }
    | INDEFINITE            { $result = TimeKeyword.INDEFINITE; }
    | NOW                   { $result = TimeKeyword.NOW; };

time_component returns [Time result]
    : e=amount u=time_unit  { $result = new TimeComponent($e.result, $u.result); }
    | i=identifier              { $result = $i.result; };

time_unit returns [TimeUnit result]
    : ACTION        { $result = TimeUnitLiteral.ACTION; }
    | BONUS_ACTION  { $result = TimeUnitLiteral.BONUS_ACTION; }
    | REACTION      { $result = TimeUnitLiteral.REACTION; }
    | ROUND         { $result = TimeUnitLiteral.ROUND; }
    | SECOND        { $result = TimeUnitLiteral.SECOND; }
    | MINUTE        { $result = TimeUnitLiteral.MINUTE; }
    | HOUR          { $result = TimeUnitLiteral.HOUR; }
    | DAY           { $result = TimeUnitLiteral.DAY; }
    | SHORT_REST    { $result = TimeUnitLiteral.SHORT_REST; }
    | LONG_REST     { $result = TimeUnitLiteral.LONG_REST; }
    | i=identifier  { $result = $i.result; };

distance returns [Distance result]
    : distance_amount EOF { $result = $distance_amount.result; };

distance_amount returns [Distance result]
    : distance_component                        { $result = $distance_component.result; }
    | a=distance_amount SEMICOLON b=distance_amount
        { $result = new QuantityBinary($a.result, $b.result); }
    | TOUCH { $result = DistanceKeyword.TOUCH; }
    | SELF { $result = DistanceKeyword.SELF; };

distance_component returns [Distance result]
    : e=amount u=distance_unit  { $result = new DistanceComponent($e.result, $u.result); }
    | i=identifier              { $result = $i.result; };

distance_unit returns [DistanceUnit result]
    : FOOT          { $result = DistanceUnitLiteral.FOOT; }
    | MILE          { $result = DistanceUnitLiteral.MILE; }
    | SPACE         { $result = DistanceUnitLiteral.SPACE; }
    | i=identifier  { $result = $i.result; };

damage returns [Damage result]
    : damage_amount EOF { $result = $damage_amount.result; };

damage_amount returns [Damage result]
    : damage_component                        { $result = $damage_component.result; }
    | a=damage_amount SEMICOLON b=damage_amount
        { $result = new QuantityBinary($a.result, $b.result); }
    | NONE { $result = DamageKeyword.NONE; };

damage_component returns [Damage result]
    : e=amount u=damage_unit    { $result = new DamageComponent($e.result, $u.result); }
    | i=identifier              { $result = $i.result; };

damage_unit returns [DamageUnit result]
    : ACID          { $result = DamageUnitLiteral.ACID; }
    | BLUDGEONING   { $result = DamageUnitLiteral.BLUDGEONING; }
    | COLD          { $result = DamageUnitLiteral.COLD; }
    | FIRE          { $result = DamageUnitLiteral.FIRE; }
    | FORCE         { $result = DamageUnitLiteral.FORCE; }
    | LIGHTNING     { $result = DamageUnitLiteral.LIGHTNING; }
    | NECROTIC      { $result = DamageUnitLiteral.NECROTIC; }
    | PIERCING      { $result = DamageUnitLiteral.PIERCING; }
    | POISON        { $result = DamageUnitLiteral.POISON; }
    | PSYCHIC       { $result = DamageUnitLiteral.PSYCHIC; }
    | RADIANT       { $result = DamageUnitLiteral.RADIANT; }
    | SLASHING      { $result = DamageUnitLiteral.SLASHING; }
    | THUNDER       { $result = DamageUnitLiteral.THUNDER; }
    | MELEE         { $result = DamageUnitLiteral.MELEE; }
    | RANGED        { $result = DamageUnitLiteral.RANGED; }
    | i=identifier  { $result = $i.result; };

amount returns [Amount result]
    : l=amount bop=( TIMES | DIVIDE_DOWN | DIVIDE_UP ) r=amount
        { $result = new AmountBinary(AmountBinaryOp.fromString($bop.getText()), $l.result, $r.result); }
    | l=amount bop=( PLUS | MINUS ) r=amount
        { $result = new AmountBinary(AmountBinaryOp.fromString($bop.getText()), $l.result, $r.result); }
    | OPEN_PAREN e=amount CLOSE_PAREN  { $result = $e.result; }
    | n=NUMBER { $result = new NumberLiteral($n.getText()); }
    | d=DICE
        {   String s = $d.getText();
            String[] parts = s.toLowerCase().split("d");
            $result = new Dice(parts[0], parts[1]); }
    | i=identifier { $result = $i.result; };

identifier returns [Identifier result]
    : o=NAME DOLLAR i=NAME { $result = new Identifier($o.getText(), $i.getText()); }
    | i=NAME { $result = new Identifier($i.getText(), null); };
