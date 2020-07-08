package model.results.effects

import model.Character
import model.results.Result

abstract class Effect : Result {
    private var applied: Boolean = false
    final override fun apply(c: Character) {
        applyEffect(c)
        applied = true
    }
    abstract fun applyEffect(c: Character)
    override fun purge(): Boolean = applied
}