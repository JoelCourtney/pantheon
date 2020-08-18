package model.gameObjects.items

open class Item(
        val identity: String,
        val weight: Double,
        val price: Double,
        val rarity: Rarity
)