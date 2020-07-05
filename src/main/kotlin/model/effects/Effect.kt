package model.effects

import model.effects.results.*

class Effect {
    val triggers: Array<Trigger> = arrayOf()
    val conditions: Array<Condition> = arrayOf()
    val `saving throws`: Array<SavingThrow> = arrayOf()
    val results: Array<Result> = arrayOf()
}