package com.github.conf;

import com.alibaba.fastjson.JSON;
import io.netty.util.internal.StringUtil;
import lombok.Data;
import lombok.SneakyThrows;
import lombok.experimental.Accessors;

import java.io.File;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;

@Data
@Accessors(chain = true)
public class IndexConf {
    public static final String ORANGE_PATH = "C:/Users/Administrator/IdeaProjects/orange/src/main/resources/.orange";
    public static final String INDEX_PATH = ORANGE_PATH + "/index";
    public static final int PORT = Integer.parseInt(System.getProperty("port", "8080"));
    public static final String DATA_PATH = ORANGE_PATH + "/data";
    public static final String CONF_PATH = ORANGE_PATH + "/conf";
    public static final String INDEX_CONF = CONF_PATH + "/index";
    public static final String SUGGEST_CONF = CONF_PATH + "/suggest";

    private Long lastStatTime;

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
            return new IndexConf().setLastStatTime(0L);
        }
        return JSON.parseObject(index, IndexConf.class);
    }

    @SneakyThrows
    public void save2file() {
        String s = JSON.toJSONString(this);
        Files.writeString(Paths.get(INDEX_CONF), s);
    }
}
