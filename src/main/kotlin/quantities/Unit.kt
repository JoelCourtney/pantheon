package quantities

interface Unit

enum class Distance : Unit {
    FEET,
    MILE,
    SPACE
}

enum class Time : Unit {
    ACTION,
    BONUS_ACTION,
    REACTION,
    SECOND,
    MINUTE,
    HOUR,
    DAY,
    LONG_REST,
    SHORT_REST
}

enum class Damage : Unit {
    ACID,
    BLUDGEONING,
    COLD,
    FIRE,
    FORCE,
    LIGHTNING,
    NECROTIC,
    PIERCING,
    POISON,
    PSYCHIC,
    RADIANT,
    SLASHING,
    THUNDER
}

enum class Unitless : Unit {
    UNITLESS
}