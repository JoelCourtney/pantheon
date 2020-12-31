package model.gameObjects

enum class ProficiencyMultiplier {
    NONE,
    SINGLE,
    DOUBLE;

    companion object {
        fun fromDoubleBoolean(b: Boolean): ProficiencyMultiplier {
            return if (b) {
                DOUBLE
            } else {
                SINGLE
            }
        }
    }
}