package model.gameObjects.items

import com.fasterxml.jackson.annotation.JsonSubTypes
import com.fasterxml.jackson.annotation.JsonTypeInfo
import com.fasterxml.jackson.annotation.JsonTypeName

@JsonTypeName("misc")
@JsonTypeInfo(
        use = JsonTypeInfo.Id.NAME,
        include = JsonTypeInfo.As.PROPERTY,
        property = "type")
@JsonSubTypes(
        JsonSubTypes.Type(value = Weapon::class, name = "weapon"),
        JsonSubTypes.Type(value = Armor::class, name = "armor")
)
open class Item(
        val name: String,
        val rarity: Rarity,
        val weight: Double,
        val price: Double,
        val magical: Boolean,
        val description: String,
        val stackable: Boolean
)