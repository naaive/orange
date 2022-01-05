package com.github.utils;

import java.nio.file.Paths;
import java.util.Arrays;
import java.util.Comparator;

public class FileUtil {
    public static void main(String[] args) {
        String s = "C:\\\\Users\\\\Administrator\\\\IdeaProjects\\\\orange\\\\src\\\\main\\\\resources\\\\.orange\\\\data\\\\000023.log";
        String s1 = absPath2name(s);
        System.out.println(s1);
    }
    public static String name2ext(String name) {
        String[] split = name.split("\\.");
        if (split.length==1) {
            return "";
        }
        return split[split.length - 1];
    }

    public static String absPath2name(String absPath) {
        String[] split = absPath.split("\\\\");
        return split[split.length - 1];
    }

    public static String formatPath(String logPath) {
        return Paths.get(logPath).normalize().toAbsolutePath().toString();
    }
}
