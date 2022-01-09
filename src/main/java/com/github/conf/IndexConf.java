package com.github.conf;

import com.github.utils.JsonUtil;
import io.netty.util.internal.StringUtil;
import lombok.Data;
import lombok.experimental.Accessors;
import lombok.extern.java.Log;

import java.io.File;
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.Date;
import java.util.logging.Level;

@Log
@Data
@Accessors(chain = true)
public class IndexConf {
    public static final int PORT = Integer.parseInt(System.getProperty("port", "41320"));
    public static final String PROJECT_PATH = System.getProperty("project.path", "..");
    public static final String LIB_PATH = "/lib";
    public static final String FSEVENT_EXE = "fsevent.exe";
    public static final String FSEVENT_PATH = PROJECT_PATH + LIB_PATH + "/" + FSEVENT_EXE;
    public static final String INDEX_PATH = PROJECT_PATH + "/.orange/index";
    public static final String DATA_PATH = PROJECT_PATH + "/.orange/data";
    public static final String CONF_PATH = PROJECT_PATH + "/.orange/conf";
    public static final String INDEX_CONF = CONF_PATH + "/app";
    public static final String SUGGEST_CONF = PROJECT_PATH + "/.orange/suggest";
    public static final String IK_CONF = PROJECT_PATH + "/.orange/conf/ik";

    private Date lastStatTime;
    private static IndexConf indexConf;

    private IndexConf() {
    }

    public synchronized static IndexConf getInstance() {
        if (indexConf != null) {
            return indexConf;
        }
        String index = null;
        try {
            Path path = Paths.get(INDEX_CONF);
            File file = path.toFile();
            if (!file.exists()) {
                file.getParentFile().mkdirs();
                file.createNewFile();
            }
            index = Files.readString(path);
            if (StringUtil.isNullOrEmpty(index)) {
                indexConf = new IndexConf().setLastStatTime(new Date());
                indexConf.save2file();
                return indexConf;
            } else {
                indexConf= JsonUtil.fromJson(index, IndexConf.class);
                return indexConf;
            }
        } catch (IOException e) {
            log.log(Level.SEVERE, "read from file err", e);
            indexConf = new IndexConf().setLastStatTime(new Date());
            indexConf.save2file();
            return indexConf;
        }
    }

    public void save2file() {

        try {
            String csq = JsonUtil.toJson(this);
            Files.writeString(Paths.get(INDEX_CONF), csq);
        } catch (IOException e) {
            log.log(Level.SEVERE, "save to file err", e);
        }
    }
}
