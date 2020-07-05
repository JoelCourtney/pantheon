package model.effects.results

import model.Character
import model.quantities.Damage
import model.quantities.DamageKeyword

interface Result {
    fun apply(c: Character)
}

