package io

import io.FileSystemWrapper.getAllContentOfType
import model.gameObjects.Spell
import model.gameObjects.items.Item
import model.gameObjects.prototypes.Class
import model.gameObjects.prototypes.Feat
import model.gameObjects.prototypes.Race
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

    @ParameterizedTest(name = "parse class file {1}")
    @MethodSource
    fun parseClassFile(dir: String, file: String) {
        assertDoesNotThrow {
            JacksonWrapper.readFile<Class>(dir, file)
        }
    }

    @ParameterizedTest(name = "parse item file {1}")
    @MethodSource
    fun parseItemFile(dir: String, file: String) {
        assertDoesNotThrow {
            JacksonWrapper.readFile<Item>(dir, file)
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

        @JvmStatic
        fun parseClassFile(): List<Arguments> {
            return getAllContentOfType("Classes").map { Arguments.of(it.parent.toString(), it.fileName.toString()) }
        }

        @JvmStatic
        fun parseItemFile(): List<Arguments> {
            return getAllContentOfType("Items").map { Arguments.of(it.parent.toString(), it.fileName.toString()) }
        }
    }
}
