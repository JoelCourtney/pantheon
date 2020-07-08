package io

import java.io.File
import java.nio.file.Files
import java.nio.file.Path
import java.nio.file.Paths
import java.util.stream.Stream
import kotlin.streams.toList

object FileSystemWrapper {
    fun getDirectories(dir: String): List<Path> {
        val path = Paths.get(dir)
        return Files.list(path).filter { Files.isDirectory(it) }.toList()
    }
    fun getFiles(dir: String): List<Path> {
        val path = Paths.get(dir)
        return Files.list(path).filter { Files.isRegularFile(it) }.toList()
    }
    fun getFilesAndDirectories(dir: String): List<Path> {
        val path = Paths.get(dir)
        return Files.list(path).toList()
    }
    fun getFilesRecursive(dir: String): List<Path> {
        return File(dir).walkTopDown().filter { it.isFile }.toList().map { it.toPath() }
    }
}