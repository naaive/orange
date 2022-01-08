package com.github.utils;

import com.github.conf.IndexConf;

import java.time.Duration;
import java.time.LocalDateTime;
import java.util.List;
import java.util.stream.Collectors;

import static com.github.conf.IndexConf.readFromFile;

public class ProcessUtil {

    private static final String ORANGE_CORE = "orange_core";

    public static boolean shouldStat() {

        IndexConf indexConf = readFromFile();
        LocalDateTime lastStatTime = indexConf.getLastStatTime();
        LocalDateTime now = LocalDateTime.now();
        indexConf.setLastStatTime(now);
        indexConf.save2file();
        return Duration.between(lastStatTime, now).toHours() < 1;
    }

    public static boolean isAlive() {
        List<ProcessHandle> handles = ProcessHandle.allProcesses().collect(Collectors.toList());
        for (ProcessHandle handle : handles) {
            ProcessHandle.Info info = handle.info();
            if (info.command().isPresent()) {
                String s = info.command().get();
                if (s.contains(ORANGE_CORE)) {
                    return true;
                }
            }
            if (info.commandLine().isPresent()) {
                String s = info.commandLine().get();
                if (s.contains(ORANGE_CORE)) {
                    return true;
                }
            }
        }
        return false;
    }

    public static void clean() {

        ProcessHandle.allProcesses().forEach(processHandle -> {
            ProcessHandle.Info info = processHandle.info();
            if (info.command().isPresent()) {
                String s = info.command().get();
                if (s.contains(IndexConf.EXE)) {
                    processHandle.destroyForcibly();
                }
            }
            if (info.commandLine().isPresent()) {
                String s = info.commandLine().get();
                if (s.contains(IndexConf.EXE)) {
                    processHandle.destroyForcibly();
                }
            }
        });
    }
}
