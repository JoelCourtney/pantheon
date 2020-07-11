package io.deserializers

import com.fasterxml.jackson.core.JsonParser
import com.fasterxml.jackson.databind.DeserializationContext
import com.fasterxml.jackson.databind.deser.std.StdDeserializer
import io.FileSystemWrapper
import io.JacksonWrapper
import model.gameObjects.Language
import model.gameObjects.Race
import model.spells.Spell

sealed class FileNameDeserializer<T>(val type: String) : StdDeserializer<Any>(Any::class.java) {
    override fun deserialize(p: JsonParser?, ctxt: DeserializationContext?): Any {
        val name = p!!.readValueAs(String::class.java)
        val paths = FileSystemWrapper.getAllContentOfType(type)
        val files = paths.map { it.fileName.toString().removeSuffix(".yaml") }
        val index: Int = files.indexOf(name)
        val dir = paths[index].parent.toString()
        return JacksonWrapper.readFile<Race>(dir, "$name.yaml")
    }
    
    object RaceFileNameDeserializer: FileNameDeserializer<Race>("Races")
    object LanguageFileNameDeserializer: FileNameDeserializer<Language>("Languages")
    object SpellFileNameDeserializer: FileNameDeserializer<Spell>("Spells")
}
