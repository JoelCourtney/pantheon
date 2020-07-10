package model.quantities

import model.identity.Evaluated
import model.quantities.QuantityType.Damage

enum class DamageUnit(var identity: String): QuantityUnit<Damage> {
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

    override fun toString(): String {
        return identity
    }
}
