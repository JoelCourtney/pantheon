import model.access.Accessible
import model.access.Environment
import model.gameObjects.Character
import model.gameObjects.FinalCharacter
import model.gameObjects.Instance
import model.modifications.results.Result

class Engine(val character: Character) {
    /*
    Sources of Effects:
    - Race
    - Class
        - feats contained in class
    - Background
    - Results that generate effects

    Algorithm:
    1. we have a list of events that caused recalculation
    2. create a new buffer for collecting events
    3. copy equipped items
    4. apply effects from race
    5. apply effects from class (and feats)
    6. apply effects from background
    7. apply effects from equipped items
    8. apply results and apply generated effects
    9. purge results
    10. If the new event buffer is nonEmpty, go back to 1.
    11. copy misc (inspiration, name, etc)
    12. copy inventory
    13. Calculate all emergent properties (max health, modifiers, etc)

    ALGORITHM REQUIREMENTS
    1. If you run it twice and the second run has no events, you MUST get EXACTLY the same result.
     */
    fun calculate(): FinalCharacter {
        Environment.character = character as Accessible
        throw NotImplementedError()
    }
}