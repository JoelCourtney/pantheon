package model.quantities.damage

import com.fasterxml.jackson.databind.annotation.JsonDeserialize
import io.deserializers.DamageUnitDeserializer

@JsonDeserialize(using = DamageUnitDeserializer::class)
interface DamageUnit {
    fun getDamageUnitLiteral(): DamageUnitLiteral
}