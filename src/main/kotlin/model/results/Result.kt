package model.results

import com.fasterxml.jackson.databind.annotation.JsonDeserialize
import io.deserializers.ResultDeserializer
import model.Character

interface Result {
    fun apply(c: Character)
    fun purge(): Boolean
}
