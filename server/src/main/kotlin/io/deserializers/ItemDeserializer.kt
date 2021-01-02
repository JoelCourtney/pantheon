package io.deserializers

import com.fasterxml.jackson.core.JsonParser
import com.fasterxml.jackson.core.TreeNode
import com.fasterxml.jackson.databind.DeserializationContext
import com.fasterxml.jackson.databind.deser.std.StdDeserializer
import io.JacksonWrapper
import model.gameObjects.items.Armor
import model.gameObjects.items.Item
import model.gameObjects.items.Weapon

class ItemDeserializer: StdDeserializer<Item>(Item::class.java) {
    override fun deserialize(p: JsonParser?, ctxt: DeserializationContext?): Item {
        val tn = p!!.readValueAsTree<TreeNode>()
        val type = tn["type"].toString().drop(1).dropLast(1)
        val targetClass = when (type) {
            "armor" -> Armor::class.java
            "weapon" -> Weapon::class.java
            else -> Item::class.java
        }
        val parser = tn.traverse()
        parser.codec = JacksonWrapper.mapper
        parser.nextToken()
        return ctxt!!.readValue(parser, targetClass)
    }
}