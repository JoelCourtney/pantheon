package io.deserializers

import com.fasterxml.jackson.core.JsonParser
import com.fasterxml.jackson.core.TreeNode
import com.fasterxml.jackson.databind.DeserializationContext
import com.fasterxml.jackson.databind.deser.std.StdDeserializer
import io.JacksonWrapper
import model.races.Race
import model.races.VariantRace
import model.results.Result

class RaceDeserializer: StdDeserializer<Race>(Race::class.java) {
    override fun deserialize(p: JsonParser?, ctxt: DeserializationContext?): Race {
        val tn = p!!.readValueAsTree<TreeNode>()
        
        var parser = tn.get("name").traverse(JacksonWrapper.mapper)
        parser.nextToken()
        val name = ctxt!!.readValue(
            parser,
            String::class.java
        )
        
        parser = tn.get("description").traverse(JacksonWrapper.mapper)
        parser.nextToken()
        val description = ctxt.readValue(
            parser,
            String::class.java
        )
        
        parser = tn.get("traits").traverse(JacksonWrapper.mapper)
        parser.nextToken()
        val traits = ctxt.readValue(
            parser,
            Array<Result>::class.java
        )
        
        val keys = tn.fieldNames().asSequence().toList()
        
        var chooseable: Boolean = true
        if (keys.contains("chooseable")) {
            parser = tn.get("name").traverse(JacksonWrapper.mapper)
            parser.nextToken()
            chooseable = ctxt.readValue(
                parser,
                Boolean::class.java
            )
        }
        
        return if (keys.contains("variant of")) {
            parser = tn.get("variant of").traverse(JacksonWrapper.mapper)
            parser.nextToken()
            
            val parent = FileNameDeserializer.RaceFileNameDeserializer.deserialize(parser, ctxt) as Race
            VariantRace(
                name,
                parent,
                traits,
                description,
                chooseable
            )
        } else {
            Race(name, traits, description, chooseable)
        }
    }
}
