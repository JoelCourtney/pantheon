package model.gameObjects

import model.gameObjects.items.Item
import model.gameObjects.prototypes.Background
import model.gameObjects.prototypes.Class
import model.gameObjects.prototypes.Instance
import model.gameObjects.prototypes.Race

data class BaseCharacter(
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
)