package com.github.conf;

import com.github.utils.JsonUtil;
import io.netty.util.internal.StringUtil;
import lombok.Data;
import lombok.SneakyThrows;
import lombok.experimental.Accessors;

import java.io.File;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.time.LocalDateTime;

@Data
@Accessors(chain = true)
public class IndexConf {
    public static final String EXE = "./lib/fsevent.exe ";
    public static final String ORANGE_PATH = "./.orange";
    public static final String INDEX_PATH = ORANGE_PATH + "/index";
    public static final int PORT = Integer.parseInt(System.getProperty("port", "8080"));
    public static final String DATA_PATH = ORANGE_PATH + "/data";
    public static final String CONF_PATH = ORANGE_PATH + "/conf";
    public static final String INDEX_CONF = CONF_PATH + "/index";
    public static final String SUGGEST_CONF = ORANGE_PATH + "/suggest";
    public static final String IK_CONF = ORANGE_PATH + "/conf/ik";

    private LocalDateTime lastStatTime;

    @SneakyThrows
    public static IndexConf readFromFile() {
        Path path = Paths.get(INDEX_CONF);
        File file = path.toFile();
        if (!file.exists()) {
            file.getParentFile().mkdirs();
            file.createNewFile();
        }
        String index = Files.readString(path);
        if (StringUtil.isNullOrEmpty(index)) {
            return new IndexConf().setLastStatTime(LocalDateTime.MIN);
        }
        return JsonUtil.fromJson(index, IndexConf.class);
    }

    @SneakyThrows
    public void save2file() {

        Files.writeString(Paths.get(INDEX_CONF), JsonUtil.toJson(this));
    }
}
