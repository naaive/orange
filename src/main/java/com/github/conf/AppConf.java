package com.github.conf;

import com.github.utils.JsonUtil;
import io.netty.util.internal.StringUtil;
import lombok.Data;
import lombok.experimental.Accessors;
import lombok.extern.java.Log;

import java.io.File;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.Date;
import java.util.HashMap;
import java.util.Map;
import java.util.logging.Level;

@Log
@Data
@Accessors(chain = true)
public class AppConf {
    public static final int PORT = Integer.parseInt(System.getProperty("port", "41320"));
    public static final String PROJECT_PATH = new File(System.getProperty("project.path", "")).getAbsolutePath();

    public static final String LIB_PATH = "/lib";
    public static final String FSEVENT_EXE = "fsevent.exe";
    public static final String FSEVENT_PATH = PROJECT_PATH + LIB_PATH + "/" + FSEVENT_EXE;
    public static final String CACHEDATA = "cachedata";
    public static final String INDEX_PATH = PROJECT_PATH + "/" + CACHEDATA + "/index";
    public static final String DATA_PATH = PROJECT_PATH + "/" + CACHEDATA + "/data";
    public static final String SUGGEST_CONF = PROJECT_PATH + "/" + CACHEDATA + "/suggest";
    public static final String STAT_PATH = PROJECT_PATH + "/conf/stat";
    public static final String INDEX_CONF = STAT_PATH + "/volume";
    public static final String IK_CONF = PROJECT_PATH + "/conf/ik";

    private Date lastStatTime;
    private static AppConf appConf;
    private static Map<String, AppConf> from2indexConf = new HashMap<>();

    private AppConf() {}

    @SuppressWarnings("ResultOfMethodCallIgnored")
    public static synchronized AppConf getInstance(String from) {
        AppConf conf = from2indexConf.get(from);
        if (conf != null) {
            return conf;
        }
        String index = null;
        try {
            Path path = Paths.get(INDEX_CONF + from);
            File file = path.toFile();
            if (!file.exists()) {
                file.getParentFile().mkdirs();
                file.createNewFile();
            }
            index = Files.readString(path);
            if (StringUtil.isNullOrEmpty(index)) {
                appConf = new AppConf().setLastStatTime(new Date(0));
                appConf.save2file(from);
                from2indexConf.put(from, appConf);
                return appConf;
            } else {
                appConf = JsonUtil.fromJson(index, AppConf.class);
                from2indexConf.put(from, appConf);
                return AppConf.appConf;
            }
        } catch (Exception e) {
            log.log(Level.SEVERE, "read from file err", e);
            appConf = new AppConf().setLastStatTime(new Date());
            from2indexConf.put(from, appConf);
            appConf.save2file(from);
            return appConf;
        }
    }

    public void save2file(String from) {
        try {
            String csq = JsonUtil.toJson(this);
            Files.writeString(Paths.get(INDEX_CONF + from), csq);
        } catch (Exception e) {
            log.log(Level.SEVERE, "save to file err", e);
        }
    }
}
