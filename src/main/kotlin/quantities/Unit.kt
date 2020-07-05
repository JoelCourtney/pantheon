package quantities

interface Unit {
    val symbol: String
}

enum class Distance(override val symbol: String) : Unit {
    FOOT("ft"),
    MILE("mi"),
    SPACE("spc");
}

enum class Time(override val symbol: String) : Unit {
    ACTION("action"),
    BONUS_ACTION("bonus action"),
    REACTION("reaction"),
    SECOND("sec"),
    MINUTE("min"),
    HOUR("hr"),
    DAY("day"),
    LONG_REST("long rest"),
    SHORT_REST("short rest");
}

enum class Damage(override var symbol: String) : Unit {
    ACID("acid"),
    BLUDGEONING("bludgeoning"),
    COLD("cold"),
    FIRE("fire"),
    FORCE("force"),
    LIGHTNING("lightning"),
    NECROTIC("necrotic"),
    PIERCING("piercing"),
    POISON("poison"),
    PSYCHIC("psychic"),
    RADIANT("radiant"),
    SLASHING("slashing"),
    THUNDER("thunder");
}

enum class Unitless(override var symbol: String) : Unit {
    UNITLESS("")
}