package com.github.conf;


import lombok.extern.java.Log;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.nio.file.attribute.BasicFileAttributes;
import java.util.logging.Logger;

@Log
public class LogConf {

    private static final int _200_M = 200 * 1024 * 1024;
//    private static final String LOG_FILE = "logs/orange.log";

//    private static final Logger LOGGER;
//    static {
//        try {
//            LogManager.getLogManager().readConfiguration(LogConf.class.getResourceAsStream("/logging.properties"));
//        } catch (IOException e) {
//            e.printStackTrace();
//        }
//        LOGGER = Logger.getLogger(LogConf.class.getName());
//    }

    private static long getSize(String file) {
        BasicFileAttributes basicFileAttributes = null;
        try {
            basicFileAttributes = Files.readAttributes(Paths.get(file), BasicFileAttributes.class);
        } catch (IOException e) {
            e.printStackTrace();
            return 0;
        }
        return basicFileAttributes.size();
    }
    private static  Logger LOGGER;

    public static void initialize() {
//            try {
//                LogManager.getLogManager().readConfiguration(OrangeServer.class.getResourceAsStream("/logging.properties"));
//            } catch (IOException e) {
//                e.printStackTrace();
//            }
//            LOGGER = Logger.getLogger(OrangeServer.class.getName());




    }
}
