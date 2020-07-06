import com.fasterxml.jackson.core.JsonParser
import com.fasterxml.jackson.databind.ObjectMapper
import com.fasterxml.jackson.dataformat.yaml.YAMLFactory
import com.fasterxml.jackson.module.kotlin.KotlinModule
import model.SavingThrowType
import model.SavingThrowTypeDeserializer
import model.conditions.Condition
import model.conditions.ConditionDeserializer
import model.conditions.ConditionListType
import model.conditions.ConditionListTypeDeserializer
import model.results.ResultDeserializer
import model.results.Result
import model.spells.SpellSchool
import model.spells.SpellSchoolDeserializer
import model.quantities.*
import model.timers.Timer
import model.timers.TimerDeserializer
import model.triggers.Trigger
import model.triggers.TriggerDeserializer
import model.triggers.TriggerListType
import model.triggers.TriggerListTypeDeserializer
import java.nio.file.FileSystems
import java.nio.file.Files

class IOWrapper {
    companion object {
        val mapper = ObjectMapper(YAMLFactory())

        init {
            val module = KotlinModule()
            module.addDeserializer(
                SpellSchool::class.java,
                SpellSchoolDeserializer()
            )
            module.addDeserializer(
                Time::class.java,
                TimeDeserializer()
            )
            module.addDeserializer(
                Damage::class.java,
                DamageDeserializer()
            )
            module.addDeserializer(
                Distance::class.java,
                DistanceDeserializer()
            )
            module.addDeserializer(
                Result::class.java,
                ResultDeserializer()
            )
            module.addDeserializer(
                Condition::class.java,
                ConditionDeserializer()
            )
            module.addDeserializer(
                SavingThrowType::class.java,
                SavingThrowTypeDeserializer()
            )
            module.addDeserializer(
                ConditionListType::class.java,
                ConditionListTypeDeserializer()
            )
            module.addDeserializer(
                Expression::class.java,
                ExpressionDeserializer()
            )
            module.addDeserializer(
                Trigger::class.java,
                TriggerDeserializer()
            )
            module.addDeserializer(
                DamageUnit::class.java,
                DamageUnitDeserializer()
            )
            module.addDeserializer(
                TriggerListType::class.java,
                TriggerListTypeDeserializer()
            )
            module.addDeserializer(
                Timer::class.java,
                TimerDeserializer()
            )
            mapper.registerModule(module)
        }

        inline fun<reified T: Any> read(dir: String, file: String): T {
            val path = FileSystems.getDefault().getPath(dir, file)
            return Files.newBufferedReader(path).use {
                mapper.readValue(it, T::class.java)
            }
        }

        fun enumString(p: JsonParser?): String {
            return p!!.readValueAs(String::class.java).toUpperCase()
        }
    }
}