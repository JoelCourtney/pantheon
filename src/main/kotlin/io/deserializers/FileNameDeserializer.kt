package io.deserializers

import com.fasterxml.jackson.core.JsonParser
import com.fasterxml.jackson.databind.DeserializationContext
import com.fasterxml.jackson.databind.deser.std.StdDeserializer
import io.FileSystemWrapper
import io.JacksonWrapper
import model.gameObjects.Language
import model.gameObjects.prototypes.Race
import model.gameObjects.Spell

sealed class FileNameDeserializer<T>(val type: String, val cls: Class<T>): StdDeserializer<T>(cls) {
    fun getFile(p: JsonParser?): Pair<String,String> {
        val name = p!!.readValueAs(String::class.java)
        val paths = FileSystemWrapper.getAllContentOfType(type)
        val files = paths.map { it.fileName.toString().removeSuffix(".yaml") }
        val index: Int = files.indexOf(name)
        val dir = paths[index].parent.toString()
        return dir to name
        // TODO("make an error for file not found")
    }

    object RaceFileNameDeserializer: FileNameDeserializer<Race>("Races", Race::class.java) {
        override fun deserialize(p: JsonParser?, ctxt: DeserializationContext?): Race {
            val path = getFile(p)
            return JacksonWrapper.readFile(path.first, "${path.second}.yaml")
        }
    }

    object LanguageFileNameDeserializer: FileNameDeserializer<Language>("Languages", Language::class.java) {
        override fun deserialize(p: JsonParser?, ctxt: DeserializationContext?): Language {
            val path = getFile(p)
            return JacksonWrapper.readFile(path.first, "${path.second}.yaml")
        }
    }

    object SpellFileNameDeserializer: FileNameDeserializer<Spell>("Spells", Spell::class.java) {
        override fun deserialize(p: JsonParser?, ctxt: DeserializationContext?): Spell {
            val path = getFile(p)
            return JacksonWrapper.readFile(path.first, "${path.second}.yaml")
        }
    }
}
