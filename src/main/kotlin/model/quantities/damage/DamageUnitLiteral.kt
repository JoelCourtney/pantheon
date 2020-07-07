package model.quantities.damage

enum class DamageUnitLiteral(var symbol: String) : DamageUnit {
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
    THUNDER("thunder"),
    MELEE("melee"),
    RANGED("ranged");

    override fun getDamageUnitLiteral(): DamageUnitLiteral {
        return this
    }

    override fun toString(): String {
        return symbol
    }
}