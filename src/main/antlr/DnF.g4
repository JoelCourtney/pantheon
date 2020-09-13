parser grammar DnF;

options { tokenVocab = DnLexer; }

@parser::header {
    import model.access.*;
    import model.quantities.*;
    import model.quantities.QuantityType.*;
    import model.quantities.amounts.*;
}

damage returns [Expression<Quantity<Damage>> result]
    : { ArrayList<Expression<QuantityComponent<Damage>>> components = new ArrayList<Expression<QuantityComponent<Damage>>>(); }
    c=damage_component { components.add($c.result); }
    (
        SEMICOLON c=damage_component { components.add($c.result); }
    )* EOF { $result = new Quantity<Damage>(components); }
    | i=identifier { $result = (Identifier<Quantity<Damage>>) $i.result; };

damage_component returns [Expression<QuantityComponent<Damage>> result]
    : e=amount u=damage_unit  { $result = new QuantityComponent<Damage>($e.result, $u.result); }
    | i=identifier          { $result = (Identifier<QuantityComponent<Damage>>) $i.result; }
    | NONE                  { $result = DamageKeyword.NONE; };

damage_unit returns [Expression<QuantityUnit<Damage>> result]
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
    | THUNDER       { $result = DamageUnit.THUNDER; }
    | MELEE         { $result = DamageUnit.MELEE; }
    | RANGED        { $result = DamageUnit.RANGED; }
    | i=identifier  { $result = (Identifier<QuantityUnit<Damage>>) $i.result; };

distance returns [Expression<Quantity<Distance>> result]
    : { ArrayList<Expression<QuantityComponent<Distance>>> components = new ArrayList<Expression<QuantityComponent<Distance>>>(); }
    c=distance_component { components.add($c.result); }
    (
        SEMICOLON c=distance_component { components.add($c.result); }
    )* EOF { $result = new Quantity<Distance>(components); }
    | i=identifier { $result = (Identifier<Quantity<Distance>>) $i.result; };

distance_component returns [Expression<QuantityComponent<Distance>> result]
    : e=amount u=distance_unit  { $result = new QuantityComponent<Distance>($e.result, $u.result); }
    | i=identifier          { $result = (Identifier<QuantityComponent<Distance>>) $i.result; }
    | TOUCH                 { $result = DistanceKeyword.TOUCH; }
    | SELF                  { $result = DistanceKeyword.SELF; };

distance_unit returns [Expression<QuantityUnit<Distance>> result]
    : FOOT          { $result = DistanceUnit.FOOT; }
    | MILE          { $result = DistanceUnit.MILE; }
    | SPACE         { $result = DistanceUnit.SPACE; }
    | i=identifier  { $result = (Identifier<QuantityUnit<Distance>>) $i.result; };

time returns [Expression<Quantity<Time>> result]
    : { ArrayList<Expression<QuantityComponent<Time>>> components = new ArrayList<Expression<QuantityComponent<Time>>>(); }
    c=time_component { components.add($c.result); }
    (
        SEMICOLON c=time_component { components.add($c.result); }
    )* EOF { $result = new Quantity<Time>(components); }
    | i=identifier { $result = (Identifier<Quantity<Time>>) $i.result; };

time_component returns [Expression<QuantityComponent<Time>> result]
    : e=amount u=time_unit  { $result = new QuantityComponent<Time>($e.result, $u.result); }
    | i=identifier          { $result = (Identifier<QuantityComponent<Time>>) $i.result; }
    | INSTANTANEOUS         { $result = TimeKeyword.INSTANTANEOUS; }
    | INDEFINITE            { $result = TimeKeyword.INDEFINITE; }
    | NOW                   { $result = TimeKeyword.NOW; };

time_unit returns [Expression<QuantityUnit<Time>> result]
    : ACTION        { $result = TimeUnit.ACTION; }
    | BONUS_ACTION  { $result = TimeUnit.BONUS_ACTION; }
    | REACTION      { $result = TimeUnit.REACTION; }
    | ROUND         { $result = TimeUnit.ROUND; }
    | SECOND        { $result = TimeUnit.SECOND; }
    | MINUTE        { $result = TimeUnit.MINUTE; }
    | HOUR          { $result = TimeUnit.HOUR; }
    | DAY           { $result = TimeUnit.DAY; }
    | SHORT_REST    { $result = TimeUnit.SHORT_REST; }
    | LONG_REST     { $result = TimeUnit.LONG_REST; }
    | i=identifier  { $result = (Identifier<QuantityUnit<Time>>) $i.result; };

amount returns [Expression<Amount> result]
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
    | i=identifier { $result = (Identifier<Amount>) $i.result; };

identifier returns [Identifier<? extends Object> result]
    : n1=NAME OPEN_BRACKET n2=NAME CLOSE_BRACKET { $result = new Identifier<Object>($n1.getText(), $n2.getText()); };
