package io

import java.io.File
import java.nio.file.Files
import java.nio.file.Path
import java.nio.file.Paths
import kotlin.streams.toList

/**
 * Miscellaneous file IO for testing.
 * 
 * Provides convenience methods for getting lists of file, directories, or both. Singleton object; do not instantiate.
 */
object FileSystemWrapper {
    /**
     * Get a list of directories contained in the given path.
     * 
     * @param [dir] Path to search in. Must be a directory; not checked.
     * @return A [List] of Paths, where each is a directory in [dir]. 
     */
    fun getDirectories(dir: String): List<Path> {
        val path = Paths.get(dir)
        return Files.list(path).filter { Files.isDirectory(it) }.toList()
    }
    
    /**
     * Get a list of files contained in the given path.
     *
     * @param [dir] Path to search in. Must be a directory; not checked.
     * @return A [List] of Paths, where each is a regular file in [dir].
     */
    fun getFiles(dir: String): List<Path> {
        val path = Paths.get(dir)
        return Files.list(path).filter { Files.isRegularFile(it) }.toList()
    }

    /**
     * Get a list of all nodes (files and directories) contained in the given path.
     *
     * @param [dir] Path to search in. Must be a directory; not checked.
     * @return A [List] of Paths, where each is in [dir].
     */
    fun getAll(dir: String): List<Path> {
        val path = Paths.get(dir)
        return Files.list(path).toList()
    }

    /**
     * Recursively get a list of files contained in the given path.
     * 
     * Finds all files that are contained in [dir] or children of [dir], not just immediate children.
     *
     * @param [dir] Path to search in. Must be a directory; not checked.
     * @return A [List] of Paths, where each is a file in [dir]'s hierarchy.
     */
    fun getFilesRecursive(dir: String): List<Path> {
        return File(dir).walkTopDown().filter { it.isFile }.toList().map { it.toPath() }
    }

    /**
     * Gets the list of all content of the given type.
     * 
     * @param [type] can be one of:
     *     - Races
     *     - Spells
     *     - Languages
     *     - Classes
     *     - Feats
     * @return A [List] of Paths, each to a file of that content type.
     */
    fun getAllContentOfType(type: String): List<Path> {
        val contentSources = getDirectories("content-packs")
        val packs: MutableList<Path> = mutableListOf()
        for (source in contentSources) {
            packs.addAll(getDirectories(source.toString()))
        }
        val files: MutableList<Path> = mutableListOf()
        for (pack in packs) {
            val path = pack.resolve(type)
            files.addAll(getFilesRecursive(path.toString()))
        }
        return files
    }
}
