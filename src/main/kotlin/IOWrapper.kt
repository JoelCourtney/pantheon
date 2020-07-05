import com.fasterxml.jackson.databind.ObjectMapper
import com.fasterxml.jackson.dataformat.yaml.YAMLFactory
import com.fasterxml.jackson.module.kotlin.KotlinModule
import java.nio.file.FileSystems
import java.nio.file.Files
import java.nio.file.Path
import kotlin.reflect.jvm.internal.impl.load.kotlin.JvmType

class IOWrapper {
    companion object {
        val mapper = ObjectMapper(YAMLFactory())

        init {
            mapper.registerModule(KotlinModule())
        }

        inline fun<reified T: Any> read(dir: String, file: String): T {
            val path = FileSystems.getDefault().getPath(dir, file)
            return Files.newBufferedReader(path).use {
                mapper.readValue(it, T::class.java)
            }
        }
    }
}