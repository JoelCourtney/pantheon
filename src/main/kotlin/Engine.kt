import model.access.Environment
import model.effects.Effect
import model.gameObjects.BaseCharacter
import model.gameObjects.Character

class Engine(private val base: BaseCharacter) {
    /**
     * Converts the Character into a FinalCharacter for sending to the client.
     *
     * Sources of Effects:
     * - Race
     * - Class
     * - feats contained in class
     * - Background
     * - Results that generate effects
     *
     * Algorithm:
     * 1. Calculate all emergent properties (max health, modifiers, etc)
     * 2. apply effects from class (and feats)
     * 3. apply effects from race
     * 4. apply effects from background
     * 5. apply effects from equipped items
     * 6. purge results
     * 7. copy misc (inspiration, name, etc)
     * 8. copy inventory
     */
    fun compute(): Character {
        val c = Character(base)
        Environment.character = c
        val effects = standardEffects.toMutableList()
        effects.addAll(c.race.getEffects())
        for (classInstance in c.classes) {
            effects.addAll(classInstance.getEffects())
        }
        effects.addAll(c.background.getEffects())
        while (effects.isNotEmpty()) {
            for (i in 0..effects.size) {
                val effect = effects[i]
                val dep = effect.dependencies
                var free = true
                if (dep!!.isNotEmpty()) {
                    for (j in 0..effects.size) {
                        for (effected in effects[j].effected!!) {
                            if (i != j && dep.contains(effected)) {
                                free = false
                                break
                            }
                        }
                        if (!free) break
                    }
                }
                if (free) {
                    effect.apply()
                    effects.removeAt(i)
                    break
                }
            }
        }
        return c
    }

    companion object {
        val standardEffects = listOf<Effect>()
    }
}