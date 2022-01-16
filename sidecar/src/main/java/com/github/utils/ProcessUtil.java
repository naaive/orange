package com.github.utils;

import com.github.conf.AppConf;
import com.google.common.hash.Hashing;
import lombok.extern.java.Log;

import java.io.BufferedReader;
import java.io.InputStreamReader;
import java.nio.charset.StandardCharsets;
import java.util.Date;
import java.util.List;
import java.util.logging.Level;
import java.util.stream.Collectors;

import static com.github.conf.AppConf.getInstance;

@Log
public class ProcessUtil {

    private static final String ORANGE_SIDECAR = "orange_sidecar";
    private static final String ORANGE_UI = "Orange";

    public static boolean shouldStat(String from) {

        AppConf appConf = getInstance(Hashing.sha256()
                .hashBytes(from.getBytes(StandardCharsets.UTF_8))
                .toString());
        Date lastStatTime = appConf.getLastStatTime();
        Date now = new Date();
        appConf.setLastStatTime(now);
        appConf.save2file(Hashing.sha256()
                .hashBytes(from.getBytes(StandardCharsets.UTF_8))
                .toString());

        return now.getTime() - lastStatTime.getTime() < 1000 * 60 * 60 * 12;
    }

    public static void winKillByPort(int port) {
        try {
            Runtime rt = Runtime.getRuntime();
            Process proc = rt.exec("cmd /c netstat -ano | findstr " + port);

            BufferedReader stdInput = new BufferedReader(new InputStreamReader(proc.getInputStream()));
            String s;
            if ((s = stdInput.readLine()) != null) {
                int index = s.lastIndexOf(" ");
                String sc = s.substring(index);
                rt.exec("cmd /c Taskkill /PID" + sc + " /T /F");
                log.info("killed pid:" + sc);
            }
        } catch (Exception e) {
            log.log(Level.SEVERE, "kill err", e);
        }
    }

    public static boolean isAlive() {
        ProcessHandle current = ProcessHandle.current();
        List<ProcessHandle> handles = ProcessHandle.allProcesses().collect(Collectors.toList());
        for (ProcessHandle handle : handles) {
            ProcessHandle.Info info = handle.info();
            if (info.command().isPresent()) {
                String name = FileUtil.absPath2name(info.command().get());
                if (name.contains(ORANGE_UI) && !current.equals(handle)) {
                    return true;
                }
            }
            if (info.commandLine().isPresent()) {
                String name = FileUtil.absPath2name(info.commandLine().get());
                if (name.contains(ORANGE_UI) && !current.equals(handle)) {
                    return true;
                }
            }
        }
        return false;
    }

    public static void cleanOrangeCore() {
        ProcessHandle current = ProcessHandle.current();
        List<ProcessHandle> handles = ProcessHandle.allProcesses().collect(Collectors.toList());
        for (ProcessHandle handle : handles) {
            ProcessHandle.Info info = null;
            try {
                info = handle.info();
            } catch (Exception e) {
                continue;
            }
            if (info.command().isPresent()) {
                String name = FileUtil.absPath2name(info.command().get());
                if (name.contains(ORANGE_SIDECAR) && !current.equals(handle)) {
                    log.info("close orange_sidecar:" + name);
                    handle.destroyForcibly();
                }
            }
            if (info.commandLine().isPresent()) {
                String name = FileUtil.absPath2name(info.command().get());
                if (name.contains(ORANGE_SIDECAR) && !current.equals(handle)) {
                    log.info("close orange_sidecar:" + name);
                    handle.destroyForcibly();
                }
            }
        }
    }

    public static void clean() {
        cleanFsevent();
        cleanOrangeCore();
    }

    public static void cleanFsevent() {

        ProcessHandle.allProcesses().forEach(processHandle -> {
            ProcessHandle.Info info = processHandle.info();
            if (info.command().isPresent()) {
                String name = FileUtil.absPath2name(info.command().get());
                if (name.contains(AppConf.FSEVENT_EXE)) {
                    log.info("close fsevent:" + name);
                    processHandle.destroyForcibly();
                }
            }
            if (info.commandLine().isPresent()) {
                String name = FileUtil.absPath2name(info.command().get());
                if (name.contains(AppConf.FSEVENT_EXE)) {
                    log.info("close fsevent:" + name);
                    processHandle.destroyForcibly();
                }
            }
        });
    }
}
