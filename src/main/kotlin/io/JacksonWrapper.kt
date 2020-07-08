package io

import com.fasterxml.jackson.databind.ObjectMapper
import com.fasterxml.jackson.dataformat.yaml.YAMLFactory
import com.fasterxml.jackson.module.kotlin.KotlinModule
import io.deserializers.*
import model.AbilityScoreType
import model.logic.Event
import model.logic.Timer
import model.quantities.Identifier
import model.quantities.amounts.Amount
import model.quantities.damage.Damage
import model.quantities.damage.DamageUnit
import model.quantities.distance.Distance
import model.quantities.distance.DistanceUnit
import model.quantities.time.Time
import model.quantities.time.TimeUnit
import model.results.Result
import java.nio.file.FileSystems
import java.nio.file.Files
import java.io.IOException
import com.fasterxml.jackson.core.JsonParseException
import com.fasterxml.jackson.databind.JsonMappingException

/**
 * Convenience object for deserializing objects through com.fasterxml.jackson.
 * 
 * Singleton object; do not attempt to instantiate.
 * 
 * @property [mapper] ObjectMapper for parsing YAML files. Direct usage is allowed but not recommended.
 */
object JacksonWrapper {
    val mapper: ObjectMapper = ObjectMapper(YAMLFactory())
    private val module = KotlinModule()

    init {
        module.addDeserializer(
            Time::class.java,
            TimeDeserializer
        )
        module.addDeserializer(
            Damage::class.java,
            DamageDeserializer
        )
        module.addDeserializer(
            Distance::class.java,
            DistanceDeserializer
        )
        module.addDeserializer(
            Result::class.java,
            ResultDeserializer
        )
        module.addDeserializer(
            Amount::class.java,
            AmountDeserializer
        )
        module.addDeserializer(
            DamageUnit::class.java,
            DamageUnitDeserializer
        )
        module.addDeserializer(
            TimeUnit::class.java,
            TimeUnitDeserializer
        )
        module.addDeserializer(
            DistanceUnit::class.java,
            DistanceUnitDeserializer
        )
        module.addDeserializer(
            Event::class.java,
            EventDeserializer
        )
        module.addDeserializer(
            Identifier::class.java,
            IdentifierDeserializer
        )
        module.addDeserializer(
            Timer::class.java,
            TimerDeserializer
        )
        module.addDeserializer(
            AbilityScoreType::class.java,
            AbilityScoreTypeDeserializer
        )
        mapper.registerModule(module)
    }

    /**
     * Reads a file and deserializes it as the given type.
     * 
     * @param [T] Type to deserialize contents as.
     * @param [dir] Directory containing the file.
     * @param [file] Name of the file.
     * 
     * @return Instance of [T], deserialized from [file].
     * 
     * @throws [IOException]
     * @throws [JsonParseException]
     * @throws [JsonMappingException]
     */
    inline fun<reified T: Any> readFile(dir: String, file: String): T {
        val path = FileSystems.getDefault().getPath(dir, file)
        return Files.newBufferedReader(path).use {
            mapper.readValue(it, T::class.java)
        }
    }
    
    /**
     * Parses a string and deserializes it as the given type.
     *
     * @param [T] Type to deserialize contents as.
     * @param [str] String to deserialize.
     *
     * @return Instance of [T], deserialized from [str].
     *
     * @throws [IOException]
     * @throws [JsonParseException]
     * @throws [JsonMappingException]
     */
    inline fun<reified T: Any> readString(str: String): T {
        val reader = java.io.StringReader(str)
        return reader.use {
            mapper.readValue(it, T::class.java)
        }
    }
}
