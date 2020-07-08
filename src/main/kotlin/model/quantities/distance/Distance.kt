package model.quantities.distance

import com.fasterxml.jackson.databind.annotation.JsonDeserialize
import io.deserializers.DistanceDeserializer
import model.quantities.Quantity

@JsonDeserialize(using = DistanceDeserializer::class)
interface Distance : Quantity

