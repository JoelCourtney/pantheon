package io

import io.FileSystemWrapper.getAllContentOfType
import model.gameObjects.Feat
import model.gameObjects.Race
import model.gameObjects.Spell
import org.junit.jupiter.api.assertDoesNotThrow
import org.junit.jupiter.params.ParameterizedTest
import org.junit.jupiter.params.provider.Arguments
import org.junit.jupiter.params.provider.MethodSource

class TestAllRead {
    @ParameterizedTest(name = "parse spell file {1}")
    @MethodSource
    fun parseSpellFile(dir: String, file: String) {
        assertDoesNotThrow {
            JacksonWrapper.readFile<Spell>(dir, file)
        }
    }

    @ParameterizedTest(name = "parse race file {1}")
    @MethodSource
    fun parseRaceFile(dir: String, file: String) {
        assertDoesNotThrow {
            JacksonWrapper.readFile<Race>(dir, file)
        }
    }
    
    @ParameterizedTest(name = "parse feat file {1}")
    @MethodSource
    fun parseFeatFile(dir: String, file: String) {
        assertDoesNotThrow {
            JacksonWrapper.readFile<Feat>(dir, file)
        }
    }

    companion object {
        @JvmStatic
        fun parseSpellFile(): List<Arguments> {
            return getAllContentOfType("Spells").map { Arguments.of(it.parent.toString(), it.fileName.toString()) }
        }

        @JvmStatic
        fun parseRaceFile(): List<Arguments> {
            return getAllContentOfType("Races").map { Arguments.of(it.parent.toString(), it.fileName.toString()) }
        }
        
        @JvmStatic
        fun parseFeatFile(): List<Arguments> {
            return getAllContentOfType("Feats").map { Arguments.of(it.parent.toString(), it.fileName.toString()) }
        }
    }
}
