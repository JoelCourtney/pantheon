package model.quantities.time

import com.fasterxml.jackson.databind.annotation.JsonDeserialize
import io.deserializers.TimeDeserializer
import model.quantities.Quantity

@JsonDeserialize(using = TimeDeserializer::class)
interface Time : Quantity






