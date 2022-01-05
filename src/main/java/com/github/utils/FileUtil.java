package com.github.utils;

import java.nio.file.Paths;

public class FileUtil {


    public static String name2ext(String name) {
        String[] split = name.split("\\.");
        if (split.length == 1) {
            return "";
        }
        return split[split.length - 1];
    }

    public static String absPath2name(String absPath) {
        String[] split = absPath.split("/");
        return split[split.length - 1];
    }

    public static String absPath2absPathIndexed(String absPath) {
        return String.join(" ", absPath.split("/"));
    }

    public static String formatPath(String logPath) {
        String s = Paths.get(logPath).normalize().toAbsolutePath().toString();
        String sp = "\\\\";
        if (s.contains(sp)) {
            return s.replace(sp, "/");
        }
        String sp0 = "\\";
        if (s.contains(sp0)) {
            return s.replace(sp0, "/");
        }
        return s;
    }
}
