package model.gameObjects.items

open class Item(
        val name: String,
        val weight: Double,
        val cost: Double,
        val rarity: Rarity,
        val intrinsic: Boolean,
        val magical: Boolean,
        val description: String = ""
)