package model.effects

import model.effects.results.*

class SavingThrow(
    val type: SavingThrowType,
    val fail: Array<Result>
) {
    val succeed: Array<Result> = arrayOf()
    val `half damage on succeed`: Boolean? = null
}

enum class SavingThrowType {
    STRENGTH,
    DEXTERITY,
    CONSTITUTION,
    INTELLIGENCE,
    WISDOM,
    CHARISMA,
    DEATH;
}