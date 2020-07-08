package model.quantities.distance

import com.fasterxml.jackson.databind.annotation.JsonDeserialize
import io.deserializers.DistanceUnitDeserializer

@JsonDeserialize(using = DistanceUnitDeserializer::class)
interface DistanceUnit {
    fun getDistanceUnitLiteral(): DistanceUnitLiteral
}