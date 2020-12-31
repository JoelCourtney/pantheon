package model.gameObjects

import model.access.Accessible
import model.gameObjects.moves.Move
import model.gameObjects.items.Armor
import model.gameObjects.items.Item
import model.gameObjects.prototypes.Background
import model.gameObjects.prototypes.Class
import model.gameObjects.prototypes.Instance
import model.gameObjects.prototypes.Race
import model.quantities.Quantity
import model.quantities.QuantityType

data class Character(
    val name: String,
    val race: Instance<Race>,
    val classes: List<Instance<Class>>,
    val background: Instance<Background>,
    val strBase: Int,
    val dexBase: Int,
    val conBase: Int,
    val intBase: Int,
    val wisBase: Int,
    val chaBase: Int,
    val HP: Int,
    val tempHP: Int,
    val inventory: MutableList<Item>,
    val equipped: MutableList<Item>,
    val inspired: Boolean
): Accessible {
    constructor(base: BaseCharacter): this(
            base.name,
            base.race,
            base.classes,
            base.background,
            base.strBase,
            base.dexBase,
            base.conBase,
            base.intBase,
            base.wisBase,
            base.chaBase,
            base.HP,
            base.tempHP,
            base.inventory,
            base.equipped,
            base.inspired
    )
    // SIDE BAR

    var maxHealth: Int = 0

    var strMod: Int = 0
    var dexMod: Int = 0
    var conMod: Int = 0
    var intMod: Int = 0
    var wisMod: Int = 0
    var chaMod: Int = 0

    var initiative: Int = 0

    val proficiencyBonus: Int = 0

    val walkingSpeed: Quantity<QuantityType.Distance>? = null
    val flyingSpeed: Quantity<QuantityType.Distance>? = null
    val climbingSpeed: Quantity<QuantityType.Distance>? = null
    val burrowingSpeed: Quantity<QuantityType.Distance>? = null
    val swimmingSpeed: Quantity<QuantityType.Distance>? = null

    val acrobaticsBonus: Int? = null
    val acrobaticsProficiencyMultiplier: ProficiencyMultiplier? = null

    val animalHandlingBonus: Int? = null
    val animalHandlingProficiencyMultiplier: ProficiencyMultiplier? = null

    val arcanaBonus: Int? = null
    val arcanaProficiencyMultiplier: ProficiencyMultiplier? = null

    val athleticsBonus: Int? = null
    val athleticsProficiencyMultiplier: ProficiencyMultiplier? = null

    val deceptionBonus: Int? = null
    val deceptionProficiencyMultiplier: ProficiencyMultiplier? = null

    val historyBonus: Int? = null
    val historyProficiencyMultiplier: ProficiencyMultiplier? = null

    val insightBonus: Int? = null
    val insightProficiencyMultiplier: ProficiencyMultiplier? = null

    val intimidationBonus: Int? = null
    val intimidationProficiencyMultiplier: ProficiencyMultiplier? = null

    val investigationBonus: Int? = null
    val investigationProficiencyMultiplier: ProficiencyMultiplier? = null

    val medicineBonus: Int? = null
    val medicineProficiencyMultiplier: ProficiencyMultiplier? = null

    val natureBonus: Int? = null
    val natureProficiencyMultiplier: ProficiencyMultiplier? = null

    val perceptionBonus: Int? = null
    val perceptionProficiencyMultiplier: ProficiencyMultiplier? = null

    val performanceBonus: Int? = null
    val performanceProficiencyMultiplier: ProficiencyMultiplier? = null

    val persuasionBonus: Int? = null
    val persuasionProficiencyMultiplier: ProficiencyMultiplier? = null

    val religionBonus: Int? = null
    val religionProficiencyMultiplier: ProficiencyMultiplier? = null

    val sleightOfHandBonus: Int? = null
    val sleightOfHandProficiencyMultiplier: ProficiencyMultiplier? = null

    val stealthBonus: Int? = null
    val stealthProficiencyMultiplier: ProficiencyMultiplier? = null

    val survivalBonus: Int? = null
    val survivalProficiencyMultiplier: ProficiencyMultiplier? = null

    // COMBAT

    val actions: List<Move> = listOf()
    val bonusActions: List<Move> = listOf()
    val reactions: List<Move> = listOf()

    // MISC

    val primaryHandItem: Item? = null
    val offHandItem: Item? = null
    val armor: Armor? = null

    fun commit() {

    }

    override fun get(id: String): Any {
        TODO("Not yet implemented")
    }

    override fun set(id: String, value: String) {
        TODO("Not yet implemented")
    }
}
